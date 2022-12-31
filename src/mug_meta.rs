use super::*;
impl EmberMug {
    /// Retrieves id of the mug
    pub async fn get_mug_meta(&self) -> Result<MugMeta, ReadError> {
        let b = self.read(&MUG_ID).await?;
        println!("{}", b.as_slice().escape_ascii());
        MugMeta::read(&mut Cursor::new(b)).map_err(Into::into)
    }
}

/// Metadata for the device
#[derive(BinRead, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[br(little)]
pub struct MugMeta {
    /// ID
    pub mug_id: [u8; 6],
    /// Serial number
    #[br(pad_before = 1, parse_with = binrw::helpers::until_eof, try_map = String::from_utf8)]
    pub serial_number: String,
}
