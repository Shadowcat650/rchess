use super::Square;

/// All the files in order.
pub const FILES: [File; 8] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
];

/// The [`File`] enum represents a file of the chessboard.
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    /// Gets the [`File`] of a given [`Square`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{Square, File};
    ///
    /// assert_eq!(File::of(Square::A1), File::A);
    /// assert_eq!(File::of(Square::H8), File::H);
    /// assert_eq!(File::of(Square::E5), File::E);
    /// ```
    #[inline]
    pub const fn of(square: Square) -> Self {
        // SAFETY: The maximum value of a square index % 8 is 7.
        unsafe { Self::from_u8_unchecked(square.as_u8() % 8) }
    }

    /// Converts the [`File`] into a [`u8`].
    ///
    /// # Examples
    /// ```
    /// use rchess::File;
    ///
    /// assert_eq!(File::A.to_u8(), 0);
    /// assert_eq!(File::H.to_u8(), 7);
    /// ```
    #[inline]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Creates a new [`File`] from a [`u8`].
    ///
    /// If the [`u8`] is an invalid number, a `None` variant is returned.
    ///
    /// # Examples
    /// ```
    /// use rchess::File;
    ///
    /// assert_eq!(File::from_u8(0), Some(File::A));
    /// assert_eq!(File::from_u8(7), Some(File::H));
    /// ```
    #[inline]
    pub const fn from_u8(val: u8) -> Option<Self> {
        if val > 7 {
            return None;
        }
        // SAFETY: The index is in a valid range.
        unsafe { std::mem::transmute(val) }
    }

    /// Creates a new [`File`] from an [`u8`].
    ///
    /// This function does not check if the [`u8`] is valid.
    ///
    /// # Examples
    /// ```
    /// use rchess::File;
    ///
    /// unsafe {
    ///     assert_eq!(File::from_u8_unchecked(0), File::A);
    ///     assert_eq!(File::from_u8_unchecked(7), File::H);
    /// }
    /// ```
    #[inline]
    pub const unsafe fn from_u8_unchecked(val: u8) -> Self {
        match Self::from_u8(val) {
            Some(file) => file,
            // SAFETY: Caller upholds the safety contract.
            None => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}
