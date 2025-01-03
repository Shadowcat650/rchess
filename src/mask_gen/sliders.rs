use crate::defs::*;

/// Gets bishop seen squares without magic bitboards.
pub fn get_bishop_attacks_slow(
    rays: &[[BitBoard; 8]; 64],
    square: Square,
    occupancy: BitBoard,
) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    attacks |= ray_attacks(rays, square, Direction::UpLeft, occupancy);
    attacks |= ray_attacks(rays, square, Direction::UpRight, occupancy);
    attacks |= ray_attacks(rays, square, Direction::DownLeft, occupancy);
    attacks |= ray_attacks(rays, square, Direction::DownRight, occupancy);

    attacks
}

/// Gets rook seen squares without magic bitboards.
pub fn get_rook_attacks_slow(
    rays: &[[BitBoard; 8]; 64],
    square: Square,
    occupancy: BitBoard,
) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    attacks |= ray_attacks(rays, square, Direction::Up, occupancy);
    attacks |= ray_attacks(rays, square, Direction::Down, occupancy);
    attacks |= ray_attacks(rays, square, Direction::Left, occupancy);
    attacks |= ray_attacks(rays, square, Direction::Right, occupancy);

    attacks
}

/// Gets the attacks in a given direction.
fn ray_attacks(
    rays: &[[BitBoard; 8]; 64],
    square: Square,
    dir: Direction,
    occupancy: BitBoard,
) -> BitBoard {
    let mut attacks = rays[square.index()][dir.index()];
    let blockers = attacks & occupancy;
    if !blockers.is_empty() {
        let blocker_square = match dir {
            Direction::Up | Direction::Right | Direction::UpLeft | Direction::UpRight => {
                blockers.b_scan_forward().unwrap()
            }
            Direction::Down | Direction::Left | Direction::DownLeft | Direction::DownRight => {
                blockers.b_scan_reverse().unwrap()
            }
        };
        attacks ^= rays[blocker_square.index()][dir.index()];
    }
    attacks
}
