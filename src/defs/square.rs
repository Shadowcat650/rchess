use super::{BitBoard, Color, File, Rank};
use std::fmt::{Display, Formatter};

/// All the squares in order.
#[rustfmt::skip]
pub const SQUARES: [Square;64] = [
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8
];

/// The [`Square`] enum represents a square of the chess board.
///
/// Little-Endian Rank File mapping is used for square enumerations.
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

impl Square {
    /// Creates a new [`Square`] at the given [`Rank`] and [`File`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{Square, Rank, File};
    ///
    /// assert_eq!(Square::at(Rank::First, File::A), Square::A1);
    /// assert_eq!(Square::at(Rank::Eighth, File::H), Square::H8);
    /// assert_eq!(Square::at(Rank::Fifth, File::E), Square::E5);
    /// ```
    #[inline]
    pub const fn at(rank: Rank, file: File) -> Self {
        let square_index = (rank.to_u8() * 8) + file.to_u8();
        // SAFETY: square_index will always be less than 64.
        unsafe { Self::from_u8_unchecked(square_index) }
    }

    /// Creates a new [`Square`] from its little-endian rank-file [`u8`] index.
    ///
    /// If the index is not a valid [`Square`] index, a `None` value is returned.
    ///
    /// # Examples
    /// ```
    /// use rchess::Square;
    ///
    /// assert_eq!(Square::from_u8(0), Some(Square::A1));
    /// assert_eq!(Square::from_u8(63), Some(Square::H8));
    /// assert_eq!(Square::from_u8(64), None);
    /// ```
    #[inline]
    pub const fn from_u8(val: u8) -> Option<Self> {
        if val > 63 {
            return None;
        }
        // SAFETY: The val is in a valid range.
        unsafe { Some(std::mem::transmute(val)) }
    }

    /// Creates a new [`Square`] from its little-endian rank-file [`u8`] index.
    ///
    /// This function does not check if the index is valid.
    ///
    /// # Examples
    /// ```
    /// use rchess::Square;
    ///
    /// unsafe {
    ///     assert_eq!(Square::from_u8_unchecked(0), Square::A1);
    ///     assert_eq!(Square::from_u8_unchecked(63), Square::H8);
    /// }
    /// ```
    ///
    /// # Safety
    /// The caller ensures the given index is a valid [`Square`] enum variant.
    ///
    /// Valid [`Square`] enum variants range 0 through 63.
    #[inline]
    pub const unsafe fn from_u8_unchecked(val: u8) -> Self {
        // SAFETY: Caller upholds the safety contract.
        unsafe { Self::from_u8(val).unwrap_unchecked() }
    }

    /// Creates a new [`Square`] from a [`&str`].
    ///
    /// If the given [`&str`] is not a valid square, an `Err` value is returned.
    ///
    /// A valid string representations of a [`Square`] is a lowercase letter representing the file
    /// followed by a number representing the rank.
    ///
    /// # Examples
    /// ```
    /// use rchess::Square;
    ///
    /// assert_eq!(Square::from_string("a1"), Ok(Square::A1));
    /// assert_eq!(Square::from_string("h8"), Ok(Square::H8));
    /// assert!(Square::from_string("A1").is_err());
    /// assert!(Square::from_string("8h").is_err());
    /// ```
    #[inline]
    pub const fn from_string(str: &str) -> Result<Self, ()> {
        if str.len() != 2 {
            return Err(());
        }

        let file = match str.as_bytes()[0] {
            b'a' => File::A,
            b'b' => File::B,
            b'c' => File::C,
            b'd' => File::D,
            b'e' => File::E,
            b'f' => File::F,
            b'g' => File::G,
            b'h' => File::H,
            _ => return Err(()),
        };

        let rank = match str.as_bytes()[1] {
            b'1' => Rank::First,
            b'2' => Rank::Second,
            b'3' => Rank::Third,
            b'4' => Rank::Fourth,
            b'5' => Rank::Fifth,
            b'6' => Rank::Sixth,
            b'7' => Rank::Seventh,
            b'8' => Rank::Eighth,
            _ => return Err(()),
        };

        Ok(Self::at(rank, file))
    }

    /// Gets the little-endian rank file index as a [`u8`] representing the [`Square`].
    ///
    /// # Examples
    /// ```
    /// use rchess::Square;
    ///
    /// assert_eq!(Square::A1.as_u8(), 0);
    /// assert_eq!(Square::H8.as_u8(), 63);
    /// ```
    #[inline]
    pub const fn as_u8(&self) -> u8 {
        *self as u8
    }

    /// Gets a [`usize`] used to index arrays by the [`Square`].
    ///
    /// # Examples
    /// ```no_run
    /// use rchess::{Rank, Square, SQUARES};
    ///
    /// let mut pawn_on_square = [false;64];
    /// for square in SQUARES {
    ///     if square.rank() == Rank::Second || square.rank() == Rank::Seventh {
    ///         pawn_on_square[square.index()] = true;
    ///     }
    /// }
    /// ```
    #[inline]
    pub const fn index(&self) -> usize {
        *self as usize
    }

    /// Gets the [`Rank`] of the [`Square`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{Square, Rank};
    ///
    /// assert_eq!(Square::A1.rank(), Rank::First);
    /// assert_eq!(Square::H8.rank(), Rank::Eighth);
    /// assert_eq!(Square::E5.rank(), Rank::Fifth);
    /// ```
    #[inline]
    pub const fn rank(&self) -> Rank {
        Rank::of(*self)
    }

    /// Gets the [`File`] of the [`Square`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{Square, File};
    ///
    /// assert_eq!(Square::A1.file(), File::A);
    /// assert_eq!(Square::H8.file(), File::H);
    /// assert_eq!(Square::E5.file(), File::E);
    /// ```
    #[inline]
    pub const fn file(&self) -> File {
        File::of(*self)
    }

    /// Gets a [`BitBoard`] containing the [`Square`].
    ///
    /// # Examples
    /// ```no_run
    /// use rchess::{Square, BitBoard, Rank, File};
    ///
    /// assert_eq!(Square::A1.bitboard(), BitBoard::from_rank(Rank::First) & BitBoard::from_file(File::A));
    /// assert_eq!(Square::H8.bitboard(), BitBoard::from_rank(Rank::Eighth) & BitBoard::from_file(File::H));
    /// ```
    #[inline]
    pub const fn bitboard(&self) -> BitBoard {
        BitBoard::from_square(*self)
    }

    /// Gets the [`Color`] of the [`Square`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{Color, Square};
    ///
    /// assert_eq!(Square::A1.color(), Color::Black);
    /// assert_eq!(Square::H1.color(), Color::White);
    /// assert_eq!(Square::H8.color(), Color::Black);
    /// assert_eq!(Square::A8.color(), Color::White);
    /// ```
    #[inline]
    pub const fn color(&self) -> Color {
        if self.bitboard().overlaps(BitBoard::WHITE_SQUARES) {
            Color::White
        } else {
            Color::Black
        }
    }

    /// Moves the [`Square`] up one rank.
    ///
    /// If the [`Square`] is on the eighth rank, a `None` value is returned.
    ///
    /// # Examples
    /// ```
    /// use rchess::Square;
    ///
    /// assert_eq!(Square::A1.up(), Some(Square::A2));
    /// assert_eq!(Square::H8.up(), None);
    /// assert_eq!(Square::E5.up(), Some(Square::E6));
    /// ```
    #[inline]
    pub fn up(self) -> Option<Self> {
        if self.rank() == Rank::Eighth {
            return None;
        }
        unsafe { Some(Self::from_u8_unchecked(self.as_u8() + 8)) }
    }

    /// Moves the [`Square`] down one rank.
    ///
    /// If the [`Square`] is on the first rank, a `None` value is returned.
    ///
    /// # Examples
    /// ```
    /// use rchess::Square;
    ///
    /// assert_eq!(Square::A1.down(), None);
    /// assert_eq!(Square::H8.down(), Some(Square::H7));
    /// assert_eq!(Square::E5.down(), Some(Square::E4));
    /// ```
    #[inline]
    pub fn down(self) -> Option<Self> {
        if self.rank() == Rank::First {
            return None;
        }
        unsafe { Some(Self::from_u8_unchecked(self.as_u8() - 8)) }
    }

    /// Moves the [`Square`] left one file.
    ///
    /// If the [`Square`] is on the 'A' file, a `None` value is returned.
    ///
    /// # Examples
    /// ```
    /// use rchess::Square;
    ///
    /// assert_eq!(Square::A1.left(), None);
    /// assert_eq!(Square::H8.left(), Some(Square::G8));
    /// assert_eq!(Square::E5.left(), Some(Square::D5));
    /// ```
    #[inline]
    pub fn left(self) -> Option<Self> {
        if self.file() == File::A {
            return None;
        }
        unsafe { Some(Self::from_u8_unchecked(self.as_u8() - 1)) }
    }

    /// Moves the [`Square`] right one file.
    ///
    /// If the [`Square`] is on the 'H' file, a `None` value is returned.
    ///
    /// # Examples
    /// ```
    /// use rchess::Square;
    ///
    /// assert_eq!(Square::A1.right(), Some(Square::B1));
    /// assert_eq!(Square::H8.right(), None);
    /// assert_eq!(Square::E5.right(), Some(Square::F5));
    /// ```
    #[inline]
    pub fn right(self) -> Option<Self> {
        if self.file() == File::H {
            return None;
        }
        unsafe { Some(Self::from_u8_unchecked(self.as_u8() + 1)) }
    }
}

impl Display for Square {
    /// Displays the [`Square`] in algebraic chess notation.
    ///
    /// # Examples
    /// ```
    /// use rchess::Square;
    ///
    /// assert_eq!(&Square::A1.to_string(), "a1");
    /// assert_eq!(&Square::H8.to_string(), "h8");
    /// assert_eq!(&Square::E5.to_string(), "e5");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let file = match self.file() {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        };

        let rank = match self.rank() {
            Rank::First => '1',
            Rank::Second => '2',
            Rank::Third => '3',
            Rank::Fourth => '4',
            Rank::Fifth => '5',
            Rank::Sixth => '6',
            Rank::Seventh => '7',
            Rank::Eighth => '8',
        };

        write!(f, "{}{}", file, rank)
    }
}
