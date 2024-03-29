use std::fmt::Display;

use crate::{
    background::BackgroundColor,
    constants::{
        BACKGROUND, BLACK, BLINK, BLUE, BOLD, CYAN, DIM, FOREGROUND, GREEN, HIDDEN, HIGH_DEPTH,
        INVERT, ITALIC, LOW_DEPTH, MAGENTA, RED, STRIKE, UNDERLINE, WHITE, YELLOW,
    },
    foreground::ForegroundColor,
};

/// Represents a string with internal data for the ANSI escape sequences, so it
/// can be constructed when the `Display` is called. It is preferred to use the
/// `Styled` trait to interact with your strings instead of manually
/// constructing a `StyledString`, which is more verbose.
#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use]
pub struct StyledString {
    text: String,
    modes: Vec<u8>,
    foreground: ForegroundColor,
    background: BackgroundColor,
}

impl Default for StyledString {
    fn default() -> Self {
        Self {
            text: String::new(),
            modes: Vec::new(),
            foreground: ForegroundColor::Empty,
            background: BackgroundColor::Empty,
        }
    }
}

impl From<&str> for StyledString {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for StyledString {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl StyledString {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            text: text.as_ref().to_string(),
            ..Default::default()
        }
    }

    /// Sets the text color of the string.
    pub fn foreground(mut self, color: impl Into<ForegroundColor>) -> Self {
        self.foreground = color.into();
        self
    }

    /// Sets the background color of the string.
    pub fn background(mut self, color: impl Into<BackgroundColor>) -> Self {
        self.background = color.into();
        self
    }

    /// Applies the bold attribute to the string.
    pub fn bold(mut self) -> Self {
        self.modes.push(BOLD);
        self
    }

    /// Applies the dim attribute to the string.
    pub fn dim(mut self) -> Self {
        self.modes.push(DIM);
        self
    }

    /// Applies the italic attribute to the string.
    pub fn italic(mut self) -> Self {
        self.modes.push(ITALIC);
        self
    }

    /// Applies the underline attribute to the string.
    pub fn underline(mut self) -> Self {
        self.modes.push(UNDERLINE);
        self
    }

    /// Applies the blink attribute to the string.
    pub fn blink(mut self) -> Self {
        self.modes.push(BLINK);
        self
    }

    /// Inverts the strings foreground and background colors.
    pub fn invert(mut self) -> Self {
        self.modes.push(INVERT);
        self
    }

    /// Applies the hidden attribute to the string.
    pub fn hidden(mut self) -> Self {
        self.modes.push(HIDDEN);
        self
    }

    /// Applies the strike-through attribute to the string.
    pub fn strike(mut self) -> Self {
        self.modes.push(STRIKE);
        self
    }
}

impl Display for StyledString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We need to apply the sequence codes in order.
        //     Open: \x1b[
        //     Close: \x1b[0m
        //
        // Each code is delimited with a semicolon.
        // A 3 or 4 bit color is represented by a constant:
        //     x1b[38Hello, world!\x1b[0m
        //
        // 8 and 24 bit colors are represented by a sequence of 3 to 6 numbers;
        //     8-bit: POSITION;5;COLOR
        //     24-bit: POSITION;2;RED;GREEN;BLUE
        //
        // The POSITION represents the foreground or background.
        //     38 = foreground
        //     48 = background
        //
        // The middle number (5 or 2) is the color format (ie. 8-bit or 24-bit).
        //
        // A full color sequence can look like such:
        //     \x1b[1;38;5;0;48;5;255mHello, world!\x1b[0m
        //
        // This would display "Hello, world!" in bold with a white background
        // and black foreground. Broken down, the sequence would be:
        //
        //     OPEN BOLD FOREGROUND 8-BIT COLOR BACKGROUND 8-BIT COLOR CLOSE
        let mut sequence: Vec<u8> = Vec::new();

        // Modes come first in the sequence.
        for mode in &self.modes {
            sequence.push(*mode);
        }

        // Colors come next; we will apply foreground then background.
        match &self.foreground {
            ForegroundColor::Simple(color) => {
                if [BLACK, RED, GREEN, YELLOW, BLUE, MAGENTA, CYAN, WHITE].contains(color) {
                    sequence.push(*color);
                } else {
                    sequence.push(FOREGROUND);
                    sequence.push(LOW_DEPTH);
                    sequence.push(*color);
                }
            }
            ForegroundColor::Rgb(rgb) => {
                sequence.push(FOREGROUND);
                sequence.push(HIGH_DEPTH);
                sequence.push(rgb.red);
                sequence.push(rgb.green);
                sequence.push(rgb.blue);
            }
            ForegroundColor::Empty => {}
        }

        match &self.background {
            BackgroundColor::Simple(color) => {
                if [BLACK, RED, GREEN, YELLOW, BLUE, MAGENTA, CYAN, WHITE].contains(color) {
                    sequence.push(*color + 10);
                } else {
                    sequence.push(BACKGROUND);
                    sequence.push(LOW_DEPTH);
                    sequence.push(*color);
                }
            }
            BackgroundColor::Rgb(rgb) => {
                sequence.push(BACKGROUND);
                sequence.push(HIGH_DEPTH);
                sequence.push(rgb.red);
                sequence.push(rgb.green);
                sequence.push(rgb.blue);
            }
            BackgroundColor::Empty => {}
        }

        let delimited_sequence = sequence
            .iter()
            .map(|byte| format!("{byte}"))
            .collect::<Vec<String>>()
            .join(";");

        let text = format!("\x1b[{}m{}\x1b[0m", &delimited_sequence, &self.text);

        write!(f, "{text}")
    }
}
