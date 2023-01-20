//! Functions for communicating with BLE to connect to Ember Mugs
use futures::{StreamExt, TryStreamExt};

use btleplug::{
    api::{BDAddr, Central, Manager, Peripheral, ScanFilter},
    platform,
};

use crate::SearchError;

/// Search given adapter for a mug
pub async fn search_adapter_for_ember(
    adapter: &platform::Adapter,
    mac: Option<BDAddr>,
) -> Result<
    impl futures::Stream<Item = Result<crate::mug::Peripheral, btleplug::Error>> + 'static,
    btleplug::Error,
> {
    use futures::FutureExt;
    let adapter_info = adapter.adapter_info().boxed().await?;
    tracing::debug!(
        adapter.adapter_info = ?adapter_info,
        "discovering mugs on adapter"
    );
    let adapter = adapter.clone();

    adapter
        .start_scan(ScanFilter {
            services: crate::EMBER_MUG_PUBLIC_SERVICES.to_vec(),
        })
        .await?;
    let stream = adapter.events().await?;
    Ok(stream
        .filter_map(move |f| {
            let adapter = adapter.clone();
            async move {
                match f {
                    btleplug::api::CentralEvent::DeviceDiscovered(id) => {
                        tracing::trace!(?id, "discovered");
                        Some(adapter.peripheral(&id).await)
                    }
                    // TODO: can this stall if the service is not found but the device was discovered?
                    _ => None,
                }
            }
        })
        .try_filter_map(move |peripheral| async move {
            let peripheral = if let Some(props) = peripheral.properties().await? {
                match mac {
                    Some(mac) => props.address == mac,
                    None => {
                        props
                            .local_name
                            .map(|name| name.contains("Ember"))
                            .unwrap_or_default()
                            || props
                                .manufacturer_data
                                .keys()
                                .any(|&m| m == crate::EMBER_ASSIGNED_NUMBER)
                    }
                }
                .then_some(peripheral)
            } else {
                None
            };
            Ok(peripheral)
        })
        .boxed())
}

/// Get mugs on all adapters
pub async fn get_mugs() -> Result<
    impl futures::Stream<Item = Result<(platform::Adapter, crate::mug::Peripheral), crate::SearchError>>,
    SearchError,
> {
    let manager = platform::Manager::new().await?;
    let adapters = manager.adapters().await?;
    Ok(get_mugs_on_adapters(&adapters)
        .await
        .map_ok(move |(i, p)| (adapters[i].clone(), p)))
}

/// Search for mugs on all adapters
pub async fn get_mugs_on_adapters(
    adapters: &[platform::Adapter],
) -> impl futures::Stream<Item = Result<(usize, crate::mug::Peripheral), crate::SearchError>> + 'static
{
    let mut set = tokio::task::JoinSet::new();
    for (i, adapter) in adapters.iter().enumerate() {
        let adapter = adapter.clone();
        set.spawn(async move {
            search_adapter_for_ember(&adapter, None)
                .await
                .map(|res| res.map_ok(move |p| (i, p)))
        });
    }
    tracing::debug!("spawned search tasks");
    futures::stream::try_unfold(set, |mut set| async move {
        if let Some(res) = set.join_next().await {
            Ok(Some((res??, set)))
        } else {
            Ok(None)
        }
    })
    // FIXME: Replace with TryFlattenUnordered when available
    .flat_map_unordered(0, |f: Result<_, crate::SearchError>| match f {
        Ok(s) => s.map_err(crate::SearchError::BtleError).boxed(),
        Err(e) => futures::stream::once(async { Err(e) }).boxed(),
    })
}
