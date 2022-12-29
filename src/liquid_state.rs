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
    Unknown,
    Empty,
    Filling,
    ColdNoTempControl,
    Cooling,
    Heating,
    TargetTemperature,
    WarmNoTempControl,
}

impl std::fmt::Display for LiquidState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiquidState::Unknown => f.write_str("Unknown"),
            LiquidState::Empty => f.write_str("Empty"),
            LiquidState::Filling => f.write_str("Filling"),
            LiquidState::ColdNoTempControl => f.write_str("Cold (No control)"),
            LiquidState::Cooling => f.write_str("Cooling"),
            LiquidState::Heating => f.write_str("Heating"),
            LiquidState::TargetTemperature => f.write_str("Perfect"),
            LiquidState::WarmNoTempControl => f.write_str("Warm (No control)"),
        }
    }
}

impl LiquidState {
    /// Returns `true` if the liquid state is [`Unknown`].
    ///
    /// [`Unknown`]: LiquidState::Unknown
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

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

    /// Returns `true` if the liquid state is [`ColdNoTempControl`].
    ///
    /// [`ColdNoTempControl`]: LiquidState::ColdNoTempControl
    #[must_use]
    pub fn is_cold_no_temp_control(&self) -> bool {
        matches!(self, Self::ColdNoTempControl)
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

    /// Returns `true` if the liquid state is [`TargetTemperature`].
    ///
    /// [`TargetTemperature`]: LiquidState::TargetTemperature
    #[must_use]
    pub fn is_target_temperature(&self) -> bool {
        matches!(self, Self::TargetTemperature)
    }

    /// Returns `true` if the liquid state is [`WarmNoTempControl`].
    ///
    /// [`WarmNoTempControl`]: LiquidState::WarmNoTempControl
    #[must_use]
    pub fn is_warm_no_temp_control(&self) -> bool {
        matches!(self, Self::WarmNoTempControl)
    }
}
