/// Simple new-type wrapper around u8 for providing extra API's when working
/// with colors.
pub struct Simple(u8);

impl Simple {
    /// Returns a single, random color value for use with 8-bit ANSI escape
    /// sequences.
    ///
    /// Requires the `random` features to be enabled (on by default).
    #[cfg(feature = "random")]
    pub fn random() -> u8 {
        rand::random::<u8>()
    }
}

/// Represents a RGB (red-green-blue) color value as 3 values between 0-255.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    #[must_use]
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl From<&[u8; 3]> for Rgb {
    fn from(color: &[u8; 3]) -> Self {
        Self::new(color[0], color[1], color[2])
    }
}

impl From<&str> for Rgb {
    fn from(color: &str) -> Self {
        let color = color.trim_start_matches('#');
        let red = u8::from_str_radix(&color[0..2], 16).unwrap();
        let green = u8::from_str_radix(&color[2..4], 16).unwrap();
        let blue = u8::from_str_radix(&color[4..6], 16).unwrap();

        Self::new(red, green, blue)
    }
}

impl Rgb {
    /// Returns an Rgb struct with 3 random values for use with 24-bit ANSI
    /// escape sequences.
    ///
    /// Requires the `random` feature to be enabled (on by default).
    #[cfg(feature = "random")]
    pub fn random() -> Rgb {
        {
            Rgb::new(
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>(),
            )
        }
    }
}
