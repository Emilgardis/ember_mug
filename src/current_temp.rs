use super::*;
impl EmberMug {
    /// Retrieves the current temperature of the mug
    pub async fn get_current_temperature(&self) -> Result<Temperature, ReadError> {
        Temperature::read(&mut Cursor::new(self.read(&CURRENT_TEMP).await?)).map_err(Into::into)
    }
}

