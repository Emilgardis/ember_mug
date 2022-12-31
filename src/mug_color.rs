use super::*;
impl EmberMug {
    /// Retrieves the color of the mug's LED indicator.
    pub async fn get_mug_color(&self) -> Result<Color, ReadError> {
        Color::read(&mut Cursor::new(self.read(&MUG_COLOR).await?)).map_err(Into::into)
    }
    /// Sets the color of the mug's LED indicator.
    pub async fn set_mug_color(&self, color: &Color) -> Result<(), WriteError> {
        self.command(&MUG_COLOR, color).await
    }
}

/// Color tint for an Ember Mug
#[derive(BinRead, BinWrite, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[br(little)]
#[bw(little)]
pub struct Color {
    /// Red value (0-255)
    pub r: u8,
    /// Green value (0-255)
    pub g: u8,
    /// Blue value (0-255)
    pub b: u8,
    /// Alpha value (0-255)
    pub a: u8,
}
