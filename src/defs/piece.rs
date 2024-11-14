/// The [`PieceType`] enum represents a type of chess piece.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    /// Gets the lowercase character representation of the [`PieceType`].
    ///
    /// # Examples
    /// ```
    /// use rchess::PieceType;
    ///
    /// assert_eq!(PieceType::Pawn.to_char(), 'p');
    /// assert_eq!(PieceType::King.to_char(), 'k');
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

    /// Creates a new [`PieceType`] from a given [`char`].
    ///
    /// # Examples
    /// ```
    /// use rchess::PieceType;
    ///
    /// assert_eq!(PieceType::from_char('p'), Some(PieceType::Pawn));
    /// assert_eq!(PieceType::from_char('K'), Some(PieceType::King));
    /// assert_eq!(PieceType::from_char('-'), None);
    /// ```
    #[inline]
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            'p' | 'P' => Some(PieceType::Pawn),
            'n' | 'N' => Some(PieceType::Knight),
            'b' | 'B' => Some(PieceType::Bishop),
            'r' | 'R' => Some(PieceType::Rook),
            'q' | 'Q' => Some(PieceType::Queen),
            'k' | 'K' => Some(PieceType::King),
            _ => None,
        }
    }

    /// Gets a [`usize`] used to index arrays by the [`PieceType`].
    ///
    /// # Examples
    /// ```no_run
    /// use rchess::PieceType;
    ///
    /// let mut piece_counts = [0;6];
    /// piece_counts[PieceType::Pawn.index()] = 16;
    /// piece_counts[PieceType::Knight.index()] = 2;
    /// ```
    #[inline]
    pub const fn index(&self) -> usize {
        *self as usize
    }
}
