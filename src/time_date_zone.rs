use super::*;
impl EmberMug {
    /// Get the current date and timezone on the mug
    pub async fn get_time_date_zone(&self) -> Result<TimeDateZone, ReadError> {
        TimeDateZone::read(&mut Cursor::new(self.read(&TIME_DATE_ZONE).await?)).map_err(Into::into)
    }
    /// A sink for the mug to store the current date and timezone
    pub async fn set_time_date_zone(
        &self,
        time_date_zone: &TimeDateZone,
    ) -> Result<(), WriteError> {
        self.command(&TIME_DATE_ZONE, time_date_zone).await
    }
}

#[derive(Debug, BinWrite, BinRead)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[bw(little)]
#[br(little)]
pub struct TimeDateZone {
    /// Unix timestamp recorded by the app.
    unix_timestamp: u32,
    /// Timezone offset (ex: GMT+03)
    offset: u8,
}
