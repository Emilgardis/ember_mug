use super::*;
impl EmberMug {
    /// Retrieves the level of liquid present in the cup
    pub async fn get_liquid_level(&self) -> Result<LiquidLevel, ReadError> {
        LiquidLevel::read(&mut Cursor::new(self.read(&LIQUID_LEVEL).await?)).map_err(Into::into)
    }
}

#[derive(BinRead, Debug)]
#[br(little)]
pub struct LiquidLevel {
    level: u8,
}

impl LiquidLevel {
    pub fn is_empty(&self) -> bool {
        self.level == 0
    }

    pub fn has_liquid(&self) -> bool {
        self.level != 0
    }
}
