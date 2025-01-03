mod general;
mod leapers;
mod sliders;
mod zobrist;

use crate::defs::*;
use crate::table_gen::general::{AXIS_CONNECTIONS, DIRECT_CONNECTIONS, RAYS};
use crate::table_gen::leapers::{KING_ATTACKS, KNIGHT_ATTACKS, PAWN_ATTACKS};
use crate::table_gen::sliders::{BISHOP_ATTACKS, BISHOP_MAGICS, ROOK_ATTACKS, ROOK_MAGICS};
use crate::table_gen::zobrist::{
    CASTLE_RIGHTS_ZOBRIST, EN_PASSANT_ZOBRIST, PIECE_ZOBRIST, TURN_ZOBRIST,
};
use std::fs::File;
use std::io::Write;

macro_rules! write_tables {
    ($f:expr, $( $table:ident ),* ) => {
        $(
            write_table($table.as_ref(), stringify!($table), $f);
        )*
    };
}

/// Writes all tables to a file.
pub fn generate_tables(f: &mut File) {
    write_tables!(
        f,
        PAWN_ATTACKS,
        KNIGHT_ATTACKS,
        KING_ATTACKS,
        DIRECT_CONNECTIONS,
        AXIS_CONNECTIONS,
        RAYS
    );

    #[cfg(feature = "magic-table")]
    {
        generate_magic_tables(f);
    }
}

fn generate_magic_tables(f: &mut File) {
    writeln!(f, "const BISHOP_MAGICS: [Magic; 64] = [").unwrap();
    BISHOP_MAGICS.0.iter().for_each(|magic| {
        writeln!(
            f,
            "Magic::new(BitBoard::from_u64({}), {}, {}, {}),",
            magic.mask().to_u64(),
            magic.magic(),
            magic.shift(),
            magic.offset()
        )
        .unwrap()
    });
    writeln!(f, "];").unwrap();

    writeln!(f, "const ROOK_MAGICS: [Magic; 64] = [").unwrap();
    ROOK_MAGICS.0.iter().for_each(|magic| {
        writeln!(
            f,
            "Magic::new(BitBoard::from_u64({}), {}, {}, {}),",
            magic.mask().to_u64(),
            magic.magic(),
            magic.shift(),
            magic.offset()
        )
        .unwrap()
    });
    writeln!(f, "];").unwrap();

    writeln!(
        f,
        "const BISHOP_ATTACKS: [BitBoard; {}] = [",
        BISHOP_ATTACKS.len()
    )
    .unwrap();
    BISHOP_ATTACKS
        .iter()
        .for_each(|bb| writeln!(f, "BitBoard::from_u64({}),", bb.to_u64()).unwrap());
    writeln!(f, "];").unwrap();

    writeln!(
        f,
        "const ROOK_ATTACKS: [BitBoard; {}] = [",
        ROOK_ATTACKS.len()
    )
    .unwrap();
    ROOK_ATTACKS
        .iter()
        .for_each(|bb| writeln!(f, "BitBoard::from_u64({}),", bb.to_u64()).unwrap());
    writeln!(f, "];").unwrap();
}

/// Writes all zobrist numbers to a file.
pub fn generate_zobrist(f: &mut File) {
    write_tables!(f, PIECE_ZOBRIST, CASTLE_RIGHTS_ZOBRIST, EN_PASSANT_ZOBRIST);
    writeln!(f, "const TURN_ZOBRIST: u64 = {};", *TURN_ZOBRIST).unwrap();
}

fn write_table<T: Table>(table: &T, name: &str, f: &mut File) {
    writeln!(f, "const {}: {} = [", name, T::content_type()).unwrap();
    table.write_contents(f);
    writeln!(f, "];").unwrap();
}

trait Table {
    fn content_type() -> String;

    fn write_contents(&self, f: &mut File);
}

impl<const L: usize> Table for [BitBoard; L] {
    fn content_type() -> String {
        format!("[BitBoard; {}]", L)
    }

    fn write_contents(&self, f: &mut File) {
        for bb in self {
            writeln!(f, "BitBoard::from_u64({}),", bb.to_u64()).unwrap();
        }
    }
}

impl<const L: usize> Table for [u64; L] {
    fn content_type() -> String {
        format!("[u64; {}]", L)
    }

    fn write_contents(&self, f: &mut File) {
        for num in self {
            writeln!(f, "{},", num).unwrap();
        }
    }
}

impl<T: Table, const L: usize> Table for [T; L] {
    fn content_type() -> String {
        format!("[{}; {}]", T::content_type(), L)
    }

    fn write_contents(&self, f: &mut File) {
        for i in self {
            writeln!(f, "[").unwrap();
            i.write_contents(f);
            writeln!(f, "],").unwrap();
        }
    }
}
