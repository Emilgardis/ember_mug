use super::*;
impl EmberMug {
    /// Retrieves the dsk of the cup
    pub async fn get_dsk(&self) -> Result<Vec<u8>, ReadError> {
        self.read(&crate::characteristics::DSK).await
    }

    /// Retrieves the dsk of the cup
    pub async fn get_udsk(&self) -> Result<Vec<u8>, ReadError> {
        self.read(&crate::characteristics::UDSK).await
    }
}