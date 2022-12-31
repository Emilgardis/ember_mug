//! Functions for communicating with BLE to connect to Ember Mugs
use std::time::Duration;

use btleplug::{
    api::{BDAddr, Central, Manager, Peripheral, ScanFilter},
    platform,
};

use crate::SearchError;

/// Search given adapter for a mug
pub async fn search_adapter_for_ember(
    adapter: &platform::Adapter,
    mac: Option<BDAddr>,
) -> Result<Vec<crate::Peripheral>, btleplug::Error> {
    use futures::FutureExt;
    let adapter_info = adapter.adapter_info().boxed().await?;
    tracing::debug!(
        adapter.adapter_info = ?adapter_info,
        "discovering mugs on adapter"
    );
    adapter.start_scan(ScanFilter::default()).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    let mut mugs = Vec::new();
    for peripheral in adapter.peripherals().await? {
        if let Some(props) = peripheral.properties().await? {
            if match mac {
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
            } {
                mugs.push(peripheral);
            }
        }
    }
    Ok(mugs)
}

/// Get mugs on all adapters
pub async fn get_mugs() -> Result<
    impl futures::Stream<Item = Result<crate::Peripheral, SearchError>> + 'static,
    SearchError,
> {
    let manager = platform::Manager::new().await?;
    let adapters = manager.adapters().await?;
    Ok(get_mugs_on_adapters(&adapters).await)
}

/// Search for mugs on all adapters
pub async fn get_mugs_on_adapters(
    adapters: &[platform::Adapter],
) -> impl futures::Stream<Item = Result<crate::Peripheral, SearchError>> + 'static {
    let mut set = tokio::task::JoinSet::new();
    for adapter in adapters {
        let adapter = adapter.clone();
        set.spawn(async move { search_adapter_for_ember(&adapter, None).await });
    }
    futures::stream::try_unfold((set, vec![]), |(mut set, mut rem)| async move {
        if !rem.is_empty() {
            return Ok(Some((rem.pop().unwrap(), (set, rem))));
        }
        if let Some(res) = set.join_next().await {
            rem = res??;
            let Some(p) = rem.pop() else {
                return Ok(None)
            };
            Ok(Some((p, (set, rem))))
        } else {
            Ok(None)
        }
    })
}
