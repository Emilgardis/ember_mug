use super::*;
impl EmberMug {
    /// Retrieves id of the mug
    pub async fn get_mug_meta(&self) -> Result<MugMeta, ReadError> {
        self.read_deserialize::<MugMeta>(&crate::KnownCharacteristic::MugId)
            .await
    }
}

/// Metadata for the device
#[derive(BinRead, Debug)]
#[br(little)]
pub struct MugMeta {
    /// ID
    pub mug_id: [u8; 6],
    /// Serial number
    #[br(pad_before = 1, parse_with = until_eof)]
    pub serial_number: String,
}

// FIXME: This is a workaround for https://github.com/jam1garner/binrw/issues/239
pub fn until_eof<Reader, T, Arg>(
    reader: &mut Reader,
    endian: binrw::Endian,
    args: Arg,
) -> Result<String, binrw::Error>
where
    T: for<'a> BinRead<Args<'a> = Arg>,
    Vec<u8>: FromIterator<T>,
    Reader: std::io::Read + std::io::Seek,
    Arg: Clone,
{
    let res: Vec<u8> = binrw::helpers::until_eof::<Reader, T, Arg, Vec<u8>>(reader, endian, args)?;
    String::from_utf8(res).map_err(|e| binrw::Error::Custom {
        pos: 6 + 1,
        err: Box::new(e) as _,
    })
}
