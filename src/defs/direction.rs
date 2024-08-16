use super::{BitBoard, File, Rank};

/// All the directions.
pub const DIRS: [Direction; 8] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownLeft,
    Direction::DownRight,
];

/// The [`Direction`] enum represents a direction on the chess board.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    /// Gets a [`usize`] that used to index arrays by the [`Direction`].
    ///
    /// # Examples
    /// ```no_run
    /// use rchess::Direction;
    ///
    /// let mut direction_counts = [0;8];
    /// direction_counts[Direction::Up.index()] += 1;
    /// direction_counts[Direction::DownRight.index()] += 1;
    /// ```
    pub const fn index(&self) -> usize {
        *self as usize
    }

    /// Gets a [`BitBoard`] containing the squares of the edge a [`Direction`] will eventually hit.
    ///
    /// # Examples
    /// ```no_run
    /// use rchess::{Direction, Square, BitBoard};
    ///
    /// let mut loc = BitBoard::from_square(Square::E5);
    /// while !loc.overlaps(Direction::Up.edge()) {
    ///     // Do something until we hit the top edge.
    ///     loc.shift_dir(Direction::Up);
    /// }
    /// ```
    pub const fn edge(&self) -> BitBoard {
        match self {
            Direction::Up => BitBoard::from_rank(Rank::Eighth),
            Direction::Down => BitBoard::from_rank(Rank::First),
            Direction::Left => BitBoard::from_file(File::A),
            Direction::Right => BitBoard::from_file(File::H),
            Direction::UpLeft => BitBoard::from_rank(Rank::Eighth).or(BitBoard::from_file(File::A)),
            Direction::UpRight => {
                BitBoard::from_rank(Rank::Eighth).or(BitBoard::from_file(File::H))
            }
            Direction::DownLeft => {
                BitBoard::from_rank(Rank::First).or(BitBoard::from_file(File::A))
            }
            Direction::DownRight => {
                BitBoard::from_rank(Rank::First).or(BitBoard::from_file(File::H))
            }
        }
    }
}
