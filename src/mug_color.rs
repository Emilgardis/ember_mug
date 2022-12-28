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

#[derive(BinRead, BinWrite, Debug)]
#[br(little)]
#[bw(little)]
pub struct Color {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
    /// Alpha
    pub a: u8,
}
