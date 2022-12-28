use std::time::Duration;

use btleplug::api::{Central as _, Manager as _, Peripheral as _, ScanFilter};
use ember_mug::EMBER_ASSIGNED_NUMBER;
use tokio::time;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), color_eyre::Report> {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_file(true)
        .pretty()
        .init();
    color_eyre::install()?;
    run().await
}

async fn run() -> Result<(), color_eyre::Report> {
    let manager = btleplug::platform::Manager::new().await?;
    let adapters = manager.adapters().await?;

    if adapters.is_empty() {
        tracing::error!("no adapters found");
    }

    for adapter in adapters {
        tracing::info!("Starting scan on {}...", adapter.adapter_info().await?);
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(10)).await;
        let peripherals = adapter.peripherals().await?;
        if peripherals.is_empty() {
            tracing::warn!("->>> BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range

            for peripheral in &peripherals {
                let properties = peripheral
                    .properties()
                    .await?
                    .ok_or_else(|| color_eyre::eyre::eyre!("oops"))?;
                if !properties
                    .manufacturer_data
                    .keys()
                    .any(|&i| i == EMBER_ASSIGNED_NUMBER)
                {
                    continue;
                }
                let is_connected = peripheral.is_connected().await?;
                let local_name = properties
                    .local_name
                    .as_deref()
                    .unwrap_or("(peripheral name unknown)");
                tracing::info!(
                    "Peripheral {:?} is connected: {:?}\n{:?}",
                    local_name,
                    is_connected,
                    properties
                );
                if !is_connected {
                    tracing::info!("Connecting to peripheral {:?}...", &local_name);
                    match time::timeout(Duration::from_secs(2), peripheral.connect()).await {
                        Ok(Ok(_)) => (),
                        Err(e) => {
                            tracing::error!(error = %e, "couldn't connect.");
                            continue;
                        }
                        Ok(Err(e)) => {
                            tracing::error!(error = %e, "Error connecting to peripheral, skipping");
                            continue;
                        }
                    }
                }
                let is_connected = peripheral.is_connected().await?;

                tracing::info!(
                    "Now connected ({:?}) to peripheral {:?}...",
                    is_connected,
                    &local_name
                );
                peripheral.discover_services().await?;
                tracing::info!("Discover peripheral {:?} services...", &local_name);
                for service in peripheral.services() {
                    tracing::info!(
                        "Service UUID {}, primary: {}",
                        service.uuid,
                        service.primary
                    );
                    for characteristic in service.characteristics {
                        tracing::info!("  {:?}", characteristic);
                    }
                }
                if is_connected {
                    tracing::info!("Disconnecting from peripheral {:?}...", &local_name);
                    peripheral
                        .disconnect()
                        .await
                        .expect("Error disconnecting from BLE peripheral");
                }
            }
        }
    }
    Ok(())
}
