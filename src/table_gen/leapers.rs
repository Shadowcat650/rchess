use crate::defs::*;
use lazy_static::lazy_static;

lazy_static! {
    /// The pawn attack table.
    pub static ref PAWN_ATTACKS: Box<[[BitBoard;64];2]> = generate_pawn_attacks();

    /// The knight attack table.
    pub static ref KNIGHT_ATTACKS: Box<[BitBoard;64]> = generate_knight_attacks();

    /// The king attack table.
    pub static ref KING_ATTACKS: Box<[BitBoard;64]> = generate_king_attacks();
}

/// Generates the pawn attack table.
fn generate_pawn_attacks() -> Box<[[BitBoard; 64]; 2]> {
    let mut pawn_attacks = Box::new([[BitBoard::EMPTY; 64]; 2]);

    for square in SQUARES {
        // Generate white pawn attacks.
        if square.file() != File::A && square.rank() != Rank::Eighth {
            pawn_attacks[Color::White.index()][square.index()] |= square.bitboard().up().left();
        }

        if square.file() != File::H && square.rank() != Rank::Eighth {
            pawn_attacks[Color::White.index()][square.index()] |= square.bitboard().up().right();
        }

        // Generate black pawn attacks.
        if square.file() != File::A && square.rank() != Rank::First {
            pawn_attacks[Color::Black.index()][square.index()] |= square.bitboard().down().left();
        }

        if square.file() != File::H && square.rank() != Rank::First {
            pawn_attacks[Color::Black.index()][square.index()] |= square.bitboard().down().right();
        }
    }

    pawn_attacks
}

/// Generates the knight attack table.
fn generate_knight_attacks() -> Box<[BitBoard; 64]> {
    let mut knight_attacks = Box::new([BitBoard::EMPTY; 64]);

    for square in SQUARES {
        if square.rank() != Rank::Eighth && square.file() > File::B {
            knight_attacks[square.index()] |= square.bitboard().up().left().left();
        }

        if square.rank() != Rank::Eighth && square.file() < File::G {
            knight_attacks[square.index()] |= square.bitboard().up().right().right();
        }

        if square.rank() != Rank::First && square.file() > File::B {
            knight_attacks[square.index()] |= square.bitboard().down().left().left();
        }

        if square.rank() != Rank::First && square.file() < File::G {
            knight_attacks[square.index()] |= square.bitboard().down().right().right();
        }

        if square.file() != File::A && square.rank() < Rank::Seventh {
            knight_attacks[square.index()] |= square.bitboard().left().up().up();
        }

        if square.file() != File::A && square.rank() > Rank::Second {
            knight_attacks[square.index()] |= square.bitboard().left().down().down();
        }

        if square.file() != File::H && square.rank() < Rank::Seventh {
            knight_attacks[square.index()] |= square.bitboard().right().up().up();
        }

        if square.file() != File::H && square.rank() > Rank::Second {
            knight_attacks[square.index()] |= square.bitboard().right().down().down();
        }
    }

    knight_attacks
}

/// Generates the king attack table.
fn generate_king_attacks() -> Box<[BitBoard; 64]> {
    let mut king_attacks = Box::new([BitBoard::EMPTY; 64]);

    for square in SQUARES {
        if square.rank() != Rank::First {
            king_attacks[square.index()] |= square.bitboard().down();
        }

        if square.rank() != Rank::Eighth {
            king_attacks[square.index()] |= square.bitboard().up();
        }

        if square.file() != File::A {
            king_attacks[square.index()] |= square.bitboard().left();
        }

        if square.file() != File::H {
            king_attacks[square.index()] |= square.bitboard().right();
        }

        if square.rank() != Rank::First && square.file() != File::A {
            king_attacks[square.index()] |= square.bitboard().down().left();
        }

        if square.rank() != Rank::First && square.file() != File::H {
            king_attacks[square.index()] |= square.bitboard().down().right();
        }

        if square.rank() != Rank::Eighth && square.file() != File::A {
            king_attacks[square.index()] |= square.bitboard().up().left();
        }

        if square.rank() != Rank::Eighth && square.file() != File::H {
            king_attacks[square.index()] |= square.bitboard().up().right();
        }
    }

    king_attacks
}
