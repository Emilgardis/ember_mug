use super::*;
impl EmberMug {
    /// A sink for the mug to store the current date and timezone
    pub async fn set_time_date_zone(
        &self,
        time_date_zone: &TimeDateZone,
    ) -> Result<(), WriteError> {
        self.command(&TIME_DATE_ZONE, time_date_zone).await
    }
}

#[derive(Debug, BinWrite)]
#[bw(little)]
pub struct TimeDateZone {
    /// Unix timestamp recorded by the app.
    unix_timestamp: u32,
    /// Timezone offset (ex: GMT+03)
    offset: u32,
}
