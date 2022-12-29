mod btle;

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
pub use liquid_level::LiquidLevel;
pub use liquid_state::LiquidState;
pub use mug_color::Color;
pub use mug_meta::MugMeta;
pub use ota::Ota;
pub use push_events::PushEvent;
pub use temperature_unit::TemperatureUnit;
pub use time_date_zone::TimeDateZone;

use binrw::{BinRead, BinWrite};
use btleplug::{
    api::{Characteristic, Peripheral as _},
    platform,
};

use std::io::Cursor;

/// Assigned Bluetooth company identifier for `Ember Technologies, Inc.`
pub static EMBER_ASSIGNED_NUMBER: u16 = 0x03C1;

#[rustfmt::skip]
mod characteristics {
    use uuid::uuid;

    pub const EMBER_MUG_SERVICE_UUID: uuid::Uuid = uuid!("fc543622-236c-4c94-8fa9-944a3e5353fa");

    pub static NAME: uuid::Uuid             = uuid!("fc540001-236c-4c94-8fa9-944a3e5353fa");
    pub static CURRENT_TEMP: uuid::Uuid     = uuid!("fc540002-236c-4c94-8fa9-944a3e5353fa");
    pub static TARGET_TEMP: uuid::Uuid      = uuid!("fc540003-236c-4c94-8fa9-944a3e5353fa");
    pub static TEMPERATURE_UNIT: uuid::Uuid = uuid!("fc540004-236c-4c94-8fa9-944a3e5353fa");
    pub static LIQUID_LEVEL: uuid::Uuid     = uuid!("fc540005-236c-4c94-8fa9-944a3e5353fa");
    pub static TIME_DATE_ZONE: uuid::Uuid   = uuid!("fc540006-236c-4c94-8fa9-944a3e5353fa");
    pub static BATTERY: uuid::Uuid          = uuid!("fc540007-236c-4c94-8fa9-944a3e5353fa");
    pub static LIQUID_STATE: uuid::Uuid     = uuid!("fc540008-236c-4c94-8fa9-944a3e5353fa");
    pub static OTA: uuid::Uuid              = uuid!("fc54000c-236c-4c94-8fa9-944a3e5353fa");
    pub static PUSH_EVENTS: uuid::Uuid      = uuid!("fc540012-236c-4c94-8fa9-944a3e5353fa");
    pub static MUG_COLOR: uuid::Uuid        = uuid!("fc540014-236c-4c94-8fa9-944a3e5353fa");

    // extras
    pub static LAST_LOCATION: uuid::Uuid    = uuid!("fc54000a-236c-4c94-8fa9-944a3e5353fa");
    pub static MUG_ID: uuid::Uuid           = uuid!("fc54000d-236c-4c94-8fa9-944a3e5353fa");
    pub static DSK: uuid::Uuid              = uuid!("fc54000e-236c-4c94-8fa9-944a3e5353fa");
    pub static UDSK: uuid::Uuid             = uuid!("fc54000f-236c-4c94-8fa9-944a3e5353fa");

    // unknown
    pub static CONTROL_REGISTER_ADDRESS: uuid::Uuid = uuid!("fc540010-236c-4c94-8fa9-944a3e5353fa");
    pub static CONTROL_REGISTER_DATA: uuid::Uuid     = uuid!("fc540011-236c-4c94-8fa9-944a3e5353fa");
    pub static STATISTICS: uuid::Uuid     = uuid!("fc540013-236c-4c94-8fa9-944a3e5353fa");
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KnownCharacteristic {
    Name,
    CurrentTemp,
    TargetTemp,
    TemperatureUnit,
    LiquidLevel,
    TimeDateZone,
    Battery,
    LiquidState,
    Ota,
    PushEvents,
    MugColor,
    LastLocation,
    MugId,
    Dsk,
    Udsk,
    ControlRegisterAddress,
    ControlRegisterData,
    Statistics,
}

impl KnownCharacteristic {
    pub fn get(&self) -> &uuid::Uuid {
        match self {
            KnownCharacteristic::Name => &NAME,
            KnownCharacteristic::CurrentTemp => &CURRENT_TEMP,
            KnownCharacteristic::TargetTemp => &TARGET_TEMP,
            KnownCharacteristic::TemperatureUnit => &TEMPERATURE_UNIT,
            KnownCharacteristic::LiquidLevel => &LIQUID_LEVEL,
            KnownCharacteristic::TimeDateZone => &TIME_DATE_ZONE,
            KnownCharacteristic::Battery => &BATTERY,
            KnownCharacteristic::LiquidState => &LIQUID_STATE,
            KnownCharacteristic::Ota => &OTA,
            KnownCharacteristic::PushEvents => &PUSH_EVENTS,
            KnownCharacteristic::MugColor => &MUG_COLOR,
            KnownCharacteristic::LastLocation => &LAST_LOCATION,
            KnownCharacteristic::MugId => &MUG_ID,
            KnownCharacteristic::Dsk => &DSK,
            KnownCharacteristic::Udsk => &UDSK,
            KnownCharacteristic::ControlRegisterAddress => &CONTROL_REGISTER_ADDRESS,
            KnownCharacteristic::ControlRegisterData => &CONTROL_REGISTER_DATA,
            KnownCharacteristic::Statistics => &STATISTICS,
        }
    }

    pub fn new(uuid: &uuid::Uuid) -> Option<Self> {
        Some(match uuid {
            id if id == &NAME => KnownCharacteristic::Name,
            id if id == &CURRENT_TEMP => KnownCharacteristic::CurrentTemp,
            id if id == &TARGET_TEMP => KnownCharacteristic::TargetTemp,
            id if id == &TEMPERATURE_UNIT => KnownCharacteristic::TemperatureUnit,
            id if id == &LIQUID_LEVEL => KnownCharacteristic::LiquidLevel,
            id if id == &TIME_DATE_ZONE => KnownCharacteristic::TimeDateZone,
            id if id == &BATTERY => KnownCharacteristic::Battery,
            id if id == &LIQUID_STATE => KnownCharacteristic::LiquidState,
            id if id == &OTA => KnownCharacteristic::Ota,
            id if id == &PUSH_EVENTS => KnownCharacteristic::PushEvents,
            id if id == &MUG_COLOR => KnownCharacteristic::MugColor,
            id if id == &LAST_LOCATION => KnownCharacteristic::LastLocation,
            id if id == &MUG_ID => KnownCharacteristic::MugId,
            id if id == &DSK => KnownCharacteristic::Dsk,
            id if id == &UDSK => KnownCharacteristic::Udsk,
            id if id == &CONTROL_REGISTER_ADDRESS => KnownCharacteristic::ControlRegisterAddress,
            id if id == &CONTROL_REGISTER_DATA => KnownCharacteristic::ControlRegisterData,
            id if id == &STATISTICS => KnownCharacteristic::Statistics,
            _ => return None,
        })
    }

    pub fn all() -> &'static [Self] {
        &[
            KnownCharacteristic::Name,
            KnownCharacteristic::CurrentTemp,
            KnownCharacteristic::TargetTemp,
            KnownCharacteristic::TemperatureUnit,
            KnownCharacteristic::LiquidLevel,
            KnownCharacteristic::TimeDateZone,
            KnownCharacteristic::Battery,
            KnownCharacteristic::LiquidState,
            KnownCharacteristic::Ota,
            KnownCharacteristic::PushEvents,
            KnownCharacteristic::MugColor,
            KnownCharacteristic::LastLocation,
            KnownCharacteristic::MugId,
            KnownCharacteristic::Dsk,
            KnownCharacteristic::Udsk,
            KnownCharacteristic::ControlRegisterAddress,
            KnownCharacteristic::ControlRegisterData,
            KnownCharacteristic::Statistics,
        ]
    }
}

use characteristics::*;

#[derive(Clone)]
pub struct EmberMug {
    peripheral: Peripheral,
    characteristics: std::collections::BTreeSet<Characteristic>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectError {
    #[error("no device found")]
    NoDevice,
    #[error("search failed")]
    SearchError(#[from] SearchError),
    #[error("couldn't connect to device")]
    BtleConnectError(#[from] btleplug::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("join failed")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("btle operation failed")]
    BtleError(#[from] btleplug::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("characteristic is missing")]
    NoSuchCharacteristic,
    #[error("btle operation failed")]
    BtleError(#[from] btleplug::Error),
    #[error("read failed")]
    ReadError(#[from] binrw::Error),
    #[error("string parse failed")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, thiserror::Error)]
pub enum WriteError {
    #[error("characteristic is missing")]
    NoSuchCharacteristic,
    #[error("btle operation failed")]
    BtleError(#[from] btleplug::Error),
    #[error("write failed")]
    WriteError(#[from] binrw::Error),
    #[error("data is invalid: {0}")]
    InvalidFormat(&'static str),
}

type Peripheral = <platform::Adapter as btleplug::api::Central>::Peripheral;

impl EmberMug {
    pub async fn find_and_connect() -> Result<EmberMug, ConnectError> {
        let Some(mug) = btle::get_mugs().await?.into_iter().next() else {
            return Err(ConnectError::NoDevice)
        };
        EmberMug::connect_mug(mug).await
    }
    pub async fn connect_mug(peripheral: Peripheral) -> Result<EmberMug, ConnectError> {
        tracing::debug!(peripheral.address = ?peripheral.address(), peripheral.id = ?peripheral.id(), "connecting to mug");
        peripheral.connect().await?;
        peripheral.discover_services().await?;
        Ok(EmberMug {
            characteristics: peripheral.characteristics(),
            peripheral,
        })
    }
}

impl EmberMug {
    pub fn get_characteristic(&self, uuid: &uuid::Uuid) -> Option<&Characteristic> {
        self.get_characteristic_on_service(uuid, &EMBER_MUG_SERVICE_UUID)
    }

    pub fn get_characteristics(&self) -> impl Iterator<Item = &Characteristic> {
        self.characteristics.iter()
    }

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
    pub async fn read(&self, uuid: &uuid::Uuid) -> Result<Vec<u8>, ReadError> {
        self.peripheral
            .read(
                self.get_characteristic(uuid)
                    .ok_or(ReadError::NoSuchCharacteristic)?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn write<D>(
        &self,
        write: btleplug::api::WriteType,
        uuid: &uuid::Uuid,
        data: &D,
    ) -> Result<(), WriteError>
    where
        D: BinWrite + binrw::meta::WriteEndian,
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

    pub async fn command<D>(&self, uuid: &uuid::Uuid, data: &D) -> Result<(), WriteError>
    where
        D: BinWrite + binrw::meta::WriteEndian,
        <D as BinWrite>::Args: Default,
    {
        self.write(btleplug::api::WriteType::WithoutResponse, uuid, data)
            .await
    }

    pub async fn request<D>(&self, uuid: &uuid::Uuid, data: &D) -> Result<(), WriteError>
    where
        D: BinWrite + binrw::meta::WriteEndian,
        <D as BinWrite>::Args: Default,
    {
        self.write(btleplug::api::WriteType::WithResponse, uuid, data)
            .await
    }
}

#[derive(BinRead, BinWrite, Debug)]
#[br(little)]
#[bw(little)]
pub struct Temperature {
    /// Battery temperature as UINT16 Little Endian, encoded like the other temperatures
    temperature: u16,
}

impl Temperature {
    pub fn to_degree(&self) -> f32 {
        f32::from(self.temperature) * 0.01
    }

    pub fn from_degree(deg: f32) -> Self {
        Self {
            temperature: (deg * 100.0) as u16,
        }
    }
}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>1} °", self.to_degree())
    }
}
