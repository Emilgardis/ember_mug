use super::*;
impl EmberMug {
    /// The current state of the mug
    pub async fn get_liquid_state(&self) -> Result<LiquidState, ReadError> {
        LiquidState::read(&mut Cursor::new(self.read(&LIQUID_STATE).await?)).map_err(Into::into)
    }
}

#[derive(BinRead, Debug)]
#[br(repr = u8)]
#[br(little)]
pub enum LiquidState {
    Empty = 1,
    Filling = 2,
    Unknown = 3,
    Cooling = 4,
    Heating = 5,
    StableTemperature = 6,
}

impl LiquidState {
    /// Returns `true` if the liquid state is [`Empty`].
    ///
    /// [`Empty`]: LiquidState::Empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns `true` if the liquid state is [`Filling`].
    ///
    /// [`Filling`]: LiquidState::Filling
    #[must_use]
    pub fn is_filling(&self) -> bool {
        matches!(self, Self::Filling)
    }

    /// Returns `true` if the liquid state is [`Cooling`].
    ///
    /// [`Cooling`]: LiquidState::Cooling
    #[must_use]
    pub fn is_cooling(&self) -> bool {
        matches!(self, Self::Cooling)
    }

    /// Returns `true` if the liquid state is [`Heating`].
    ///
    /// [`Heating`]: LiquidState::Heating
    #[must_use]
    pub fn is_heating(&self) -> bool {
        matches!(self, Self::Heating)
    }

    /// Returns `true` if the liquid state is [`StableTemperature`].
    ///
    /// [`StableTemperature`]: LiquidState::StableTemperature
    #[must_use]
    pub fn is_stable_temperature(&self) -> bool {
        matches!(self, Self::StableTemperature)
    }
}
