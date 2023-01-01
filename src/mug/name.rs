use super::*;
impl EmberMug {
    /// Retreives the name of the mug.
    pub async fn get_name(&self) -> Result<String, ReadError> {
        String::from_utf8(self.read(&crate::characteristics::NAME).await?).map_err(Into::into)
    }

    /// Sets the name of the mug.
    pub async fn set_name(&self, name: &str) -> Result<(), WriteError> {
        #[derive(BinWrite)]
        #[bw(little)]
        struct Name<'a> {
            name: &'a [u8],
        }
        if name.is_empty() || name.len() > 14 {
            // FIXME: This might be 16
            return Err(WriteError::InvalidFormat(
                "name must be between 1 and 14 ascii characters",
            ));
        }
        let mut is_valid = true;
        for c in name.chars() {
            match c {
                _ if c.is_ascii_alphabetic() => (),
                _ if c.is_ascii_digit() => (),
                ',' | '.' | '[' | ']' | '#' | '(' | ')' | '!' | '"' | '\'' | ';' | ':' | '|'
                | '-' | '_' | '+' | '<' | '>' | '%' | '=' | ' ' => (),
                _ => is_valid = false,
            }
        }
        if !is_valid {
            // FIXME: is space valid?<
            return Err(WriteError::InvalidFormat(
                r#"name must satisfy [A-Za-z0-9,.\[\]#()!\"\';:|\-_+<>%= ]"#,
            ));
        }

        self.command(
            &crate::characteristics::NAME,
            &Name {
                name: name.as_bytes(),
            },
        )
        .await
    }
}
