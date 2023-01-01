use super::*;
impl EmberMug {
    /// Info about the current firmware running on the mug.
    pub async fn get_ota(&self) -> Result<Ota, ReadError> {
       self.read_deserialize(&crate::characteristics::OTA).await
    }
}

/// Version information for the device
#[derive(BinRead, BinWrite, Debug)]
#[br(little)]
#[bw(little)]
pub struct Ota {
    /// Firmware version
    pub firmware_version: u16,
    /// Hardware version
    pub hardware_version: u16,
    #[doc(hidden)]
    #[br(try)]
    pub bootloader: Option<u16>,
}
