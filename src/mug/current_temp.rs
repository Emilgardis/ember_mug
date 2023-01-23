use super::*;
impl EmberMug {
    /// Retrieves the current temperature of the mug
    pub async fn get_current_temperature(&self) -> Result<Temperature, ReadError> {
        self.read_deserialize(&crate::KnownCharacteristic::CurrentTemp)
            .await
    }
}
