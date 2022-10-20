use crate::{
    background::BackgroundColor,
    constants::{BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, WHITE, YELLOW},
    foreground::ForegroundColor,
    StyledString,
};

/// Implements all the `StyledString` functions on any type `AsRef<str>`.
/// Handles conversion internally so that the caller doesn't have to worry about
/// it.
pub trait Styled {
    fn to_styled_string(&self) -> StyledString;

    // Various color types.
    fn foreground(&self, color: impl Into<ForegroundColor>) -> StyledString;
    fn background(&self, color: impl Into<BackgroundColor>) -> StyledString;

    // Text modes.
    fn bold(&self) -> StyledString;
    fn dim(&self) -> StyledString;
    fn italic(&self) -> StyledString;
    fn underline(&self) -> StyledString;
    fn blink(&self) -> StyledString;
    fn invert(&self) -> StyledString;
    fn hidden(&self) -> StyledString;
    fn strike(&self) -> StyledString;

    // These are helper methods for quickly styling just the foreground with
    // basic colors.
    fn black(&self) -> StyledString;
    fn red(&self) -> StyledString;
    fn green(&self) -> StyledString;
    fn yellow(&self) -> StyledString;
    fn blue(&self) -> StyledString;
    fn magenta(&self) -> StyledString;
    fn cyan(&self) -> StyledString;
    fn white(&self) -> StyledString;
}

impl<S: AsRef<str>> Styled for S {
    /// Converts an `AsRef`<str> to a `StyledString`.
    fn to_styled_string(&self) -> StyledString {
        StyledString::new(self.as_ref())
    }

    /// Sets the foreground color (the text) of the string.
    fn foreground(&self, color: impl Into<ForegroundColor>) -> StyledString {
        StyledString::new(self.as_ref()).foreground(color.into())
    }

    /// Sets the background color of the string.
    fn background(&self, color: impl Into<BackgroundColor>) -> StyledString {
        StyledString::new(self.as_ref()).background(color.into())
    }

    /// Applies the bold attribute to the string.
    fn bold(&self) -> StyledString {
        StyledString::new(self.as_ref()).bold()
    }

    /// Applies the dim attribute to the string.
    fn dim(&self) -> StyledString {
        StyledString::new(self.as_ref()).dim()
    }

    /// Applies the italic attribute to the string.
    fn italic(&self) -> StyledString {
        StyledString::new(self.as_ref()).italic()
    }

    /// Applies the underline attribute to the string.
    fn underline(&self) -> StyledString {
        StyledString::new(self.as_ref()).underline()
    }

    /// Applies the blink attribute to the string.
    fn blink(&self) -> StyledString {
        StyledString::new(self.as_ref()).blink()
    }

    /// Inverts the strings foreground and background colors.
    fn invert(&self) -> StyledString {
        StyledString::new(self.as_ref()).invert()
    }

    /// Applies the hidden attribute to the string.
    fn hidden(&self) -> StyledString {
        StyledString::new(self.as_ref()).hidden()
    }

    /// Applies the strike-through attribute to the string.
    fn strike(&self) -> StyledString {
        StyledString::new(self.as_ref()).strike()
    }

    /// Sets the foreground color to black.
    fn black(&self) -> StyledString {
        self.foreground(BLACK)
    }

    /// Sets the foreground color to red.
    fn red(&self) -> StyledString {
        self.foreground(RED)
    }

    /// Sets the foreground color to green.
    fn green(&self) -> StyledString {
        self.foreground(GREEN)
    }

    /// Sets the foreground color to yellow.
    fn yellow(&self) -> StyledString {
        self.foreground(YELLOW)
    }

    /// Sets the foreground color to blue.
    fn blue(&self) -> StyledString {
        self.foreground(BLUE)
    }

    /// Sets the foreground color to magenta.
    fn magenta(&self) -> StyledString {
        self.foreground(MAGENTA)
    }

    /// Sets the foreground color to cyan.
    fn cyan(&self) -> StyledString {
        self.foreground(CYAN)
    }

    /// Sets the foreground color to white.
    fn white(&self) -> StyledString {
        self.foreground(WHITE)
    }
}
