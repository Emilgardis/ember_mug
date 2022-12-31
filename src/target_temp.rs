use super::*;
impl EmberMug {
    /// Retrieves the target temperature of the mug
    pub async fn get_target_temperature(&self) -> Result<Temperature, ReadError> {
        self.read_deserialize(&TARGET_TEMP).await
    }
    /// Set the target temperature of the mug
    pub async fn set_target_temperature(
        &self,
        temperature: &Temperature,
    ) -> Result<(), WriteError> {
        self.command(&TARGET_TEMP, temperature).await
    }
}
