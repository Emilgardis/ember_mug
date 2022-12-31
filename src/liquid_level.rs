use super::*;
impl EmberMug {
    /// Retrieves the level of liquid present in the cup
    pub async fn get_liquid_level(&self) -> Result<LiquidLevel, ReadError> {
        LiquidLevel::read(&mut Cursor::new(self.read(&LIQUID_LEVEL).await?)).map_err(Into::into)
    }
}

/// Level of the liquid
///
/// # Notes
///
/// This seems to be highly unspecific, 0 = empty, not 0 = has liquid
#[derive(BinRead, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
#[br(little)]
pub struct LiquidLevel {
    /// The given amount of liquid
    pub level: u8,
}

impl LiquidLevel {
    /// Mug is empty
    pub fn is_empty(&self) -> bool {
        self.level == 0
    }

    /// Mug has liquid
    pub fn has_liquid(&self) -> bool {
        self.level != 0
    }
}
