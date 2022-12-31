use hashbrown::HashMap;
use tokio::sync::oneshot;

type ResolverItem = Box<dyn std::any::Any + Send + Sync>;
type PendingItem = Box<dyn FnMut() -> Option<ResolverItem>>;
type PendingStream = Box<dyn FnMut() -> Result<ResolverItem, flume::TryRecvError>>;

#[derive(Default)]
pub struct Resolver<K> {
    pending: Vec<(K, PendingItem)>,
    resolved: HashMap<K, ResolverItem>,
    streams: HashMap<K, PendingStream>,
}

impl<K: std::hash::Hash + Eq + std::fmt::Debug + Clone> Resolver<K> {
    #[track_caller]
    pub fn add<T>(&mut self, key: K, mut item: oneshot::Receiver<T>)
    where
        T: Send + Sync + 'static + std::any::Any,
        K: Clone + 'static,
    {
        tracing::debug!(?key, "adding task: {}", std::panic::Location::caller());
        self.pending.push((
            key,
            Box::new(move || {
                let out = item.try_recv().ok()?;
                Some(Box::new(out))
            }),
        ));
    }

    #[track_caller]
    pub fn add_with<T, F>(&mut self, key: K, fut: F)
    where
        T: Send + Sync + 'static + std::any::Any,
        K: Clone + 'static,
        F: std::future::Future<Output = T> + Send + 'static,
    {
        let recv = super::spawn(fut);
        self.add(key, recv);
    }

    #[track_caller]
    pub fn add_stream<T>(&mut self, key: K, item: flume::Receiver<T>)
    where
        T: Send + Sync + 'static + std::any::Any,
    {
        tracing::debug!(?key, "adding stream");
        self.streams.insert(
            key,
            Box::new(move || {
                let out = item.try_recv()?;
                Ok(Box::new(out))
            }),
        );
    }

    #[track_caller]
    pub fn add_stream_with<T, F, E>(
        &mut self,
        key: K,
        mut stream: impl FnMut(flume::Sender<T>) -> F,
    ) where
        T: Send + Sync + 'static + std::any::Any,
        F: std::future::Future<Output = Result<(), E>> + Send + 'static,
        K: Clone + 'static,
        E: Send + Sync + 'static,
    {
        tracing::debug!("adding stream");
        let (sender, receiver) = flume::unbounded();
        let recv = super::spawn(stream(sender));
        self.add_stream(key.clone(), receiver);
        self.add(key, recv);
    }

    pub fn poll(&mut self) {
        tracing::trace!(pending = ?self.pending.len(), streams = ?self.streams.len());
        self.pending.retain_mut(|(key, item)| {
            let Some(item) = item() else { return true };
            self.resolved.insert(key.clone(), item);
            false
        });
    }

    pub fn exists(&mut self, id: &K) -> bool {
        self.pending.iter().any(|(k, _)| k == id)
            || self.resolved.get(id).is_some()
            || self.streams.contains_key(id)
    }

    #[track_caller]
    pub fn try_take<T>(&mut self, id: K) -> Option<T>
    where
        T: Send + Sync + 'static + std::any::Any,
    {
        let Ok(item) = self.resolved.remove(&id)?.downcast::<T>() else {
            panic!("expected: {}", std::any::type_name::<T>())
        };

        if self.resolved.capacity() as f32 / 1.5 >= self.resolved.len() as f32 {
            self.resolved.shrink_to_fit()
        }

        Some(*item)
    }

    #[track_caller]
    pub fn try_take_with<T, F>(&mut self, key: K, fut: F) -> Option<T>
    where
        T: Send + Sync + 'static + std::any::Any,
        K: Clone + 'static,
        F: std::future::Future<Output = T> + Send + 'static,
    {
        if !self.exists(&key) {
            self.add_with(key.clone(), fut);
        }

        self.try_take(key)
    }

    #[track_caller]
    fn try_stream<E, T>(&mut self, key: K) -> Result<Option<T>, StreamError<E>>
    where
        T: Send + Sync + 'static + std::any::Any,
        E: Send + Sync + 'static,
    {
        match self.try_take::<Result<(), E>>(key.clone()).transpose() {
            Ok(None) => (),
            Ok(Some(())) => {
                // Stream ended
                self.pending.retain(|(k, _)| k != &key);
                return Err(StreamError::Ended);
            }
            Err(err) => {
                // Stream errored
                self.pending.retain(|(k, _)| k != &key);
                return Err(err.into());
            }
        };

        let Some(item) = self.streams.get_mut(&key) else {
            return Ok(None)
        };

        let item = match item() {
            Ok(item) => item,
            Err(flume::TryRecvError::Empty) => return Ok(None),
            Err(e) => {
                self.streams.remove(&key);
                if self.streams.capacity() as f32 / 1.5 >= self.streams.len() as f32 {
                    self.streams.shrink_to_fit()
                }
                self.pending.retain(|(k, _)| k != &key);
                return Err(StreamError::TryRecvError(e));
            }
        };

        let Ok(item) = item.downcast::<T>() else {
            panic!("expected: {}", std::any::type_name::<T>())
        };

        Ok(Some(*item))
    }

    #[track_caller]
    pub fn try_stream_with<T, F, E>(
        &mut self,
        key: K,
        stream: impl FnMut(flume::Sender<T>) -> F,
    ) -> Result<Option<T>, StreamError<E>>
    where
        T: Send + Sync + 'static + std::any::Any,
        F: std::future::Future<Output = Result<(), E>> + Send + 'static,
        K: Clone + 'static,
        E: Send + Sync + 'static,
    {
        if !self.exists(&key) {
            self.add_stream_with(key.clone(), stream);
        }

        self.try_stream(key)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StreamError<E> {
    #[error(transparent)]
    TryRecvError(flume::TryRecvError),
    #[error("fail")]
    Other(#[from] E),
    #[error("stream ended")]
    Ended,
}
