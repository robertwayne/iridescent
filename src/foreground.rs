use crate::color::Rgb;

/// Represents a color that will be applied to the text of a `StyledString`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForegroundColor {
    Empty,
    Simple(u8),
    Rgb(Rgb),
}

impl From<u8> for ForegroundColor {
    fn from(color: u8) -> Self {
        ForegroundColor::Simple(color)
    }
}

impl From<Rgb> for ForegroundColor {
    fn from(color: Rgb) -> Self {
        ForegroundColor::Rgb(color)
    }
}

impl From<&[u8; 3]> for ForegroundColor {
    fn from(color: &[u8; 3]) -> Self {
        ForegroundColor::Rgb(Rgb::new(color[0], color[1], color[2]))
    }
}

impl From<&str> for ForegroundColor {
    fn from(color: &str) -> Self {
        ForegroundColor::Rgb(Rgb::from(color))
    }
}
