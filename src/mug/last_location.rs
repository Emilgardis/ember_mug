use super::*;
impl EmberMug {
    #[doc(hidden)]
    pub async fn set_last_location(&self, last_location: &LastLocation) -> Result<(), WriteError> {
        self.command(&crate::characteristics::LAST_LOCATION, last_location)
            .await
    }
}

/// Location
#[derive(BinRead, BinWrite, Debug)]
#[br(little)]
#[bw(little)]
pub struct LastLocation {
    #[br(parse_with = binrw::helpers::until_eof)]
    loc: Vec<u8>,
}
