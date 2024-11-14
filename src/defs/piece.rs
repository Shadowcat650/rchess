/// The [`Piece`] enum represents a chess piece.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    /// Gets the lowercase character representation of the [`Piece`].
    ///
    /// # Examples
    /// ```
    /// use rchess::Piece;
    ///
    /// assert_eq!(Piece::Pawn.to_char(), 'p');
    /// assert_eq!(Piece::King.to_char(), 'k');
    /// ```
    #[inline]
    pub const fn to_char(self) -> char {
        match self {
            Self::Pawn => 'p',
            Self::Knight => 'n',
            Self::Bishop => 'b',
            Self::Rook => 'r',
            Self::Queen => 'q',
            Self::King => 'k',
        }
    }

    /// Creates a new [`Piece`] from a given [`char`].
    ///
    /// # Examples
    /// ```
    /// use rchess::Piece;
    ///
    /// assert_eq!(Piece::from_char('p'), Some(Piece::Pawn));
    /// assert_eq!(Piece::from_char('K'), Some(Piece::King));
    /// assert_eq!(Piece::from_char('-'), None);
    /// ```
    #[inline]
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            'p' | 'P' => Some(Piece::Pawn),
            'n' | 'N' => Some(Piece::Knight),
            'b' | 'B' => Some(Piece::Bishop),
            'r' | 'R' => Some(Piece::Rook),
            'q' | 'Q' => Some(Piece::Queen),
            'k' | 'K' => Some(Piece::King),
            _ => None,
        }
    }

    /// Gets a [`usize`] used to index arrays by the [`Piece`].
    ///
    /// # Examples
    /// ```no_run
    /// use rchess::Piece;
    ///
    /// let mut piece_counts = [0;6];
    /// piece_counts[Piece::Pawn.index()] = 16;
    /// piece_counts[Piece::Knight.index()] = 2;
    /// ```
    #[inline]
    pub const fn index(&self) -> usize {
        *self as usize
    }
}
