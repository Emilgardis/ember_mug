use super::*;
impl EmberMug {
    /// The current state of the mug
    pub async fn get_liquid_state(&self) -> Result<LiquidState, ReadError> {
        self.read_deserialize(&crate::KnownCharacteristic::LiquidState)
            .await
    }
}

#[derive(BinRead, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[br(repr = u8)]
#[br(little)]
/// Represents the current state of the liquid in an Ember Mug
pub enum LiquidState {
    /// The liquid state is unknown
    Unknown,
    /// The mug is empty
    Empty,
    /// The mug is filling with liquid
    Filling,
    /// The mug is cold and temperature control is disabled
    ColdNoTempControl,
    /// The mug is cooling down to the target temperature
    Cooling,
    /// The mug is heating up to the target temperature
    Heating,
    /// The mug's liquid is at the target temperature
    TargetTemperature,
    /// The mug is warm and temperature control is disabled
    WarmNoTempControl,
}

impl std::fmt::Display for LiquidState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => f.write_str("Unknown"),
            Self::Empty => f.write_str("Empty"),
            Self::Filling => f.write_str("Filling"),
            Self::ColdNoTempControl => f.write_str("Cold (No control)"),
            Self::Cooling => f.write_str("Cooling"),
            Self::Heating => f.write_str("Heating"),
            Self::TargetTemperature => f.write_str("Perfect"),
            Self::WarmNoTempControl => f.write_str("Warm (No control)"),
        }
    }
}

impl LiquidState {
    /// Returns `true` if the liquid state is [`Unknown`].
    ///
    /// [`Unknown`]: LiquidState::Unknown
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns `true` if the liquid state is [`Empty`].
    ///
    /// [`Empty`]: LiquidState::Empty
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns `true` if the liquid state is [`Filling`].
    ///
    /// [`Filling`]: LiquidState::Filling
    #[must_use]
    pub const fn is_filling(&self) -> bool {
        matches!(self, Self::Filling)
    }

    /// Returns `true` if the liquid state is [`ColdNoTempControl`].
    ///
    /// [`ColdNoTempControl`]: LiquidState::ColdNoTempControl
    #[must_use]
    pub const fn is_cold_no_temp_control(&self) -> bool {
        matches!(self, Self::ColdNoTempControl)
    }

    /// Returns `true` if the liquid state is [`Cooling`].
    ///
    /// [`Cooling`]: LiquidState::Cooling
    #[must_use]
    pub const fn is_cooling(&self) -> bool {
        matches!(self, Self::Cooling)
    }

    /// Returns `true` if the liquid state is [`Heating`].
    ///
    /// [`Heating`]: LiquidState::Heating
    #[must_use]
    pub const fn is_heating(&self) -> bool {
        matches!(self, Self::Heating)
    }

    /// Returns `true` if the liquid state is [`TargetTemperature`].
    ///
    /// [`TargetTemperature`]: LiquidState::TargetTemperature
    #[must_use]
    pub const fn is_target_temperature(&self) -> bool {
        matches!(self, Self::TargetTemperature)
    }

    /// Returns `true` if the liquid state is [`WarmNoTempControl`].
    ///
    /// [`WarmNoTempControl`]: LiquidState::WarmNoTempControl
    #[must_use]
    pub const fn is_warm_no_temp_control(&self) -> bool {
        matches!(self, Self::WarmNoTempControl)
    }
}
