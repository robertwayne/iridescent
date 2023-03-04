#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

/// An enum representing a background color type.
pub mod background;
/// An enum representing different color types applied to a `StyledString`.
pub mod color;
/// Various constants used by the library, including the base ANSI color values.
pub mod constants;
/// An enum representing a foreground color type.
pub mod foreground;
/// Trait implementing various methods on `&str` and `String` types.
pub mod styled;
/// A struct representing the internal state of an `&str` or `String` type with
/// applied styles.
pub mod styled_string;

pub use crate::{
    background::*, color::*, constants::*, foreground::*, styled::*, styled_string::*,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreground_const_colors() {
        let black = "black".black();
        let red = "red".red();
        let green = "green".green();
        let yellow = "yellow".yellow();
        let blue = "blue".blue();
        let magenta = "magenta".magenta();
        let cyan = "cyan".cyan();
        let white = "white".white();

        assert_eq!(black.to_string(), "\x1b[30mblack\x1b[0m");
        assert_eq!(red.to_string(), "\x1b[31mred\x1b[0m");
        assert_eq!(green.to_string(), "\x1b[32mgreen\x1b[0m");
        assert_eq!(yellow.to_string(), "\x1b[33myellow\x1b[0m");
        assert_eq!(blue.to_string(), "\x1b[34mblue\x1b[0m");
        assert_eq!(magenta.to_string(), "\x1b[35mmagenta\x1b[0m");
        assert_eq!(cyan.to_string(), "\x1b[36mcyan\x1b[0m");
        assert_eq!(white.to_string(), "\x1b[37mwhite\x1b[0m");
    }

    #[test]
    fn test_background_const_colors() {
        let black = "black".background(BLACK);
        let red = "red".background(RED);
        let green = "green".background(GREEN);
        let yellow = "yellow".background(YELLOW);
        let blue = "blue".background(BLUE);
        let magenta = "magenta".background(MAGENTA);
        let cyan = "cyan".background(CYAN);
        let white = "white".background(WHITE);

        assert_eq!(black.to_string(), "\x1b[40mblack\x1b[0m");
        assert_eq!(red.to_string(), "\x1b[41mred\x1b[0m");
        assert_eq!(green.to_string(), "\x1b[42mgreen\x1b[0m");
        assert_eq!(yellow.to_string(), "\x1b[43myellow\x1b[0m");
        assert_eq!(blue.to_string(), "\x1b[44mblue\x1b[0m");
        assert_eq!(magenta.to_string(), "\x1b[45mmagenta\x1b[0m");
        assert_eq!(cyan.to_string(), "\x1b[46mcyan\x1b[0m");
        assert_eq!(white.to_string(), "\x1b[47mwhite\x1b[0m");
    }

    #[test]
    fn test_modes() {
        let bold = "bold".bold();
        let dim = "dim".dim();
        let italic = "italic".italic();
        let underline = "underline".underline();
        let blink = "blink".blink();
        let invert = "invert".invert();
        let hidden = "hidden".hidden();
        let strike = "strike".strike();

        assert_eq!(bold.to_string(), "\x1b[1mbold\x1b[0m");
        assert_eq!(dim.to_string(), "\x1b[2mdim\x1b[0m");
        assert_eq!(italic.to_string(), "\x1b[3mitalic\x1b[0m");
        assert_eq!(underline.to_string(), "\x1b[4munderline\x1b[0m");
        assert_eq!(blink.to_string(), "\x1b[5mblink\x1b[0m");
        assert_eq!(invert.to_string(), "\x1b[7minvert\x1b[0m");
        assert_eq!(hidden.to_string(), "\x1b[8mhidden\x1b[0m");
        assert_eq!(strike.to_string(), "\x1b[9mstrike\x1b[0m");
    }

    #[test]
    fn test_high_bit_depth() {
        let pure_red = "pure red".foreground(Rgb::new(255, 0, 0));

        let pure_green = "pure green".foreground(Rgb::new(0, 255, 0));

        let pure_blue = "pure blue".foreground(Rgb::new(0, 0, 255));

        let bg = "random background".background(Rgb::new(50, 50, 50));

        let combined =
            "combined".foreground(Rgb::new(255, 255, 255)).background(Rgb::new(50, 50, 50));

        let combined_with_modes = "combined with modes"
            .bold()
            .foreground(Rgb::new(125, 125, 125))
            .background(Rgb::new(25, 25, 25));

        assert_eq!(pure_red.to_string(), "\x1b[38;2;255;0;0mpure red\x1b[0m");
        assert_eq!(pure_green.to_string(), "\x1b[38;2;0;255;0mpure green\x1b[0m");
        assert_eq!(pure_blue.to_string(), "\x1b[38;2;0;0;255mpure blue\x1b[0m");
        assert_eq!(bg.to_string(), "\x1b[48;2;50;50;50mrandom background\x1b[0m");
        assert_eq!(combined.to_string(), "\x1b[38;2;255;255;255;48;2;50;50;50mcombined\x1b[0m");
        assert_eq!(
            combined_with_modes.to_string(),
            "\x1b[1;38;2;125;125;125;48;2;25;25;25mcombined with modes\x1b[0m"
        );
    }

    #[test]
    fn test_hexadecimal_strings() {
        let white = "white".foreground("#ffffff");
        let black = "black".foreground("#000000");

        assert_eq!(white.to_string(), "\x1b[38;2;255;255;255mwhite\x1b[0m");
        assert_eq!(black.to_string(), "\x1b[38;2;0;0;0mblack\x1b[0m");
    }
}
