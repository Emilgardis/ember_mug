use super::*;

impl EmberMug {
    /// Retrieves the battery percentage of the mug and other values.
    pub async fn get_battery(&self) -> Result<Battery, ReadError> {
        Battery::read(&mut Cursor::new(self.read(&BATTERY).await?)).map_err(Into::into)
    }
}

/// Battery information
#[derive(BinRead, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[br(little)]
pub struct Battery {
    /// Battery percentage (5 - 100. Not scaled to 0 - 255)
    pub battery: u8,
    /// Charging status. 1 for plugged in, 0 for unplugged
    #[br(map = |x: u8| x != 0)]
    pub charge: bool,
    /// Battery temperature as UINT16 Little Endian, encoded like the other temperatures
    pub temperature: Temperature,
    /// (Legacy) Most likely battery voltage
    pub volt: u8,
}
