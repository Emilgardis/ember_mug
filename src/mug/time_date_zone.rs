use super::*;
impl EmberMug {
    /// Get the current date and timezone on the mug
    pub async fn get_time_date_zone(&self) -> Result<TimeDateZone, ReadError> {
        self.read_deserialize(&crate::KnownCharacteristic::TimeDateZone)
            .await
    }
    /// A sink for the mug to store the current date and timezone
    pub async fn set_time_date_zone(
        &self,
        time_date_zone: &TimeDateZone,
    ) -> Result<(), WriteError> {
        self.command(&crate::KnownCharacteristic::TimeDateZone, time_date_zone)
            .await
    }
}

/// Time and date + timezone
#[derive(Debug, BinWrite, BinRead)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[bw(little)]
#[br(little)]
pub struct TimeDateZone {
    /// Unix timestamp recorded by the app.
    pub unix_timestamp: u32,
    /// Timezone offset (ex: GMT+03)
    pub offset: u8,
}
