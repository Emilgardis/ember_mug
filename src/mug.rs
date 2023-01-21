//! Hosts [`EmberMug`] and related functions
use std::io::Cursor;

use binrw::{BinRead, BinWrite};
use btleplug::api::{Characteristic, Peripheral as _};

use crate::*;

mod battery;
mod current_temp;
mod dsk;
mod last_location;
mod liquid_level;
mod liquid_state;
mod mug_color;
mod mug_meta;
mod name;
mod ota;
mod push_events;
mod target_temp;
mod temperature_unit;
mod time_date_zone;

pub use battery::Battery;
pub use last_location::LastLocation;
pub use liquid_level::LiquidLevel;
pub use liquid_state::LiquidState;
pub use mug_color::Color;
pub use mug_meta::MugMeta;
pub use ota::Ota;
pub use push_events::PushEvent;
pub use temperature_unit::TemperatureUnit;
pub use time_date_zone::TimeDateZone;

pub(crate) type Peripheral = <btleplug::platform::Adapter as btleplug::api::Central>::Peripheral;

/// An Ember Mug device
///
/// Create an instance with [`EmberMug::find_and_connect`] or [`EmberMug::connect_mug`]
#[derive(Clone)]
pub struct EmberMug {
    /// The underlying [`Peripheral`] representing this device
    peripheral: std::sync::Arc<EmberMugInner>,
    /// The set of [`Characteristic`]s for this device
    characteristics: std::collections::BTreeSet<Characteristic>,
    /// the adapter the mug is connected to
    adapter: btleplug::platform::Adapter,
}

#[derive(Clone)]
struct EmberMugInner(Peripheral);

impl std::ops::Deref for EmberMugInner {
    type Target = Peripheral;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for EmberMugInner {
    fn drop(&mut self) {
        let peripheral = self.0.clone();
        futures::executor::block_on(async move {
            let _ = peripheral.disconnect().await;
        });
    }
}

impl EmberMug {
    /// Find and connect to the first available Ember Mug
    pub async fn find_and_connect() -> Result<Self, ConnectError> {
        use futures::TryStreamExt;
        // FIXME: pin on stack with `Pin::new_unchecked` or `pin-utils`
        let mut stream = Box::pin(crate::btle::get_mugs().await?);
        let Some((adapter, mug)) = stream.try_next().await? else {
            return Err(ConnectError::NoDevice)
        };
        Self::connect_mug(adapter, mug).await
    }

    /// Connect to specific Ember Mug
    pub async fn connect_mug(
        adapter: btleplug::platform::Adapter,
        peripheral: Peripheral,
    ) -> Result<Self, ConnectError> {
        tracing::debug!(peripheral.address = ?peripheral.address(), peripheral.id = ?peripheral.id(), "connecting to mug");
        peripheral.connect().await?;
        peripheral.discover_services().await?;
        Ok(Self {
            characteristics: peripheral.characteristics(),
            peripheral: std::sync::Arc::new(EmberMugInner(peripheral)),
            adapter,
        })
    }

    /// Returns true if the device is connected, the device might be considered disconnected if it doesn't respond in 1 second
    pub async fn is_connected(&self) -> Result<bool, btleplug::Error> {
        match tokio::time::timeout(
            std::time::Duration::from_secs(1),
            self.peripheral.is_connected(),
        )
        .await
        {
            Ok(r) => r,
            Err(_e) => Ok(false),
        }
    }

    /// Returns when the device is disconnected.
    pub async fn disconnected(&self) -> Result<(), btleplug::Error> {
        use btleplug::api::Central as _;
        use futures::StreamExt;
        let peripheral_id = std::sync::Arc::new(self.peripheral.id());
        let mut stream = Box::pin(self.adapter.events().await?.filter_map(move |e| {
            let peripheral_id = peripheral_id.clone();
            async move {
                match e {
                    btleplug::api::CentralEvent::DeviceDisconnected(id)
                        if &id == peripheral_id.as_ref() =>
                    {
                        Some(())
                    }
                    _ => None,
                }
            }
        }));
        stream.next().await;
        Ok(())
    }
}

impl EmberMug {
    /// Get characteristic on [`EMBER_MUG_SERVICE_UUID`] with given UUID
    pub fn get_characteristic(&self, uuid: &uuid::Uuid) -> Option<&Characteristic> {
        self.get_characteristic_on_service(uuid, &crate::EMBER_MUG_SERVICE_UUID)
    }

    /// Get all characteristics
    pub fn get_characteristics(&self) -> impl Iterator<Item = &Characteristic> {
        self.characteristics.iter()
    }

    /// Get characteristic on given service UUID with given UUID
    pub fn get_characteristic_on_service(
        &self,
        uuid: &uuid::Uuid,
        service_uuid: &uuid::Uuid,
    ) -> Option<&Characteristic> {
        self.characteristics
            .iter()
            .find(|&c| &c.uuid == uuid && &c.service_uuid == service_uuid)
    }
}

impl EmberMug {
    /// Read data from given characteristic with `uuid`
    pub async fn read_deserialize<T: BinRead + binrw::meta::ReadEndian>(
        &self,
        uuid: &uuid::Uuid,
    ) -> Result<T, ReadError>
    where
        T::Args: Default,
    {
        T::read(&mut Cursor::new(self.read(uuid).await?)).map_err(Into::into)
    }

    /// Deserialize data on given characteristic with `uuid`
    pub async fn read(&self, uuid: &uuid::Uuid) -> Result<Vec<u8>, ReadError> {
        self.peripheral
            .read(
                self.get_characteristic(uuid)
                    .ok_or(ReadError::NoSuchCharacteristic)?,
            )
            .await
            .map_err(Into::into)
    }

    /// Write data to given characteristic on `uuid`
    pub async fn write<D>(
        &self,
        write: btleplug::api::WriteType,
        uuid: &uuid::Uuid,
        data: &D,
    ) -> Result<(), WriteError>
    where
        D: BinWrite + binrw::meta::WriteEndian + Send + Sync,
        <D as BinWrite>::Args: Default,
    {
        let mut buf = Cursor::new(vec![]);
        data.write(&mut buf)?;
        self.peripheral
            .write(
                self.get_characteristic(uuid)
                    .ok_or(WriteError::NoSuchCharacteristic)?,
                buf.get_ref(),
                write,
            )
            .await
            .map_err(Into::into)
    }

    /// Send command to given characteristic on `uuid`
    pub async fn command<D>(&self, uuid: &uuid::Uuid, data: &D) -> Result<(), WriteError>
    where
        D: BinWrite + binrw::meta::WriteEndian + Send + Sync,
        <D as BinWrite>::Args: Default,
    {
        self.write(btleplug::api::WriteType::WithoutResponse, uuid, data)
            .await
    }

    /// Send request to given characteristic on `uuid`
    pub async fn request<D>(&self, uuid: &uuid::Uuid, data: &D) -> Result<(), WriteError>
    where
        D: BinWrite + binrw::meta::WriteEndian + Send + Sync,
        <D as BinWrite>::Args: Default,
    {
        self.write(btleplug::api::WriteType::WithResponse, uuid, data)
            .await
    }
}

#[derive(BinRead, BinWrite, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
#[br(little)]
#[bw(little)]
/// Temperature in a certain unit
pub struct Temperature {
    /// The temperature in integer value, use [`Temperature::to_degree`] for a value in degrees
    pub temperature: u16,
}

impl Temperature {
    /// Convert value to degree
    pub fn to_degree(&self) -> f32 {
        f32::from(self.temperature) * 0.01
    }

    /// Convert given degree to a temperature
    pub fn from_degree(deg: f32) -> Self {
        Self {
            temperature: (deg * 100.0) as u16,
        }
    }
}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.to_degree())
    }
}
