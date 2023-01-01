#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![warn(clippy::dbg_macro)]
pub mod btle;

pub mod mug;

pub use mug::EmberMug;

/// Assigned Bluetooth company identifier for `Ember Technologies, Inc.`
pub static EMBER_ASSIGNED_NUMBER: u16 = 0x03C1;
/// The UUID for the Ember Mug service
pub const EMBER_MUG_SERVICE_UUID: uuid::Uuid = uuid::uuid!("fc543622-236c-4c94-8fa9-944a3e5353fa");

#[rustfmt::skip]
#[doc(hidden)]
pub mod characteristics {
    use uuid::uuid;

    /// The UUID for the Name characteristic
    pub const NAME: uuid::Uuid             = uuid!("fc540001-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Current Temperature characteristic
    pub const CURRENT_TEMP: uuid::Uuid     = uuid!("fc540002-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Target Temperature characteristic
    pub const TARGET_TEMP: uuid::Uuid      = uuid!("fc540003-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Temperature Unit characteristic
    pub const TEMPERATURE_UNIT: uuid::Uuid = uuid!("fc540004-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Liquid Level characteristic
    pub const LIQUID_LEVEL: uuid::Uuid     = uuid!("fc540005-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Time, Date, and Time Zone characteristic
    pub const TIME_DATE_ZONE: uuid::Uuid   = uuid!("fc540006-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Battery characteristic
    pub const BATTERY: uuid::Uuid          = uuid!("fc540007-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Liquid State characteristic
    pub const LIQUID_STATE: uuid::Uuid     = uuid!("fc540008-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Over-the-Air Update characteristic
    pub const OTA: uuid::Uuid              = uuid!("fc54000c-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Push Events characteristic
    pub const PUSH_EVENTS: uuid::Uuid      = uuid!("fc540012-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Mug Color characteristic
    pub const MUG_COLOR: uuid::Uuid        = uuid!("fc540014-236c-4c94-8fa9-944a3e5353fa");

    // extras
    /// The UUID for the Last Location characteristic
    pub const LAST_LOCATION: uuid::Uuid    = uuid!("fc54000a-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Mug Identifier characteristic
    pub const MUG_ID: uuid::Uuid           = uuid!("fc54000d-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Device Secret Key characteristic
    pub const DSK: uuid::Uuid              = uuid!("fc54000e-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the (U?) Device Secret Key characteristic
    pub const UDSK: uuid::Uuid             = uuid!("fc54000f-236c-4c94-8fa9-944a3e5353fa");

    // unknown
    /// The UUID for the Temperature Lock address characteristic
    pub const CONTROL_REGISTER_ADDRESS: uuid::Uuid = uuid!("fc540010-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Temperature Lock data characteristic
    pub const CONTROL_REGISTER_DATA: uuid::Uuid     = uuid!("fc540011-236c-4c94-8fa9-944a3e5353fa");
    /// The UUID for the Statistics characteristic
    pub const STATISTICS: uuid::Uuid     = uuid!("fc540013-236c-4c94-8fa9-944a3e5353fa");
}

/// All known characteristics of an Ember Mug
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KnownCharacteristic {
    /// The name of the mug.
    ///
    /// Capability: READ | WRITE
    Name,
    /// The current temperature of the mug's contents.
    ///
    /// Capability: READ
    CurrentTemp,
    /// The target temperature for the mug's contents.
    ///
    /// Capability: READ | WRITE
    TargetTemp,
    /// The unit of temperature measurement used for the mug.
    ///
    /// Capability: READ | WRITE
    TemperatureUnit,
    /// The current liquid level of the mug.
    ///
    /// Capability: READ
    LiquidLevel,
    /// The current time, date, and time zone of the mug.
    ///
    /// Capability: READ | WRITE
    TimeDateZone,
    /// The current battery level of the mug.
    ///
    /// Capability: READ
    Battery,
    /// The state of the liquid in the mug (e.g. solid, liquid, etc.).
    ///
    /// Capability: READ
    LiquidState,
    /// Firmware and hardware information (versions) for the mug.
    ///
    /// Capability: READ
    Ota,
    /// Events that are sent from the mug
    ///
    /// Capability: NOTIFY
    PushEvents,
    /// The color of the mug.
    ///
    /// Capability: READ | WRITE
    MugColor,
    /// The last known location of the mug.
    ///
    /// Capability: WRITE
    LastLocation,
    /// The mug's identifier.
    ///
    /// Capability: READ
    MugId,
    /// Device Secret Key
    ///
    /// Capability: READ
    Dsk,
    /// (U?) Device Secret Key
    ///
    /// Capability: READ | WRITE
    Udsk,
    /// Temperature lock address
    ///
    /// Capability: READ | WRITE
    ControlRegisterAddress,
    /// Temperature lock data
    ///
    /// Capability: READ | WRITE
    ControlRegisterData,
    /// Statistics
    ///
    /// Capability: NOTIFY
    Statistics,
}

impl KnownCharacteristic {
    /// Get the UUID for this characteristic
    pub const fn get(&self) -> &'static uuid::Uuid {
        use characteristics::*;
        match self {
            Self::Name => &NAME,
            Self::CurrentTemp => &CURRENT_TEMP,
            Self::TargetTemp => &TARGET_TEMP,
            Self::TemperatureUnit => &TEMPERATURE_UNIT,
            Self::LiquidLevel => &LIQUID_LEVEL,
            Self::TimeDateZone => &TIME_DATE_ZONE,
            Self::Battery => &BATTERY,
            Self::LiquidState => &LIQUID_STATE,
            Self::Ota => &OTA,
            Self::PushEvents => &PUSH_EVENTS,
            Self::MugColor => &MUG_COLOR,
            Self::LastLocation => &LAST_LOCATION,
            Self::MugId => &MUG_ID,
            Self::Dsk => &DSK,
            Self::Udsk => &UDSK,
            Self::ControlRegisterAddress => &CONTROL_REGISTER_ADDRESS,
            Self::ControlRegisterData => &CONTROL_REGISTER_DATA,
            Self::Statistics => &STATISTICS,
        }
    }

    /// Create a new known characteristic from UUID
    pub const fn new(uuid: &uuid::Uuid) -> Option<Self> {
        use characteristics::*;
        Some(match uuid {
            id if id.as_u128() == NAME.as_u128() => Self::Name,
            id if id.as_u128() == CURRENT_TEMP.as_u128() => Self::CurrentTemp,
            id if id.as_u128() == TARGET_TEMP.as_u128() => Self::TargetTemp,
            id if id.as_u128() == TEMPERATURE_UNIT.as_u128() => Self::TemperatureUnit,
            id if id.as_u128() == LIQUID_LEVEL.as_u128() => Self::LiquidLevel,
            id if id.as_u128() == TIME_DATE_ZONE.as_u128() => Self::TimeDateZone,
            id if id.as_u128() == BATTERY.as_u128() => Self::Battery,
            id if id.as_u128() == LIQUID_STATE.as_u128() => Self::LiquidState,
            id if id.as_u128() == OTA.as_u128() => Self::Ota,
            id if id.as_u128() == PUSH_EVENTS.as_u128() => Self::PushEvents,
            id if id.as_u128() == MUG_COLOR.as_u128() => Self::MugColor,
            id if id.as_u128() == LAST_LOCATION.as_u128() => Self::LastLocation,
            id if id.as_u128() == MUG_ID.as_u128() => Self::MugId,
            id if id.as_u128() == DSK.as_u128() => Self::Dsk,
            id if id.as_u128() == UDSK.as_u128() => Self::Udsk,
            id if id.as_u128() == CONTROL_REGISTER_ADDRESS.as_u128() => {
                Self::ControlRegisterAddress
            }
            id if id.as_u128() == CONTROL_REGISTER_DATA.as_u128() => Self::ControlRegisterData,
            id if id.as_u128() == STATISTICS.as_u128() => Self::Statistics,
            _ => return None,
        })
    }

    /// Get all known characteristics
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Name,
            Self::CurrentTemp,
            Self::TargetTemp,
            Self::TemperatureUnit,
            Self::LiquidLevel,
            Self::TimeDateZone,
            Self::Battery,
            Self::LiquidState,
            Self::Ota,
            Self::PushEvents,
            Self::MugColor,
            Self::LastLocation,
            Self::MugId,
            Self::Dsk,
            Self::Udsk,
            Self::ControlRegisterAddress,
            Self::ControlRegisterData,
            Self::Statistics,
        ]
    }
}

/// Errors when trying to connect to an Ember Mug
#[derive(Debug, thiserror::Error)]
pub enum ConnectError {
    /// No device found
    #[error("no device found")]
    NoDevice,
    /// Search failed
    #[error("search failed")]
    SearchError(#[from] SearchError),
    /// Couldn't connect device
    #[error("couldn't connect to device")]
    BtleConnectError(#[from] btleplug::Error),
}

/// Errors when searching for a device
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    /// Task join failed
    #[error("join failed")]
    JoinError(#[from] tokio::task::JoinError),
    /// Search operation failed
    #[error("btle operation failed")]
    BtleError(#[from] btleplug::Error),
}

/// Errors when reading data from an Ember Mug
#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    /// Characteristic is missing / not present on device
    #[error("characteristic is missing")]
    NoSuchCharacteristic,
    /// Read from BLE failed
    #[error("btle operation failed")]
    BtleError(#[from] btleplug::Error),
    /// Reading of data failed
    #[error("read failed")]
    ReadError(#[from] binrw::Error),
    /// Failed to convert string to valid UTF-8
    #[error("string parse failed")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

/// Errors when writing data to an Ember Mug
#[derive(Debug, thiserror::Error)]
pub enum WriteError {
    /// Characteristic is missing / not present on device
    #[error("characteristic is missing")]
    NoSuchCharacteristic,
    /// Write with BLE failed
    #[error("btle operation failed")]
    BtleError(#[from] btleplug::Error),
    /// Interpreting source data into bytes failed
    #[error("write failed")]
    WriteError(#[from] binrw::Error),
    /// Data to be written was invalid
    #[error("data is invalid: {0}")]
    InvalidFormat(&'static str),
}
