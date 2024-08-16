use fastrand::Rng;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PIECE_ZOBRIST: Box<[[[u64; 64]; 6]; 2]> = generate_piece_zobrist();
    pub static ref CASTLE_RIGHTS_ZOBRIST: Box<[[u64; 2]; 2]> = generate_castle_right_zobrist();
    pub static ref EN_PASSANT_ZOBRIST: Box<[u64; 8]> = generate_en_passant_zobrist();
    pub static ref TURN_ZOBRIST: u64 = generate_turn_zobrist();
}

/// Generates piece zobrist random numbers.
fn generate_piece_zobrist() -> Box<[[[u64; 64]; 6]; 2]> {
    let mut piece_zobrist = Box::new([[[0; 64]; 6]; 2]);
    let mut rng = Rng::with_seed(123456);

    piece_zobrist.iter_mut().for_each(|j| {
        j.iter_mut()
            .for_each(|k| k.iter_mut().for_each(|val| *val = rng.u64(0..=u64::MAX)))
    });

    piece_zobrist
}

/// Generates castle right zobrist random numbers.
fn generate_castle_right_zobrist() -> Box<[[u64; 2]; 2]> {
    let mut castle_right_zobrist = Box::new([[0; 2]; 2]);
    let mut rng = Rng::with_seed(654321);

    castle_right_zobrist
        .iter_mut()
        .for_each(|i| i.iter_mut().for_each(|val| *val = rng.u64(0..=u64::MAX)));

    castle_right_zobrist
}

/// Generates en passant zobrist random numbers.
fn generate_en_passant_zobrist() -> Box<[u64; 8]> {
    let mut en_passant_zobrist = Box::new([0; 8]);
    let mut rng = Rng::with_seed(7890123);

    en_passant_zobrist
        .iter_mut()
        .for_each(|val| *val = rng.u64(0..=u64::MAX));

    en_passant_zobrist
}

/// Generates turn zobrist random number.
fn generate_turn_zobrist() -> u64 {
    let mut rng = Rng::with_seed(3210987);
    rng.u64(0..=u64::MAX)
}
