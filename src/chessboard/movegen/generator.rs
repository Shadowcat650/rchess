use super::movelist::{MoveList, PieceMoves};
use crate::chessboard::tables::{
    get_bishop_attacks, get_connection_axis, get_direct_connection, get_king_attacks,
    get_knight_attacks, get_pawn_attacks, get_rook_attacks,
};
use crate::chessboard::ChessBoard;
use crate::defs::*;

/// Get a [`MoveList`] of moves for a chessboard.
pub fn generate_moves<const CAPTURES_ONLY: bool>(chessboard: &ChessBoard) -> MoveList {
    let mut moves = MoveList::new();

    if chessboard.checkers().is_empty() {
        // The king is not in check.
        generate_pawn_moves::<CAPTURES_ONLY, false>(&mut moves, chessboard);
        generate_knight_moves::<CAPTURES_ONLY, false>(&mut moves, chessboard);
        generate_king_moves::<CAPTURES_ONLY, false>(&mut moves, chessboard);
        generate_bishop_moves::<CAPTURES_ONLY, false>(&mut moves, chessboard);
        generate_rook_moves::<CAPTURES_ONLY, false>(&mut moves, chessboard);
        generate_queen_moves::<CAPTURES_ONLY, false>(&mut moves, chessboard);
    } else if chessboard.checkers().popcnt() == 1 {
        // The king is in check by one piece.
        generate_pawn_moves::<CAPTURES_ONLY, true>(&mut moves, chessboard);
        generate_knight_moves::<CAPTURES_ONLY, true>(&mut moves, chessboard);
        generate_king_moves::<CAPTURES_ONLY, true>(&mut moves, chessboard);
        generate_bishop_moves::<CAPTURES_ONLY, true>(&mut moves, chessboard);
        generate_rook_moves::<CAPTURES_ONLY, true>(&mut moves, chessboard);
        generate_queen_moves::<CAPTURES_ONLY, true>(&mut moves, chessboard);
    } else {
        // The king is checked by multiple pieces.
        generate_king_moves::<false, true>(&mut moves, chessboard);
    }

    moves
}

/// Gets a [`BitBoard`] of moves for the piece on the given [`Square`].
///
/// If there was no piece on the square, or it is not that piece's turn, an empty [`BitBoard`] is returned.
pub fn generate_square_moves<const CAPTURES_ONLY: bool>(
    chessboard: &ChessBoard,
    square: Square,
) -> BitBoard {
    let piece = match chessboard.piece_at(square) {
        None => return BitBoard::EMPTY,
        Some(piece) => piece,
    };

    if piece.color != chessboard.turn() {
        return BitBoard::EMPTY;
    }

    if chessboard.checkers().popcnt() > 1 {
        if piece.kind != PieceType::King {
            return BitBoard::EMPTY;
        }
        return generate_king_attacks::<false, true>(
            chessboard,
            chessboard.get_king_square(chessboard.turn()),
        );
    }

    if chessboard.checkers().is_empty() {
        match piece.kind {
            PieceType::Pawn => generate_pawn_attacks::<CAPTURES_ONLY, false>(chessboard, square),
            PieceType::Knight => {
                generate_knight_attacks::<CAPTURES_ONLY, false, false>(chessboard, square)
            }
            PieceType::Bishop => {
                generate_bishop_attacks::<CAPTURES_ONLY, false>(chessboard, square)
            }
            PieceType::Rook => generate_rook_attacks::<CAPTURES_ONLY, false>(chessboard, square),
            PieceType::Queen => generate_queen_attacks::<CAPTURES_ONLY, false>(chessboard, square),
            PieceType::King => generate_king_attacks::<CAPTURES_ONLY, false>(chessboard, square),
        }
    } else {
        match piece.kind {
            PieceType::Pawn => generate_pawn_attacks::<CAPTURES_ONLY, true>(chessboard, square),
            PieceType::Knight => {
                generate_knight_attacks::<CAPTURES_ONLY, true, false>(chessboard, square)
            }
            PieceType::Bishop => generate_bishop_attacks::<CAPTURES_ONLY, true>(chessboard, square),
            PieceType::Rook => generate_rook_attacks::<CAPTURES_ONLY, true>(chessboard, square),
            PieceType::Queen => generate_queen_attacks::<CAPTURES_ONLY, true>(chessboard, square),
            PieceType::King => generate_king_attacks::<CAPTURES_ONLY, true>(chessboard, square),
        }
    }
}

/// Generates the pawn moves for a given [`ChessBoard`].
fn generate_pawn_moves<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    moves: &mut MoveList,
    chessboard: &ChessBoard,
) {
    // Get extra data about the chess board.
    let us = chessboard.turn();

    // The pawns that can potentially move.
    let pawns = chessboard.query((PieceType::Pawn, us));
    for pawn_sq in pawns {
        // Get the pawn attacks.
        let attacks = generate_pawn_attacks::<CAPTURES_ONLY, IN_CHECK>(chessboard, pawn_sq);

        // Add pawn moves to the move list.
        moves.push(PieceMoves::new(pawn_sq, attacks));
    }
}

/// Generates the pawn attacks for a given square.
fn generate_pawn_attacks<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    chessboard: &ChessBoard,
    square: Square,
) -> BitBoard {
    // Get extra data about the chess board.
    let us = chessboard.turn();
    let them = !chessboard.turn();
    let king_sq = chessboard.get_king_square(us);

    let en_passant_bb = match chessboard.en_passant_sq() {
        None => BitBoard::EMPTY,
        Some(sq) => sq.bitboard(),
    };

    let start_rank = match chessboard.turn() {
        Color::White => Rank::Second,
        Color::Black => Rank::Seventh,
    };

    let mut targets = BitBoard::EMPTY;

    // Look for pins.
    let (is_pinned, is_pinned_forward) = if square.bitboard().overlaps(chessboard.pinned()) {
        (true, king_sq.file() == square.file())
    } else {
        (false, false)
    };

    // Generate pawn non-captures.
    if !CAPTURES_ONLY || IN_CHECK {
        if !is_pinned || is_pinned_forward {
            let forward = match us {
                Color::White => square.bitboard().up(),
                Color::Black => square.bitboard().down(),
            };

            if !forward.overlaps(chessboard.occupancy()) {
                targets |= forward;
                if square.rank() == start_rank {
                    let double = match us {
                        Color::White => forward.up(),
                        Color::Black => forward.down(),
                    };

                    if !double.overlaps(chessboard.occupancy()) {
                        targets |= double;
                    }
                }
            }
        }
    }

    let mut en_passant_move = BitBoard::EMPTY;
    if !is_pinned_forward {
        // What the pawn attacks.
        let attacks = if is_pinned {
            get_pawn_attacks(square, us) & get_connection_axis(king_sq, square)
        } else {
            get_pawn_attacks(square, us)
        };

        // Add normal captures to targets.
        targets |= attacks & chessboard.color_occupancy(them);

        // Handle en passant.
        if attacks.overlaps(en_passant_bb) {
            let en_passanted = match us {
                Color::White => en_passant_bb.down(),
                Color::Black => en_passant_bb.up(),
            };

            let new_occupancy =
                chessboard.occupancy() ^ (square.bitboard() | en_passant_bb | en_passanted);

            let bishop_check_spots = get_bishop_attacks(king_sq, new_occupancy);
            let enemy_bishops = chessboard.query((PieceType::Bishop, them))
                | chessboard.query((PieceType::Queen, them));

            let rook_check_spots = get_rook_attacks(king_sq, new_occupancy);
            let enemy_rooks = chessboard.query((PieceType::Rook, them))
                | chessboard.query((PieceType::Queen, them));

            if !enemy_rooks.overlaps(rook_check_spots)
                && !enemy_bishops.overlaps(bishop_check_spots)
            {
                // King is not in check after en passant,
                en_passant_move = en_passant_bb;
            }
        }
    }

    if IN_CHECK {
        // We must defend the king.
        let king_sq = chessboard.get_king_square(us);
        let checker_sq = chessboard.checkers().b_scan_forward().unwrap();
        let defending = get_direct_connection(king_sq, checker_sq) | chessboard.checkers();
        targets &= defending;
    }

    targets | en_passant_move
}

/// Generates the knight moves for a given [`ChessBoard`].
fn generate_knight_moves<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    moves: &mut MoveList,
    chessboard: &ChessBoard,
) {
    // Get extra data about the chess board.
    let us = chessboard.turn();

    // The knights that can potentially move.
    let knights = chessboard.query((PieceType::Knight, us)) & !chessboard.pinned();

    // Generate moves for each unpinned knight.
    for knight_sq in knights {
        let attacks =
            generate_knight_attacks::<CAPTURES_ONLY, IN_CHECK, true>(chessboard, knight_sq);

        // Add knight moves to the move list.
        moves.push(PieceMoves::new(knight_sq, attacks));
    }
}

/// Generates the knight attacks for a given square.
fn generate_knight_attacks<
    const CAPTURES_ONLY: bool,
    const IN_CHECK: bool,
    const CHECKED_PIN: bool,
>(
    chessboard: &ChessBoard,
    square: Square,
) -> BitBoard {
    // Get extra data about the chess board.
    let us = chessboard.turn();
    let them = !chessboard.turn();

    if !CHECKED_PIN {
        if chessboard.pinned().contains(square) {
            return BitBoard::EMPTY;
        }
    }

    // Get the squares the knight attacks.
    let attacks = get_knight_attacks(square);

    // Remove the squares the knight cannot attack.
    let attacks = if IN_CHECK {
        // We must defend the king.
        let king_sq = chessboard.get_king_square(us);
        let checker_sq = chessboard.checkers().b_scan_forward().unwrap();
        let defending = get_direct_connection(king_sq, checker_sq) | chessboard.checkers();
        attacks & defending
    } else {
        if CAPTURES_ONLY {
            // We have to capture an enemy piece.
            attacks & chessboard.color_occupancy(them)
        } else {
            // We can go anywhere except for where our friendly pieces are located.
            let our_pieces = chessboard.color_occupancy(us);
            attacks & !our_pieces
        }
    };

    attacks
}

/// Generates the king moves for a given [`ChessBoard`].
fn generate_king_moves<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    moves: &mut MoveList,
    chessboard: &ChessBoard,
) {
    // Get extra data about the chess board.
    let us = chessboard.turn();

    // Get the king square.
    let king_sq = chessboard.get_king_square(us);

    // Generate the king attacks.
    let attacks = generate_king_attacks::<CAPTURES_ONLY, IN_CHECK>(chessboard, king_sq);

    // King moves to the move list.
    moves.push(PieceMoves::new(king_sq, attacks));
}

/// Generates the king attacks for a given square.
fn generate_king_attacks<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    chessboard: &ChessBoard,
    square: Square,
) -> BitBoard {
    // Get extra data about the chess board.
    let us = chessboard.turn();
    let them = !chessboard.turn();

    // The squares attacked by the king.
    let attacks = get_king_attacks(square);

    // Make sure we only attack non-friendly pieces and follow capture only rule.
    let mut attacks = if CAPTURES_ONLY && !IN_CHECK {
        // We have to capture an enemy piece.
        attacks & chessboard.color_occupancy(them)
    } else {
        // We can move anywhere but where our friendly pieces are.
        let our_pieces = chessboard.color_occupancy(us);
        attacks & !our_pieces
    };

    // Remove checked squares from king attacks.
    for target in attacks {
        let no_king_occupancy = chessboard.occupancy() ^ square.bitboard();
        if is_square_attacked_with_occupancy(target, no_king_occupancy, them, chessboard) {
            attacks ^= target.bitboard();
        }
    }

    // Add castle moves to king targets.
    if !CAPTURES_ONLY {
        attacks |= generate_castle_moves::<IN_CHECK>(chessboard);
    }

    attacks
}

/// Generates the bishop moves for a given [`ChessBoard`].
fn generate_bishop_moves<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    moves: &mut MoveList,
    chessboard: &ChessBoard,
) {
    // Get extra data about the chess board.
    let us = chessboard.turn();

    // Get the bishops that can potentially move.
    let bishops = chessboard.query((PieceType::Bishop, us));

    for bishop_sq in bishops {
        // Generate the bishop attacks.
        let attacks = generate_bishop_attacks::<CAPTURES_ONLY, IN_CHECK>(chessboard, bishop_sq);

        // Add bishop moves to the move list.
        moves.push(PieceMoves::new(bishop_sq, attacks));
    }
}

/// Generates the bishop attacks for a given square.
fn generate_bishop_attacks<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    chessboard: &ChessBoard,
    square: Square,
) -> BitBoard {
    // Get extra data about the chess board.
    let us = chessboard.turn();
    let them = !chessboard.turn();

    // Get the squares attacked by the bishop.
    let attacks = get_bishop_attacks(square, chessboard.occupancy());

    // Remove squares that the bishop cannot attack.
    let attacks = if IN_CHECK {
        // We must defend the king.
        let king_sq = chessboard.get_king_square(us);
        let checker_sq = chessboard.checkers().b_scan_forward().unwrap();
        let defending = get_direct_connection(king_sq, checker_sq) | chessboard.checkers();

        if square.bitboard().overlaps(chessboard.pinned()) {
            // We are also pinned :(
            let pinned_axis = get_connection_axis(king_sq, square);
            attacks & defending & pinned_axis
        } else {
            attacks & defending
        }
    } else {
        if CAPTURES_ONLY {
            if square.bitboard().overlaps(chessboard.pinned()) {
                // We are pinned.
                let king_sq = chessboard.get_king_square(us);
                let pinned_axis = get_connection_axis(king_sq, square);
                attacks & chessboard.color_occupancy(them) & pinned_axis
            } else {
                attacks & chessboard.color_occupancy(them)
            }
        } else {
            let our_pieces = chessboard.color_occupancy(us);
            if square.bitboard().overlaps(chessboard.pinned()) {
                // We are pinned.
                let king_sq = chessboard.get_king_square(us);
                let pinned_axis = get_connection_axis(king_sq, square);
                attacks & !our_pieces & pinned_axis
            } else {
                attacks & !our_pieces
            }
        }
    };

    attacks
}

/// Generates the rook moves for a given [`ChessBoard`].
fn generate_rook_moves<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    moves: &mut MoveList,
    chessboard: &ChessBoard,
) {
    // Get extra data about the chess board.
    let us = chessboard.turn();

    // Get the rooks that can potentially move.
    let rooks = chessboard.query((PieceType::Rook, us));

    for rook_sq in rooks {
        // Get the rook attacks.
        let attacks = generate_rook_attacks::<CAPTURES_ONLY, IN_CHECK>(chessboard, rook_sq);

        // Add rook moves to the move list.
        moves.push(PieceMoves::new(rook_sq, attacks));
    }
}

/// Generates the rook moves for a given square.
fn generate_rook_attacks<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    chessboard: &ChessBoard,
    square: Square,
) -> BitBoard {
    // Get extra data about the chess board.
    let us = chessboard.turn();
    let them = !chessboard.turn();

    // Get the squares attacked by the rook.
    let attacks = get_rook_attacks(square, chessboard.occupancy());

    // Remove squares that the bishop cannot attack.
    let attacks = if IN_CHECK {
        // We must defend the king.
        let king_sq = chessboard.get_king_square(us);
        let checker_sq = chessboard.checkers().b_scan_forward().unwrap();
        let defending = get_direct_connection(king_sq, checker_sq) | chessboard.checkers();

        if square.bitboard().overlaps(chessboard.pinned()) {
            // We are also pinned :(
            let pinned_axis = get_connection_axis(king_sq, square);
            attacks & defending & pinned_axis
        } else {
            attacks & defending
        }
    } else {
        if CAPTURES_ONLY {
            if square.bitboard().overlaps(chessboard.pinned()) {
                // We are pinned.
                let king_sq = chessboard.get_king_square(us);
                let pinned_axis = get_connection_axis(king_sq, square);
                attacks & chessboard.color_occupancy(them) & pinned_axis
            } else {
                attacks & chessboard.color_occupancy(them)
            }
        } else {
            let our_pieces = chessboard.color_occupancy(us);
            if square.bitboard().overlaps(chessboard.pinned()) {
                // We are pinned.
                let king_sq = chessboard.get_king_square(us);
                let pinned_axis = get_connection_axis(king_sq, square);
                attacks & !our_pieces & pinned_axis
            } else {
                attacks & !our_pieces
            }
        }
    };

    attacks
}

/// Generates the queen moves for a given [`ChessBoard`].
fn generate_queen_moves<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    moves: &mut MoveList,
    chessboard: &ChessBoard,
) {
    // Get extra data about the chess board.
    let us = chessboard.turn();

    // Get the queens that can potentially move.
    let queens = chessboard.query((PieceType::Queen, us));

    for queen_sq in queens {
        // Generate the queen attacks.
        let attacks = generate_queen_attacks::<CAPTURES_ONLY, IN_CHECK>(chessboard, queen_sq);

        // Add queen moves to the move list.
        moves.push(PieceMoves::new(queen_sq, attacks));
    }
}

/// Generates the queen attacks for a given square.
fn generate_queen_attacks<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    chessboard: &ChessBoard,
    square: Square,
) -> BitBoard {
    generate_bishop_attacks::<CAPTURES_ONLY, IN_CHECK>(chessboard, square)
        | generate_rook_attacks::<CAPTURES_ONLY, IN_CHECK>(chessboard, square)
}

/// Generates the castle moves for a given [`ChessBoard`].
fn generate_castle_moves<const IN_CHECK: bool>(chessboard: &ChessBoard) -> BitBoard {
    // We can't castle while in check.
    if IN_CHECK {
        return BitBoard::EMPTY;
    }

    // Get extra data about the chess board.
    let us = chessboard.turn();
    let them = !chessboard.turn();

    // Where the castle moves are stored.
    let mut castles = BitBoard::EMPTY;

    // Look for kingside castle
    if chessboard.is_castle_right_set(CastleSide::Kingside, us) {
        // The squares that must be empty for the king to castle.
        let empty_squares = match chessboard.turn() {
            Color::White => BitBoard::from_squares(&[Square::F1, Square::G1]),
            Color::Black => BitBoard::from_squares(&[Square::F8, Square::G8]),
        };

        // Make sure the empty squares are empty.
        if !empty_squares.overlaps(chessboard.occupancy()) {
            // Make sure the king does not travel through a check
            let mut checked = false;
            for square in empty_squares {
                if is_square_attacked(square, them, chessboard) {
                    checked = true;
                    break;
                }
            }

            // If the king is not checked, it can castle.
            if !checked {
                // Where the king end up while castling.
                let castle_target = match chessboard.turn() {
                    Color::White => BitBoard::from_square(Square::G1),
                    Color::Black => BitBoard::from_square(Square::G8),
                };

                // Add the castle target to the castle targets.
                castles |= castle_target;
            }
        }
    }

    // Look for queenside castle
    if chessboard.is_castle_right_set(CastleSide::Queenside, us) {
        // The squares that must un checked for the king to castle.
        let un_checked = match chessboard.turn() {
            Color::White => BitBoard::from_squares(&[Square::C1, Square::D1]),
            Color::Black => BitBoard::from_squares(&[Square::C8, Square::D8]),
        };

        // The squares that must be empty for the king to castle.
        let empty_squares = match chessboard.turn() {
            Color::White => BitBoard::from_squares(&[Square::B1, Square::C1, Square::D1]),
            Color::Black => BitBoard::from_squares(&[Square::B8, Square::C8, Square::D8]),
        };

        // Make sure the empty squares are empty.
        if !empty_squares.overlaps(chessboard.occupancy()) {
            // Make sure the king does not travel through a check
            let mut checked = false;
            for square in un_checked {
                if is_square_attacked(square, them, chessboard) {
                    checked = true;
                    break;
                }
            }

            // If the king is not checked, it can castle.
            if !checked {
                // Where the king end up while castling.
                let castle_target = match chessboard.turn() {
                    Color::Black => BitBoard::from_square(Square::C8),
                    Color::White => BitBoard::from_square(Square::C1),
                };

                // Add the castle target to the castle targets.
                castles |= castle_target;
            }
        }
    }

    castles
}

/// Checks if a square is under attack by a given color.
fn is_square_attacked(square: Square, by: Color, chessboard: &ChessBoard) -> bool {
    is_square_attacked_with_occupancy(square, chessboard.occupancy(), by, chessboard)
}

/// Checks if a square is under attack by a given color with a given occupancy.
fn is_square_attacked_with_occupancy(
    square: Square,
    occupancy: BitBoard,
    by: Color,
    chessboard: &ChessBoard,
) -> bool {
    // Look for pawn attackers.
    let pawn_check_locations = get_pawn_attacks(square, !by);
    if !(chessboard.query((PieceType::Pawn, by)) & pawn_check_locations).is_empty() {
        return true;
    };

    // Look for knight attackers.
    let knight_check_locations = get_knight_attacks(square);
    if !(chessboard.query((PieceType::Knight, by)) & knight_check_locations).is_empty() {
        return true;
    };

    // Look for king attackers.
    let king_check_locations = get_king_attacks(square);
    if !(chessboard.query((PieceType::King, by)) & king_check_locations).is_empty() {
        return true;
    };

    // Look for bishop & queen attackers.
    let bishop_check_locations = get_bishop_attacks(square, occupancy);
    if !((chessboard.query((PieceType::Bishop, by)) | chessboard.query((PieceType::Queen, by)))
        & bishop_check_locations)
        .is_empty()
    {
        return true;
    };

    // Look for rook & queen attackers.
    let rook_check_locations = get_rook_attacks(square, occupancy);
    if !((chessboard.query((PieceType::Rook, by)) | chessboard.query((PieceType::Queen, by)))
        & rook_check_locations)
        .is_empty()
    {
        return true;
    };

    // The square is safe since no enemy pieces were found attacking it.
    false
}
