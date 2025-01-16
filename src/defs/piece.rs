use super::Color;

/// The [`PieceType`] enum represents a type of chess piece.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// The [`Piece`] struct represents a chess piece.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Piece {
    pub kind: PieceType,
    pub color: Color,
}

impl Piece {
    /// A white pawn.
    pub const WHITE_PAWN: Piece = Self::new(PieceType::Pawn, Color::White);

    /// A white knight.
    pub const WHITE_KNIGHT: Piece = Self::new(PieceType::Knight, Color::White);

    /// A white bishop.
    pub const WHITE_BISHOP: Piece = Self::new(PieceType::Bishop, Color::White);

    /// A white rook.
    pub const WHITE_ROOK: Piece = Self::new(PieceType::Rook, Color::White);

    /// A white queen.
    pub const WHITE_QUEEN: Piece = Self::new(PieceType::Queen, Color::White);

    /// A white king.
    pub const WHITE_KING: Piece = Self::new(PieceType::King, Color::White);

    /// A black pawn.
    pub const BLACK_PAWN: Piece = Self::new(PieceType::Pawn, Color::Black);

    /// A black knight.
    pub const BLACK_KNIGHT: Piece = Self::new(PieceType::Knight, Color::Black);

    /// A black bishop.
    pub const BLACK_BISHOP: Piece = Self::new(PieceType::Bishop, Color::Black);

    /// A black rook.
    pub const BLACK_ROOK: Piece = Self::new(PieceType::Rook, Color::Black);

    /// A black queen.
    pub const BLACK_QUEEN: Piece = Self::new(PieceType::Queen, Color::Black);

    /// A black king.
    pub const BLACK_KING: Piece = Self::new(PieceType::King, Color::Black);

    /// Creates a new [`Piece`] from a [`PieceType`] and [`Color`].
    #[inline]
    pub const fn new(kind: PieceType, color: Color) -> Self {
        Self { kind, color }
    }

    /// Gets the character representation of the [`Piece`].
    ///
    /// # Examples
    /// ```
    /// use rchess::Piece;
    ///
    /// assert_eq!(Piece::WHITE_PAWN.to_char(), 'P');
    /// assert_eq!(Piece::BLACK_KING.to_char(), 'k');
    /// ```
    #[inline]
    pub const fn to_char(self) -> char {
        match self.color {
            Color::White => self.kind.to_char().to_ascii_uppercase(),
            Color::Black => self.kind.to_char(),
        }
    }

    /// Creates a new [`Piece`] from a given [`char`].
    ///
    /// # Examples
    /// ```
    /// use rchess::Piece;
    ///
    /// assert_eq!(Piece::from_char('p'), Some(Piece::BLACK_PAWN));
    /// assert_eq!(Piece::from_char('K'), Some(Piece::WHITE_KING));
    /// assert_eq!(Piece::from_char('-'), None);
    /// ```
    #[inline]
    pub const fn from_char(c: char) -> Option<Self> {
        let kind = match PieceType::from_char(c) {
            None => return None,
            Some(kind) => kind,
        };
        let color = if c.is_ascii_uppercase() {
            Color::White
        } else {
            Color::Black
        };
        Some(Self { kind, color })
    }
}

impl Into<Piece> for (PieceType, Color) {
    fn into(self) -> Piece {
        Piece::new(self.0, self.1)
    }
}

impl Into<Piece> for (Color, PieceType) {
    fn into(self) -> Piece {
        Piece::new(self.1, self.0)
    }
}
