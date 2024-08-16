use crate::defs::*;
use crate::table_gen::sliders::{get_bishop_attacks_slow, get_rook_attacks_slow};
use lazy_static::lazy_static;

lazy_static! {
    /// A table of rays in each direction.
    pub static ref RAYS: Box<[[BitBoard; 8]; 64]> = generate_rays();

    /// A table of the lines connecting sets of squares.
    pub static ref DIRECT_CONNECTIONS: Box<[[BitBoard; 64]; 64]> = generate_direct_connections();

    /// A table of the axis lines that connect sets of squares.
    pub static ref AXIS_CONNECTIONS: Box<[[BitBoard; 64]; 64]> = generate_axis_connections();
}

/// Generates the rays table.
fn generate_rays() -> Box<[[BitBoard; 8]; 64]> {
    let mut rays = Box::new([[BitBoard::EMPTY; 8]; 64]);

    for square in SQUARES {
        for dir in DIRS {
            let mut pos = square.bitboard();
            while !dir.edge().overlaps(pos) {
                pos = pos.shift_dir(dir);
                rays[square.index()][dir.index()] |= pos;
            }
        }
    }

    rays
}

/// Generates the direct connections table.
fn generate_direct_connections() -> Box<[[BitBoard; 64]; 64]> {
    let mut direct_connections = Box::new([[BitBoard::EMPTY; 64]; 64]);

    for start_sq in SQUARES {
        let start_bb = start_sq.bitboard();
        for end_sq in SQUARES {
            let end_bb = end_sq.bitboard();

            let occupancy = start_bb | end_bb;

            let bishop_attacks = get_bishop_attacks_slow(start_sq, occupancy);
            let rook_attacks = get_rook_attacks_slow(start_sq, occupancy);

            let connection = if bishop_attacks.overlaps(end_bb) {
                let square2_attacks = get_bishop_attacks_slow(end_sq, occupancy);
                bishop_attacks & square2_attacks
            } else if rook_attacks.overlaps(end_bb) {
                let square2_attacks = get_rook_attacks_slow(end_sq, occupancy);
                rook_attacks & square2_attacks
            } else {
                BitBoard::EMPTY
            };

            direct_connections[start_sq.index()][end_sq.index()] = connection;
        }
    }

    direct_connections
}

/// Generates. the axis connections table.
fn generate_axis_connections() -> Box<[[BitBoard; 64]; 64]> {
    let mut axis_connections = Box::new([[BitBoard::EMPTY; 64]; 64]);

    for start_sq in SQUARES {
        let start_bb = start_sq.bitboard();

        for end_sq in SQUARES {
            let end_bb = end_sq.bitboard();

            let bishop_attacks = get_bishop_attacks_slow(start_sq, BitBoard::EMPTY) | start_bb;
            let rook_attacks = get_rook_attacks_slow(start_sq, BitBoard::EMPTY) | start_bb;

            let mut connection = BitBoard::EMPTY;
            if bishop_attacks.overlaps(end_bb) {
                let square2_attacks = get_bishop_attacks_slow(end_sq, BitBoard::EMPTY) | end_bb;
                connection |= bishop_attacks & square2_attacks;
            }

            if rook_attacks.overlaps(end_bb) {
                let square2_attacks = get_rook_attacks_slow(end_sq, BitBoard::EMPTY) | end_bb;
                connection |= rook_attacks & square2_attacks;
            }

            axis_connections[start_sq.index()][end_sq.index()] = connection;
        }
    }

    axis_connections
}
