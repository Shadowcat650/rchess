use crate::defs::*;
use crate::table_gen::general::RAYS;
use fastrand::Rng;
use lazy_static::lazy_static;

/// Number of used bits to calculate a bishop magic attack key for a square.
#[rustfmt::skip]
const BISHOP_BITS: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
];

/// Number of used bits to calculate a rook magic attack key for a square.
#[rustfmt::skip]
const ROOK_BITS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

lazy_static! {
    /// The bishop magic for each square.
    pub static ref BISHOP_MAGICS: (Box<[Magic;64]>, usize) = generate_bishop_magics();

    /// The rook magic for each square.
    pub static ref ROOK_MAGICS: (Box<[Magic;64]>, usize) = generate_rook_magics();

    /// The bishop attack magic table.
    pub static ref BISHOP_ATTACKS: Vec<BitBoard> = generate_bishop_attacks();

    /// The rook attack magic table.
    pub static ref ROOK_ATTACKS: Vec<BitBoard> = generate_rook_attacks();
}

fn generate_bishop_magics() -> (Box<[Magic; 64]>, usize) {
    let mut magics = Vec::with_capacity(64);
    let mut current_offset = 0;
    let mut rng = Rng::with_seed(0xDEADBEEF);

    for square in SQUARES {
        let mut attack_table = [BitBoard::EMPTY; 512];

        let mask = get_bishop_mask(square);
        let shift = 64 - BISHOP_BITS[square.index()];
        let occupancy_attacks = Occupancies::new(mask)
            .into_iter()
            .map(|occupancy| (occupancy, get_bishop_attacks_slow(square, occupancy)))
            .collect::<Vec<_>>();

        let mut candidate;
        let mut max_key;
        loop {
            max_key = 0;
            candidate = get_magic_candidate(&mut rng);
            attack_table.fill(BitBoard::EMPTY);

            let mut is_valid = true;
            for (occupancy, attacks) in &occupancy_attacks {
                let key = Magic::calculate_key(*occupancy, mask, candidate, shift);
                max_key = max_key.max(key);

                if attack_table[key].is_empty() {
                    attack_table[key] = *attacks;
                } else if attack_table[key] != *attacks {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                break;
            }
        }

        magics.push(Magic::new(mask, candidate, shift, current_offset));
        current_offset += max_key + 1;
    }

    (Box::new(magics.try_into().unwrap()), current_offset)
}

fn generate_rook_magics() -> (Box<[Magic; 64]>, usize) {
    let mut magics = Vec::with_capacity(64);
    let mut current_offset = 0;
    let mut rng = Rng::with_seed(0xBEEFDEAD);

    for square in SQUARES {
        let mut attack_table = [BitBoard::EMPTY; 4096];

        let shift = 64 - ROOK_BITS[square.index()];
        let mask = get_rook_mask(square);
        let occupancy_attacks = Occupancies::new(mask)
            .into_iter()
            .map(|occupancy| (occupancy, get_rook_attacks_slow(square, occupancy)))
            .collect::<Vec<_>>();

        let mut candidate;
        let mut max_key;
        loop {
            max_key = 0;
            candidate = get_magic_candidate(&mut rng);
            attack_table.fill(BitBoard::EMPTY);

            let mut is_valid = true;
            for (occupancy, attacks) in &occupancy_attacks {
                let key = Magic::calculate_key(*occupancy, mask, candidate, shift);
                max_key = max_key.max(key);

                if attack_table[key].is_empty() {
                    attack_table[key] = *attacks;
                } else if attack_table[key] != *attacks {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                break;
            }
        }

        magics.push(Magic::new(mask, candidate, shift, current_offset));
        current_offset += max_key + 1;
    }

    (Box::new(magics.try_into().unwrap()), current_offset)
}

/// Gets a magic number candidate.
fn get_magic_candidate(rng: &mut Rng) -> u64 {
    rng.u64(0..=u64::MAX) & rng.u64(0..=u64::MAX) & rng.u64(0..=u64::MAX)
}

fn generate_bishop_attacks() -> Vec<BitBoard> {
    let mut bishop_attacks = vec![BitBoard::EMPTY; BISHOP_MAGICS.1];

    for square in SQUARES {
        let blocker_occupancies = Occupancies::new(get_bishop_mask(square));
        for occupancy in blocker_occupancies {
            let attacks = get_bishop_attacks_slow(square, occupancy);
            let key = BISHOP_MAGICS.0[square.index()].key(occupancy);
            bishop_attacks[key] = attacks;
        }
    }

    bishop_attacks
}

fn generate_rook_attacks() -> Vec<BitBoard> {
    let mut rook_attacks = vec![BitBoard::EMPTY; ROOK_MAGICS.1];

    for square in SQUARES {
        let blocker_occupancies = Occupancies::new(get_rook_mask(square));
        for occupancy in blocker_occupancies {
            let attacks = get_rook_attacks_slow(square, occupancy);
            let key = ROOK_MAGICS.0[square.index()].key(occupancy);
            rook_attacks[key] = attacks;
        }
    }

    rook_attacks
}

/// Gets the bishop blocker mask.
fn get_bishop_mask(square: Square) -> BitBoard {
    let mut mask = get_bishop_attacks_slow(square, BitBoard::EMPTY);

    mask &= !BitBoard::from_rank(Rank::First);
    mask &= !BitBoard::from_rank(Rank::Eighth);
    mask &= !BitBoard::from_file(File::A);
    mask &= !BitBoard::from_file(File::H);

    mask
}

/// Gets the rook blocker mask.
fn get_rook_mask(square: Square) -> BitBoard {
    let mut mask = get_rook_attacks_slow(square, BitBoard::EMPTY);

    if square.rank() != Rank::First {
        mask &= !BitBoard::from_rank(Rank::First);
    }

    if square.rank() != Rank::Eighth {
        mask &= !BitBoard::from_rank(Rank::Eighth);
    }

    if square.file() != File::A {
        mask &= !BitBoard::from_file(File::A);
    }

    if square.file() != File::H {
        mask &= !BitBoard::from_file(File::H);
    }

    mask
}

/// Gets bishop seen squares without magic bitboards.
pub(crate) fn get_bishop_attacks_slow(square: Square, occupancy: BitBoard) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    attacks |= ray_attacks(square, Direction::UpLeft, occupancy);
    attacks |= ray_attacks(square, Direction::UpRight, occupancy);
    attacks |= ray_attacks(square, Direction::DownLeft, occupancy);
    attacks |= ray_attacks(square, Direction::DownRight, occupancy);

    attacks
}

/// Gets rook seen squares without magic bitboards.
pub(crate) fn get_rook_attacks_slow(square: Square, occupancy: BitBoard) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    attacks |= ray_attacks(square, Direction::Up, occupancy);
    attacks |= ray_attacks(square, Direction::Down, occupancy);
    attacks |= ray_attacks(square, Direction::Left, occupancy);
    attacks |= ray_attacks(square, Direction::Right, occupancy);

    attacks
}

/// Gets the attacks in a given direction.
fn ray_attacks(square: Square, dir: Direction, occupancy: BitBoard) -> BitBoard {
    let mut attacks = RAYS[square.index()][dir.index()];
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
        attacks ^= RAYS[blocker_square.index()][dir.index()];
    }
    return attacks;
}

/// The [`Occupancies`] struct iterates through all occupancies for a mask.
pub struct Occupancies {
    mask: BitBoard,
    n_combinations: u16,
    current_combination: u16,
}

impl Occupancies {
    /// Creates a new [`Occupancies`] object for a given mask.
    pub fn new(mask: BitBoard) -> Self {
        Self {
            mask,
            n_combinations: 2u16.pow(mask.popcnt() as u32),
            current_combination: 0,
        }
    }
}

impl Iterator for Occupancies {
    type Item = BitBoard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_combination == self.n_combinations {
            return None;
        }
        let mut mask_combination = BitBoard::EMPTY;
        for (idx, sq) in self.mask.into_iter().enumerate() {
            if self.current_combination & (1 << idx as u16) != 0 {
                mask_combination |= sq.bitboard();
            }
        }
        self.current_combination += 1;
        Some(mask_combination)
    }
}
