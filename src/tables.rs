use crate::defs::*;

include!(concat!(env!("OUT_DIR"), "/generated_tables.rs"));

/// Returns a bitboard mask containing the squares under attack by a pawn from a given square
pub fn get_pawn_attacks(square: Square, color: Color) -> BitBoard {
    PAWN_ATTACKS[color.index()][square.index()]
}

/// Returns a bitboard mask containing all the squares a knight targets from a given square
pub fn get_knight_attacks(square: Square) -> BitBoard {
    KNIGHT_ATTACKS[square.index()]
}

/// Returns a bitboard mask containing all the squares a king targets from a given square
pub fn get_king_attacks(square: Square) -> BitBoard {
    KING_ATTACKS[square.index()]
}

/// Returns a bitboard with the connecting line between two squares.
pub fn get_direct_connection(start: Square, end: Square) -> BitBoard {
    DIRECT_CONNECTIONS[start.index()][end.index()]
}

/// Returns a bitboard with axis line intersecting two squares.
pub fn get_connection_axis(start: Square, end: Square) -> BitBoard {
    AXIS_CONNECTIONS[start.index()][end.index()]
}

/// Gets a `BitBoard` of the squares a bishop attacks with a given square and occupancy.
pub fn get_bishop_attacks(square: Square, occupancy: BitBoard) -> BitBoard {
    let key = BISHOP_MAGICS[square.index()].key(occupancy);
    BISHOP_ATTACKS[key]
}

/// Gets a `BitBoard` of the squares a rook attacks with a given square and occupancy.
pub fn get_rook_attacks(square: Square, occupancy: BitBoard) -> BitBoard {
    let key = ROOK_MAGICS[square.index()].key(occupancy);
    ROOK_ATTACKS[key]
}

/// Returns the seen squares for a bishop ignoring the first friendly blocker
pub fn get_ghost_bishop(square: Square, occupancy: BitBoard, mut friendly: BitBoard) -> BitBoard {
    let bishop_seen = get_bishop_attacks(square, occupancy);

    // Get seen friends
    friendly &= bishop_seen;

    // Return all bishop seen squares
    bishop_seen ^ get_bishop_attacks(square, occupancy ^ friendly)
}

/// Returns the seen squares for a rook ignoring the first friendly blocker
pub fn get_ghost_rook(square: Square, occupancy: BitBoard, mut friendly: BitBoard) -> BitBoard {
    let rook_seen = get_rook_attacks(square, occupancy);

    // Get seen friends
    friendly &= rook_seen;

    // Return all bishop seen squares
    rook_seen ^ get_rook_attacks(square, occupancy ^ friendly)
}
