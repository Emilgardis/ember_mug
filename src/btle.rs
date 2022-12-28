use std::time::Duration;

use btleplug::{
    api::{BDAddr, Central, Manager, Peripheral, ScanFilter},
    platform,
};

use crate::SearchError;

pub async fn search_adapter_for_ember(
    adapter: &platform::Adapter,
    mac: Option<BDAddr>,
) -> Result<Vec<crate::Peripheral>, btleplug::Error> {
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

pub async fn get_mugs() -> Result<Vec<crate::Peripheral>, SearchError> {
    let manager = platform::Manager::new().await?;
    let adapters = manager.adapters().await?;
    get_mugs_on_adapters(&adapters).await
}

pub async fn get_mugs_on_adapters(
    adapters: &[platform::Adapter],
) -> Result<Vec<crate::Peripheral>, SearchError> {
    let mut set = tokio::task::JoinSet::new();
    for adapter in adapters {
        let adapter = adapter.clone();
        set.spawn(async move { search_adapter_for_ember(&adapter, None).await });
    }
    let mut mugs = vec![];
    while let Some(p) = set.join_next().await.transpose()?.transpose()? {
        mugs.extend(p)
    }
    Ok(mugs)
}
