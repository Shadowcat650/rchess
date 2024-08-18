use super::{Direction, File, Rank, Square, FILES, RANKS};
use std::fmt::{Display, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// The [`BitBoard`] struct stores a series of squares on a chess board.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BitBoard {
    val: u64,
}

impl BitBoard {
    /// A [`BitBoard`] with no squares stored.
    pub const EMPTY: BitBoard = Self { val: 0 };

    /// A [`BitBoard`] with all the squares stored.
    pub const FULL: BitBoard = Self { val: u64::MAX };

    /// A [`BitBoard`] with all the white squares stored.
    pub const WHITE_SQUARES: BitBoard = Self {
        val: 0x55AA55AA55AA55AA
    };

    /// A [`BitBoard`] with all the black squares stored.
    pub const BLACK_SQUARES: BitBoard = Self {
        val: 0xAA55AA55AA55AA55
    };

    /// Creates a new [`BitBoard`] containing only the given [`Square`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{Square, BitBoard, Rank, File};
    ///
    /// assert_eq!(BitBoard::from_square(Square::A1), BitBoard::from_rank(Rank::First) & BitBoard::from_file(File::A));
    /// assert_eq!(BitBoard::from_square(Square::H8), BitBoard::from_rank(Rank::Eighth) & BitBoard::from_file(File::H));
    /// ```
    #[inline]
    pub const fn from_square(square: Square) -> Self {
        Self {
            val: 1u64 << square.as_u8(),
        }
    }

    /// Creates a new [`BitBoard`] containing all the given [`Square`]'s.
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, SQUARES};
    ///
    /// assert_eq!(BitBoard::from_squares(SQUARES.as_slice()), BitBoard::FULL);
    /// ```
    #[inline]
    pub const fn from_squares(squares: &[Square]) -> Self {
        let mut bitboard = Self::EMPTY;
        let mut index = 0;
        loop {
            if index >= squares.len() {
                break;
            }
            bitboard = bitboard.or(Self::from_square(squares[index]));
            index += 1;
        }
        bitboard
    }

    /// Creates a new [`BitBoard`] containing all the squares on a given [`Rank`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, Rank, RANKS, Square, SQUARES};
    ///
    /// for rank in RANKS {
    ///     let rank_squares = SQUARES.into_iter().filter(|sq| sq.rank() == rank).collect::<Vec<_>>();
    ///     let rank_bitboard = BitBoard::from_rank(rank);
    ///     assert_eq!(rank_bitboard, BitBoard::from_squares(rank_squares.as_slice()));
    /// }
    /// ```
    #[inline]
    pub const fn from_rank(rank: Rank) -> Self {
        let rank_1 = Self::from_squares(&[
            Square::A1,
            Square::B1,
            Square::C1,
            Square::D1,
            Square::E1,
            Square::F1,
            Square::G1,
            Square::H1,
        ]);
        Self {
            val: rank_1.val << (rank.to_u8() * 8),
        }
    }

    /// Creates a new [`BitBoard`] containing all the squares on a given [`File`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, File, FILES, Square, SQUARES};
    ///
    /// for file in FILES {
    ///     let rank_squares = SQUARES.into_iter().filter(|sq| sq.file() == file).collect::<Vec<_>>();
    ///     let rank_bitboard = BitBoard::from_file(file);
    ///     assert_eq!(rank_bitboard, BitBoard::from_squares(rank_squares.as_slice()));
    /// }
    /// ```
    #[inline]
    pub const fn from_file(file: File) -> Self {
        let file_a = Self::from_squares(&[
            Square::A1,
            Square::A2,
            Square::A3,
            Square::A4,
            Square::A5,
            Square::A6,
            Square::A7,
            Square::A8,
        ]);
        Self {
            val: file_a.val << file.to_u8(),
        }
    }

    /// Checks if the [`BitBoard`] has squares in common with other [`BitBoard`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{Square, BitBoard};
    ///
    /// assert!(BitBoard::from_squares(&[Square::A1, Square::E5]).overlaps(BitBoard::from_squares(&[Square::H8, Square::E5])));
    /// assert!(!BitBoard::from_square(Square::A1).overlaps(BitBoard::from_square(Square::H8)));
    /// assert!(!BitBoard::EMPTY.overlaps(BitBoard::EMPTY));
    /// ```
    #[inline]
    pub const fn overlaps(&self, other: Self) -> bool {
        self.val & other.val != 0
    }

    /// Checks if the [`BitBoard`] contains a given [`Square`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, Square};
    ///
    /// assert!(BitBoard::from_squares(&[Square::A1, Square::E5]).contains(Square::A1));
    /// assert!(!BitBoard::from_square(Square::A1).contains(Square::H8));
    /// assert!(!BitBoard::EMPTY.contains(Square::A1));
    /// ```
    #[inline]
    pub const fn contains(&self, square: Square) -> bool {
        self.overlaps(BitBoard::from_square(square))
    }

    /// Counts the number of squares stored in the [`BitBoard`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, Square};
    ///
    /// assert_eq!(BitBoard::EMPTY.popcnt(), 0);
    /// assert_eq!(BitBoard::from_squares(&[Square::A1, Square::A2, Square::A3]).popcnt(), 3);
    /// assert_eq!(BitBoard::FULL.popcnt(), 64);
    /// ```
    #[inline]
    pub const fn popcnt(&self) -> u8 {
        self.val.count_ones() as u8
    }

    /// Performs a forward bitscan on the [`BitBoard`], returning the lowest-indexed [`Square`].
    ///
    /// Returns a `None` value if the [`BitBoard`] is empty.
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, Square};
    ///
    /// assert_eq!(BitBoard::from_squares(&[Square::A1, Square::H8]).b_scan_forward(), Some(Square::A1));
    /// assert_eq!(BitBoard::from_squares(&[Square::E5, Square::F7]).b_scan_forward(), Some(Square::E5));
    /// assert_eq!(BitBoard::EMPTY.b_scan_forward(), None);
    /// ```
    #[inline]
    pub const fn b_scan_forward(&self) -> Option<Square> {
        if self.is_empty() {
            return None;
        }
        // SAFETY: The `BitBoard` is not empty, so the maximum number of trailing zeros is 63.
        unsafe { Some(Square::from_u8_unchecked(self.val.trailing_zeros() as u8)) }
    }

    /// Performs a reverse bitscan on the [`BitBoard`], returning the highest-indexed [`Square`].
    ///
    /// Returns a `None` value if the [`BitBoard`] is empty.
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, Square};
    ///
    /// assert_eq!(BitBoard::from_squares(&[Square::A1, Square::H8]).b_scan_reverse(), Some(Square::H8));
    /// assert_eq!(BitBoard::from_squares(&[Square::E5, Square::F7]).b_scan_reverse(), Some(Square::F7));
    /// assert_eq!(BitBoard::EMPTY.b_scan_reverse(), None);
    /// ```
    #[inline]
    pub const fn b_scan_reverse(&self) -> Option<Square> {
        if self.is_empty() {
            return None;
        }
        // SAFETY: The `BitBoard` is not empty, so the maximum number of leading zeros is 63.
        unsafe {
            Some(Square::from_u8_unchecked(
                63 - self.val.leading_zeros() as u8,
            ))
        }
    }

    /// Checks if the [`BitBoard`] contains no squares.
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, Square};
    ///
    /// assert!(BitBoard::EMPTY.is_empty());
    /// assert!(!BitBoard::FULL.is_empty());
    /// assert!(!BitBoard::from_square(Square::A1).is_empty());
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.val == 0
    }

    /// Creates a new [`BitBoard`] from a [`u64`].
    ///
    /// # Examples
    /// ```
    /// use rchess::BitBoard;
    ///
    /// assert_eq!(BitBoard::from_u64(0), BitBoard::EMPTY);
    /// assert_eq!(BitBoard::from_u64(u64::MAX), BitBoard::FULL);
    /// ```
    #[inline]
    pub const fn from_u64(val: u64) -> Self {
        Self { val }
    }

    /// Gets the [`u64`] representation of the [`BitBoard`].
    ///
    /// Each signed bit in the [`u64`] represents a [`Square`].
    ///
    /// # Examples
    /// ```
    /// use rchess::BitBoard;
    ///
    /// assert_eq!(BitBoard::EMPTY.to_u64(), 0);
    /// assert_eq!(BitBoard::FULL.to_u64(), u64::MAX);
    /// ```
    #[inline]
    pub const fn to_u64(self) -> u64 {
        self.val
    }

    /// Shifts all the [`Square`]'s in the [`BitBoard`] up one rank.
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, Square};
    ///
    /// let bb = BitBoard::from_square(Square::E5);
    /// assert_eq!(bb.up().b_scan_forward().unwrap(), Square::E6);
    /// ```
    #[inline]
    pub const fn up(mut self) -> Self {
        self.val <<= 8;
        self
    }

    /// Shifts all the [`Square`]'s in the [`BitBoard`] down one rank.
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, Square};
    ///
    /// let bb = BitBoard::from_square(Square::E5);
    /// assert_eq!(bb.down().b_scan_forward().unwrap(), Square::E4);
    /// ```
    #[inline]
    pub const fn down(mut self) -> Self {
        self.val >>= 8;
        self
    }

    /// Shifts all the [`Square`]'s in the [`BitBoard`] left one file.
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, Square};
    ///
    /// let bb = BitBoard::from_square(Square::E5);
    /// assert_eq!(bb.left().b_scan_forward().unwrap(), Square::D5);
    /// ```
    #[inline]
    pub const fn left(mut self) -> Self {
        self.val >>= 1;
        self
    }

    /// Shifts all the [`Square`]'s in the [`BitBoard`] left one file.
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, Square};
    ///
    /// let bb = BitBoard::from_square(Square::E5);
    /// assert_eq!(bb.right().b_scan_forward().unwrap(), Square::F5);
    /// ```
    #[inline]
    pub const fn right(mut self) -> Self {
        self.val <<= 1;
        self
    }

    /// Moves all the [`Square`]'s in the [`BitBoard`] in a given [`Direction`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, Direction, Square};
    ///
    /// let bb = BitBoard::from_square(Square::E5);
    /// assert_eq!(bb.shift_dir(Direction::Up).b_scan_forward().unwrap(), Square::E6);
    /// assert_eq!(bb.shift_dir(Direction::Down).b_scan_forward().unwrap(), Square::E4);
    /// assert_eq!(bb.shift_dir(Direction::Left).b_scan_forward().unwrap(), Square::D5);
    /// assert_eq!(bb.shift_dir(Direction::Right).b_scan_forward().unwrap(), Square::F5);
    /// assert_eq!(bb.shift_dir(Direction::UpLeft).b_scan_forward().unwrap(), Square::D6);
    /// assert_eq!(bb.shift_dir(Direction::UpRight).b_scan_forward().unwrap(), Square::F6);
    /// assert_eq!(bb.shift_dir(Direction::DownLeft).b_scan_forward().unwrap(), Square::D4);
    /// assert_eq!(bb.shift_dir(Direction::DownRight).b_scan_forward().unwrap(), Square::F4);
    /// ```
    #[inline]
    pub const fn shift_dir(self, dir: Direction) -> Self {
        match dir {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::UpLeft => self.up().left(),
            Direction::UpRight => self.up().right(),
            Direction::DownLeft => self.down().left(),
            Direction::DownRight => self.down().right(),
        }
    }

    /// Performs a const logical or on all [`Square`]'s in the [`BitBoard`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard};
    ///
    /// assert_eq!(BitBoard::FULL | BitBoard::FULL, BitBoard::FULL);
    /// assert_eq!(BitBoard::FULL | BitBoard::EMPTY, BitBoard::FULL);
    /// assert_eq!(BitBoard::EMPTY | BitBoard::EMPTY, BitBoard::EMPTY);
    /// ```
    #[inline]
    pub const fn or(mut self, other: Self) -> Self {
        self.val |= other.val;
        self
    }

    /// Performs a const logical and on all [`Square`]'s in the [`BitBoard`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard};
    ///
    /// assert_eq!(BitBoard::FULL & BitBoard::FULL, BitBoard::FULL);
    /// assert_eq!(BitBoard::FULL & BitBoard::EMPTY, BitBoard::EMPTY);
    /// assert_eq!(BitBoard::EMPTY & BitBoard::EMPTY, BitBoard::EMPTY);
    /// ```
    #[inline]
    pub const fn and(mut self, other: Self) -> Self {
        self.val &= other.val;
        self
    }

    /// Performs a const logical xor on all [`Square`]'s in the [`BitBoard`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard};
    ///
    /// assert_eq!(BitBoard::FULL ^ BitBoard::FULL, BitBoard::EMPTY);
    /// assert_eq!(BitBoard::FULL ^ BitBoard::EMPTY, BitBoard::FULL);
    /// assert_eq!(BitBoard::EMPTY ^ BitBoard::EMPTY, BitBoard::EMPTY);
    /// ```
    #[inline]
    pub const fn xor(mut self, other: Self) -> Self {
        self.val ^= other.val;
        self
    }

    /// Makes all empty [`Square`]'s occupied and occupied [`Square`]'s empty on the [`BitBoard`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard};
    ///
    /// assert_eq!(!BitBoard::FULL, BitBoard::EMPTY);
    /// assert_eq!(!BitBoard::EMPTY, BitBoard::FULL);
    /// ```
    #[inline]
    pub const fn neg(mut self) -> Self {
        self.val = !self.val;
        self
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = (*self).or(rhs);
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitAndAssign for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = (*self).and(rhs);
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

impl BitXorAssign for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = (*self).xor(rhs);
    }
}

impl Not for BitBoard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        self.neg()
    }
}

/// Iterates through all the [`Square`]'s stored in the [`BitBoard`].
impl Iterator for BitBoard {
    type Item = Square;

    /// Gets the next [`Square`] in the [`BitBoard`].
    ///
    /// # Examples
    /// ```
    /// use rchess::{BitBoard, Square, SQUARES};
    ///
    /// let mut not_found_squares = SQUARES.to_vec();
    /// let mut remaining = 64;
    /// for square in BitBoard::FULL {
    ///     assert_eq!(not_found_squares.len(), remaining);
    ///     not_found_squares = not_found_squares.into_iter().filter(|sq| *sq != square).collect();
    ///     remaining -= 1;
    /// }
    /// assert!(not_found_squares.is_empty());
    /// ```
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let square = self.b_scan_forward()?;
        self.val &= self.val.wrapping_sub(1);
        Some(square)
    }
}

impl Display for BitBoard {
    /// Displays the [`BitBoard`] in a readable manner.
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for rank in RANKS.into_iter().rev() {
            for file in FILES {
                let square = Square::at(rank, file);
                if self.contains(square) {
                    write!(f, "@")?;
                } else {
                    write!(f, "-")?;
                }

                if file == File::H {
                    writeln!(f)?;
                } else {
                    write!(f, " ")?;
                }
            }
        }
        write!(f, "")
    }
}
