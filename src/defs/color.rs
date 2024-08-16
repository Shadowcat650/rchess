use std::ops::Not;

/// Represents the color of a chess piece.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Gets the character representation of the [`Color`].
    ///
    /// # Examples
    /// ```
    /// use rchess::Color;
    ///
    /// assert_eq!(Color::White.to_char(), 'w');
    /// assert_eq!(Color::Black.to_char(), 'b');
    /// ```
    #[inline]
    pub const fn to_char(self) -> char {
        match self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }

    /// Gets a [`usize`] used to index arrays by the [`Color`].
    ///
    /// # Examples
    /// ```no_run
    /// use rchess::Color;
    ///
    /// let mut color_pieces = [0;2];
    /// color_pieces[Color::White.index()] = 18;
    /// color_pieces[Color::Black.index()] = 18;
    /// ```
    #[inline]
    pub const fn index(&self) -> usize {
        *self as usize
    }
}

impl Not for Color {
    type Output = Self;

    /// Toggles the [`Color`] from white to black and vice-versa.
    ///
    /// # Examples
    /// ```
    /// use rchess::Color;
    ///
    /// assert_eq!(!Color::White, Color::Black);
    /// assert_eq!(!Color::Black, Color::White);
    /// ```
    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
