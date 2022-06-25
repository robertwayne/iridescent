use crate::rgb::Rgb;

/// Represents a color that will be applied to a `StyledString`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Empty,
    Foreground(u8),
    Background(u8),
    RgbForeground(Rgb),
    RgbBackground(Rgb),
}

impl Color {
    /// Returns a single, random color value for use with 8-bit ANSI escape sequences.
    ///
    /// Requires the `random` features to be enabled (on by default).
    #[cfg(feature = "random")]
    pub fn random() -> u8 {
        rand::random::<u8>()
    }

    /// Returns an Rgb struct with 3 random values for use with 24-bit ANSI escape sequences.
    ///
    /// Requires the `random` feature to be enabled (on by default).
    #[cfg(feature = "random")]
    pub fn random_rgb() -> Rgb {
        {
            Rgb::new(
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>(),
            )
        }
    }
}
