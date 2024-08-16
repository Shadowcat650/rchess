use super::Square;

/// All the ranks in order.
pub const RANKS: [Rank; 8] = [
    Rank::First,
    Rank::Second,
    Rank::Third,
    Rank::Fourth,
    Rank::Fifth,
    Rank::Sixth,
    Rank::Seventh,
    Rank::Eighth,
];

/// A rank on the chess board.
#[repr(u8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl Rank {
    /// Gets the [`Rank`] of the given [`Square`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{Square, Rank};
    ///
    /// assert_eq!(Rank::of(Square::A1), Rank::First);
    /// assert_eq!(Rank::of(Square::H8), Rank::Eighth);
    /// assert_eq!(Rank::of(Square::E5), Rank::Fifth);
    /// ```
    #[inline]
    pub const fn of(square: Square) -> Self {
        // SAFETY: The maximum value of a square index / 8 is 7.
        unsafe { Self::from_u8_unchecked(square.as_u8() / 8) }
    }

    /// Converts the [`Rank`] into a [`u8`].
    ///
    /// # Examples
    /// ```
    /// use rchess::Rank;
    ///
    /// assert_eq!(Rank::First.to_u8(), 0);
    /// assert_eq!(Rank::Eighth.to_u8(), 7);
    /// ```
    #[inline]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Creates a new [`Rank`] from an index.
    ///
    /// If the [`u8`] is an invalid number, a `None` variant is returned.
    ///
    /// # Examples
    /// ```
    /// use rchess::Rank;
    ///
    /// assert_eq!(Rank::from_index(0), Some(Rank::First));
    /// assert_eq!(Rank::from_index(7), Some(Rank::Eighth));
    /// ```
    #[inline]
    pub const fn from_index(val: u8) -> Option<Self> {
        if val > 7 {
            return None;
        }
        // SAFETY: The index is in a valid range.
        unsafe { std::mem::transmute(val) }
    }

    /// Creates a new [`Rank`] from an [`u8`].
    ///
    /// This function does not check if the [`u8`] is valid.
    ///
    /// # Examples
    /// ```
    /// use rchess::Rank;
    ///
    /// unsafe {
    ///     assert_eq!(Rank::from_u8_unchecked(0), Rank::First);
    ///     assert_eq!(Rank::from_u8_unchecked(7), Rank::Eighth);
    /// }
    /// ```
    #[inline]
    pub const unsafe fn from_u8_unchecked(val: u8) -> Self {
        match Self::from_index(val) {
            Some(rank) => rank,
            // SAFETY: Caller upholds the safety contract.
            None => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}
