#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![warn(clippy::dbg_macro)]
pub mod btle;

pub mod mug;

pub use mug::EmberMug;

pub use btleplug;

/// Assigned Bluetooth company identifier for `Ember Technologies, Inc.`
pub static EMBER_ASSIGNED_NUMBER: u16 = 0x03C1;
/// The UUID for the Ember Mug service
pub const EMBER_MUG_SERVICE_UUID: uuid::Uuid = uuid::uuid!("fc543622-236c-4c94-8fa9-944a3e5353fa");
/// Known public UUIDs of ember mugs
pub const EMBER_MUG_PUBLIC_SERVICES: &[uuid::Uuid] =
    &[uuid::uuid!("0000180a-0000-1000-8000-00805f9b34fb")];

/// All known characteristics of an Ember Mug
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KnownCharacteristic {
    /// The name of the mug.
    ///
    /// Capability: READ | WRITE
    /// `fc540001-236c-4c94-8fa9-944a3e5353fa`
    Name = 0x1,
    /// The current temperature of the mug's contents.
    ///
    /// Capability: READ
    /// `fc540002-236c-4c94-8fa9-944a3e5353fa`
    CurrentTemp = 0x2,
    /// The target temperature for the mug's contents.
    ///
    /// Capability: READ | WRITE
    /// `fc540003-236c-4c94-8fa9-944a3e5353fa`
    TargetTemp = 0x3,
    /// The unit of temperature measurement used for the mug.
    ///
    /// Capability: READ | WRITE
    /// `fc540004-236c-4c94-8fa9-944a3e5353fa`
    TemperatureUnit = 0x4,
    /// The current liquid level of the mug.
    ///
    /// Capability: READ
    /// `fc540005-236c-4c94-8fa9-944a3e5353fa`
    LiquidLevel = 0x5,
    /// The current time, date, and time zone of the mug.
    ///
    /// Capability: READ | WRITE
    /// `fc540006-236c-4c94-8fa9-944a3e5353fa`
    TimeDateZone = 0x6,
    /// The current battery level of the mug.
    ///
    /// Capability: READ
    /// `fc540007-236c-4c94-8fa9-944a3e5353fa`
    Battery = 0x7,
    /// The state of the liquid in the mug (e.g. solid, liquid, etc.).
    ///
    /// Capability: READ
    /// `fc540008-236c-4c94-8fa9-944a3e5353fa`
    LiquidState = 0x8,
    /// Firmware and hardware information (versions) for the mug.
    ///
    /// Capability: READ
    /// `fc54000c-236c-4c94-8fa9-944a3e5353fa`
    Ota = 0xc,
    /// Events that are sent from the mug
    ///
    /// Capability: NOTIFY
    /// `fc540012-236c-4c94-8fa9-944a3e5353fa`
    PushEvents = 0x12,
    /// The color of the mug.
    ///
    /// Capability: READ | WRITE
    /// `fc540014-236c-4c94-8fa9-944a3e5353fa`
    MugColor = 0x14,
    /// The last known location of the mug.
    ///
    /// Capability: WRITE
    /// `fc54000a-236c-4c94-8fa9-944a3e5353fa`
    LastLocation = 0xa,
    /// The mug's identifier.
    ///
    /// Capability: READ
    /// `fc54000d-236c-4c94-8fa9-944a3e5353fa`
    MugId = 0xd,
    /// Device Secret Key
    ///
    /// Capability: READ
    /// `fc54000e-236c-4c94-8fa9-944a3e5353fa`
    Dsk = 0xe,
    /// (U?) Device Secret Key
    ///
    /// Capability: READ | WRITE
    /// `fc54000f-236c-4c94-8fa9-944a3e5353fa`
    Udsk = 0xf,
    /// Temperature lock address
    ///
    /// Capability: READ | WRITE
    /// `fc540010-236c-4c94-8fa9-944a3e5353fa`
    ControlRegisterAddress = 0x10,
    /// Temperature lock data
    ///
    /// Capability: READ | WRITE
    /// `fc540011-236c-4c94-8fa9-944a3e5353fa`
    ControlRegisterData = 0x11,
    /// Statistics
    ///
    /// Capability: NOTIFY
    /// `fc540013-236c-4c94-8fa9-944a3e5353fa`
    Statistics = 0x13,
}

impl KnownCharacteristic {
    /// Get the UUID for this characteristic
    pub const fn get(&self) -> uuid::Uuid {
        uuid::Uuid::from_fields(
            0xfc540000 | *self as u8 as u32,
            0x236c,
            0x4c94,
            &[0x8f, 0xa9, 0x94, 0x4a, 0x3e, 0x53, 0x53, 0xfa],
        )
    }

    /// Create a new known characteristic from UUID
    pub fn new(uuid: &uuid::Uuid) -> Option<&'static Self> {
        for ch in Self::all() {
            let u1 = uuid.as_u64_pair();
            let u2 = ch.get().as_u64_pair();
            if u1 == u2 {
                return Some(ch);
            }
        }
        None
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
