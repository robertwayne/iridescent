use crate::color::Rgb;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackgroundColor {
    Empty,
    Simple(u8),
    Rgb(Rgb),
}

impl From<u8> for BackgroundColor {
    fn from(color: u8) -> Self {
        BackgroundColor::Simple(color)
    }
}

impl From<Rgb> for BackgroundColor {
    fn from(color: Rgb) -> Self {
        BackgroundColor::Rgb(color)
    }
}

impl From<&[u8; 3]> for BackgroundColor {
    fn from(color: &[u8; 3]) -> Self {
        BackgroundColor::Rgb(Rgb::new(color[0], color[1], color[2]))
    }
}

impl From<&str> for BackgroundColor {
    fn from(color: &str) -> Self {
        BackgroundColor::Rgb(Rgb::from(color))
    }
}
