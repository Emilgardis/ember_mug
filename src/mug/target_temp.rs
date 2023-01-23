use super::*;
impl EmberMug {
    /// Retrieves the target temperature of the mug
    pub async fn get_target_temperature(&self) -> Result<Temperature, ReadError> {
        self.read_deserialize(&crate::KnownCharacteristic::TargetTemp)
            .await
    }
    /// Set the target temperature of the mug
    pub async fn set_target_temperature(
        &self,
        temperature: &Temperature,
    ) -> Result<(), WriteError> {
        self.command(&crate::KnownCharacteristic::TargetTemp, temperature)
            .await
    }
}
