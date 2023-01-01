use super::*;
impl EmberMug {
    /// Retrieve the current unit of temperature used by the mug.
    pub async fn get_temperature_unit(&self) -> Result<TemperatureUnit, ReadError> {
        self.read_deserialize(&crate::characteristics::TEMPERATURE_UNIT)
            .await
            .map_err(Into::into)
    }
    /// Set the current unit of temperature used by the mug.
    pub async fn set_temperature_unit(
        &self,
        temperature_unit: &TemperatureUnit,
    ) -> Result<(), WriteError> {
        self.command(&crate::characteristics::TEMPERATURE_UNIT, temperature_unit)
            .await
    }
}

/// Temperature unit/scale
#[derive(BinRead, BinWrite, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[br(repr = u8)]
#[bw(repr = u8)]
#[br(little)]
#[bw(little)]
pub enum TemperatureUnit {
    /// Celcius
    Celsius = 0,
    /// Fahrenheit
    Fahrenheit = 1,
}

impl std::fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Celsius => f.write_str("C"),
            Self::Fahrenheit => f.write_str("F"),
        }
    }
}
