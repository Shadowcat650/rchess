use super::movelist::{MoveList, PieceMoves};
use crate::chessboard::ChessBoard;
use crate::defs::*;
use crate::chessboard::tables::{
    get_bishop_attacks, get_connection_axis, get_direct_connection, get_king_attacks,
    get_knight_attacks, get_pawn_attacks, get_rook_attacks,
};

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
    } else if chessboard.checkers().popcnt() == 1 {
        // The king is in check by one piece.
        generate_pawn_moves::<CAPTURES_ONLY, true>(&mut moves, chessboard);
        generate_knight_moves::<CAPTURES_ONLY, true>(&mut moves, chessboard);
        generate_king_moves::<CAPTURES_ONLY, true>(&mut moves, chessboard);
        generate_bishop_moves::<CAPTURES_ONLY, true>(&mut moves, chessboard);
        generate_rook_moves::<CAPTURES_ONLY, true>(&mut moves, chessboard);
    } else {
        // The king is checked by multiple pieces.
        generate_king_moves::<CAPTURES_ONLY, true>(&mut moves, chessboard);
    }

    moves
}

/// Generates the pawn moves for a given [`ChessBoard`].
fn generate_pawn_moves<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    moves: &mut MoveList,
    chess_board: &ChessBoard,
) {
    // Get extra data about the chess board.
    let us = chess_board.turn();
    let them = !chess_board.turn();
    let king_sq = chess_board.get_king_square(us);

    let en_passant_bb = match chess_board.en_passant_sq() {
        None => BitBoard::EMPTY,
        Some(sq) => sq.bitboard(),
    };

    let start_rank = match chess_board.turn() {
        Color::White => Rank::Second,
        Color::Black => Rank::Seventh,
    };

    // The pawns that can potentially move.
    let pawns = chess_board.query(Piece::Pawn, us);
    for pawn_sq in pawns {
        let mut targets = BitBoard::EMPTY;

        // Look for pins.
        let (is_pinned, is_pinned_forward) = if pawn_sq.bitboard().overlaps(chess_board.pinned()) {
            (true, king_sq.file() == pawn_sq.file())
        } else {
            (false, false)
        };

        // Generate pawn non-captures.
        if !CAPTURES_ONLY || IN_CHECK {
            if !is_pinned || is_pinned_forward {
                let forward = match us {
                    Color::White => pawn_sq.bitboard().up(),
                    Color::Black => pawn_sq.bitboard().down(),
                };

                if !forward.overlaps(chess_board.occupancy()) {
                    targets |= forward;
                    if pawn_sq.rank() == start_rank {
                        let double = match us {
                            Color::White => forward.up(),
                            Color::Black => forward.down(),
                        };

                        if !double.overlaps(chess_board.occupancy()) {
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
                get_pawn_attacks(pawn_sq, us) & get_connection_axis(king_sq, pawn_sq)
            } else {
                get_pawn_attacks(pawn_sq, us)
            };

            // Add normal captures to targets.
            targets |= attacks & chess_board.color_occupancy(them);

            // Handle en passant.
            if attacks.overlaps(en_passant_bb) {
                let en_passanted = match us {
                    Color::White => en_passant_bb.down(),
                    Color::Black => en_passant_bb.up(),
                };

                let new_occupancy =
                    chess_board.occupancy() ^ (pawn_sq.bitboard() | en_passant_bb | en_passanted);

                let bishop_check_spots = get_bishop_attacks(king_sq, new_occupancy);
                let enemy_bishops =
                    chess_board.query(Piece::Bishop, them) | chess_board.query(Piece::Queen, them);

                let rook_check_spots = get_rook_attacks(king_sq, new_occupancy);
                let enemy_rooks =
                    chess_board.query(Piece::Rook, them) | chess_board.query(Piece::Queen, them);

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
            let king_sq = chess_board.get_king_square(us);
            let checker_sq = chess_board.checkers().b_scan_forward().unwrap();
            let defending = get_direct_connection(king_sq, checker_sq) | chess_board.checkers();
            targets &= defending;
        }

        // Add pawn moves to the move list.
        moves.push(PieceMoves::new(pawn_sq, targets | en_passant_move));
    }
}

/// Generates the knight moves for a given [`ChessBoard`].
fn generate_knight_moves<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    moves: &mut MoveList,
    chess_board: &ChessBoard,
) {
    // Get extra data about the chess board.
    let us = chess_board.turn();
    let them = !chess_board.turn();

    // The knights that can potentially move.
    let knights = chess_board.query(Piece::Knight, us) & !chess_board.pinned();

    // Generate moves for each unpinned knight.
    for knight_sq in knights {
        // Get the squares the knight attacks.
        let attacks = get_knight_attacks(knight_sq);

        // Remove the squares the knight cannot attack.
        let attacks = if IN_CHECK {
            // We must defend the king.
            let king_sq = chess_board.get_king_square(us);
            let checker_sq = chess_board.checkers().b_scan_forward().unwrap();
            let defending = get_direct_connection(king_sq, checker_sq) | chess_board.checkers();
            attacks & defending
        } else {
            if CAPTURES_ONLY {
                // We have to capture an enemy piece.
                attacks & chess_board.color_occupancy(them)
            } else {
                // We can go anywhere except for where our friendly pieces are located.
                let our_pieces = chess_board.color_occupancy(us);
                attacks & !our_pieces
            }
        };

        // Add knight moves to the move list.
        moves.push(PieceMoves::new(knight_sq, attacks));
    }
}

/// Generates the king moves for a given [`ChessBoard`].
fn generate_king_moves<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    moves: &mut MoveList,
    chess_board: &ChessBoard,
) {
    // Get extra data about the chess board.
    let us = chess_board.turn();
    let them = !chess_board.turn();

    // Get the king square.
    let king_sq = chess_board.get_king_square(us);

    // The squares attacked by the king.
    let attacks = get_king_attacks(king_sq);

    // Make sure we only attack non-friendly pieces and follow capture only rule.
    let mut attacks = if CAPTURES_ONLY && !IN_CHECK {
        // We have to capture an enemy piece.
        attacks & chess_board.color_occupancy(them)
    } else {
        // We can move anywhere but where our friendly pieces are.
        let our_pieces = chess_board.color_occupancy(us);
        attacks & !our_pieces
    };

    // Remove checked squares from king attacks.
    for square in attacks {
        let no_king_occupancy = chess_board.occupancy() ^ king_sq.bitboard();
        if is_square_attacked_with_occupancy(square, no_king_occupancy, them, chess_board) {
            attacks ^= square.bitboard();
        }
    }

    // Add castle moves to king targets.
    if !CAPTURES_ONLY {
        attacks |= generate_castle_moves::<IN_CHECK>(chess_board);
    }

    // King moves to the move list.
    moves.push(PieceMoves::new(king_sq, attacks));
}

/// Generates the bishop moves for a given [`ChessBoard`].
fn generate_bishop_moves<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    moves: &mut MoveList,
    chess_board: &ChessBoard,
) {
    // Get extra data about the chess board.
    let us = chess_board.turn();
    let them = !chess_board.turn();

    // Get the bishops (and queens) that can potentially move.
    let bishops = chess_board.query(Piece::Bishop, us) | chess_board.query(Piece::Queen, us);

    for bishop_sq in bishops {
        // Get the squares attacked by the bishop.
        let attacks = get_bishop_attacks(bishop_sq, chess_board.occupancy());

        // Remove squares that the bishop cannot attack.
        let attacks = if IN_CHECK {
            // We must defend the king.
            let king_sq = chess_board.get_king_square(us);
            let checker_sq = chess_board.checkers().b_scan_forward().unwrap();
            let defending = get_direct_connection(king_sq, checker_sq) | chess_board.checkers();

            if bishop_sq.bitboard().overlaps(chess_board.pinned()) {
                // We are also pinned :(
                let pinned_axis = get_connection_axis(king_sq, bishop_sq);
                attacks & defending & pinned_axis
            } else {
                attacks & defending
            }
        } else {
            if CAPTURES_ONLY {
                if bishop_sq.bitboard().overlaps(chess_board.pinned()) {
                    // We are pinned.
                    let king_sq = chess_board.get_king_square(us);
                    let pinned_axis = get_connection_axis(king_sq, bishop_sq);
                    attacks & chess_board.color_occupancy(them) & pinned_axis
                } else {
                    attacks & chess_board.color_occupancy(them)
                }
            } else {
                let our_pieces = chess_board.color_occupancy(us);
                if bishop_sq.bitboard().overlaps(chess_board.pinned()) {
                    // We are pinned.
                    let king_sq = chess_board.get_king_square(us);
                    let pinned_axis = get_connection_axis(king_sq, bishop_sq);
                    attacks & !our_pieces & pinned_axis
                } else {
                    attacks & !our_pieces
                }
            }
        };

        // Add bishop moves to the move list.
        moves.push(PieceMoves::new(bishop_sq, attacks));
    }
}

fn generate_rook_moves<const CAPTURES_ONLY: bool, const IN_CHECK: bool>(
    moves: &mut MoveList,
    chess_board: &ChessBoard,
) {
    // Get extra data about the chess board.
    let us = chess_board.turn();
    let them = !chess_board.turn();

    // Get the rooks (and queens) that can potentially move.
    let rooks = chess_board.query(Piece::Rook, us) | chess_board.query(Piece::Queen, us);

    for rook_sq in rooks {
        // Get the squares attacked by the rook.
        let attacks = get_rook_attacks(rook_sq, chess_board.occupancy());

        // Remove squares that the bishop cannot attack.
        let attacks = if IN_CHECK {
            // We must defend the king.
            let king_sq = chess_board.get_king_square(us);
            let checker_sq = chess_board.checkers().b_scan_forward().unwrap();
            let defending = get_direct_connection(king_sq, checker_sq) | chess_board.checkers();

            if rook_sq.bitboard().overlaps(chess_board.pinned()) {
                // We are also pinned :(
                let pinned_axis = get_connection_axis(king_sq, rook_sq);
                attacks & defending & pinned_axis
            } else {
                attacks & defending
            }
        } else {
            if CAPTURES_ONLY {
                if rook_sq.bitboard().overlaps(chess_board.pinned()) {
                    // We are pinned.
                    let king_sq = chess_board.get_king_square(us);
                    let pinned_axis = get_connection_axis(king_sq, rook_sq);
                    attacks & chess_board.color_occupancy(them) & pinned_axis
                } else {
                    attacks & chess_board.color_occupancy(them)
                }
            } else {
                let our_pieces = chess_board.color_occupancy(us);
                if rook_sq.bitboard().overlaps(chess_board.pinned()) {
                    // We are pinned.
                    let king_sq = chess_board.get_king_square(us);
                    let pinned_axis = get_connection_axis(king_sq, rook_sq);
                    attacks & !our_pieces & pinned_axis
                } else {
                    attacks & !our_pieces
                }
            }
        };

        // Add rook moves to the move list.
        moves.push(PieceMoves::new(rook_sq, attacks));
    }
}

/// Generates the castle moves for a given [`ChessBoard`].
fn generate_castle_moves<const IN_CHECK: bool>(chess_board: &ChessBoard) -> BitBoard {
    // We can't castle while in check.
    if IN_CHECK {
        return BitBoard::EMPTY;
    }

    // Get extra data about the chess board.
    let us = chess_board.turn();
    let them = !chess_board.turn();

    // Where the castle moves are stored.
    let mut castles = BitBoard::EMPTY;

    // Look for kingside castle
    if chess_board.is_castle_right_set(CastleSide::Kingside, us) {
        // The squares that must be empty for the king to castle.
        let empty_squares = match chess_board.turn() {
            Color::White => BitBoard::from_squares(&[Square::F1, Square::G1]),
            Color::Black => BitBoard::from_squares(&[Square::F8, Square::G8]),
        };

        // Make sure the empty squares are empty.
        if !empty_squares.overlaps(chess_board.occupancy()) {
            // Make sure the king does not travel through a check
            let mut checked = false;
            for square in empty_squares {
                if is_square_attacked(square, them, chess_board) {
                    checked = true;
                    break;
                }
            }

            // If the king is not checked, it can castle.
            if !checked {
                // Where the king end up while castling.
                let castle_target = match chess_board.turn() {
                    Color::White => BitBoard::from_square(Square::G1),
                    Color::Black => BitBoard::from_square(Square::G8),
                };

                // Add the castle target to the castle targets.
                castles |= castle_target;
            }
        }
    }

    // Look for queenside castle
    if chess_board.is_castle_right_set(CastleSide::Queenside, us) {
        // The squares that must un checked for the king to castle.
        let un_checked = match chess_board.turn() {
            Color::White => BitBoard::from_squares(&[Square::C1, Square::D1]),
            Color::Black => BitBoard::from_squares(&[Square::C8, Square::D8]),
        };

        // The squares that must be empty for the king to castle.
        let empty_squares = match chess_board.turn() {
            Color::White => BitBoard::from_squares(&[Square::B1, Square::C1, Square::D1]),
            Color::Black => BitBoard::from_squares(&[Square::B8, Square::C8, Square::D8]),
        };

        // Make sure the empty squares are empty.
        if !empty_squares.overlaps(chess_board.occupancy()) {
            // Make sure the king does not travel through a check
            let mut checked = false;
            for square in un_checked {
                if is_square_attacked(square, them, chess_board) {
                    checked = true;
                    break;
                }
            }

            // If the king is not checked, it can castle.
            if !checked {
                // Where the king end up while castling.
                let castle_target = match chess_board.turn() {
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
fn is_square_attacked(square: Square, by: Color, chess_board: &ChessBoard) -> bool {
    is_square_attacked_with_occupancy(square, chess_board.occupancy(), by, chess_board)
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
    if !(chessboard.query(Piece::Pawn, by) & pawn_check_locations).is_empty() {
        return true;
    };

    // Look for knight attackers.
    let knight_check_locations = get_knight_attacks(square);
    if !(chessboard.query(Piece::Knight, by) & knight_check_locations).is_empty() {
        return true;
    };

    // Look for king attackers.
    let king_check_locations = get_king_attacks(square);
    if !(chessboard.query(Piece::King, by) & king_check_locations).is_empty() {
        return true;
    };

    // Look for bishop & queen attackers.
    let bishop_check_locations = get_bishop_attacks(square, occupancy);
    if !((chessboard.query(Piece::Bishop, by) | chessboard.query(Piece::Queen, by))
        & bishop_check_locations)
        .is_empty()
    {
        return true;
    };

    // Look for rook & queen attackers.
    let rook_check_locations = get_rook_attacks(square, occupancy);
    if !((chessboard.query(Piece::Rook, by) | chessboard.query(Piece::Queen, by))
        & rook_check_locations)
        .is_empty()
    {
        return true;
    };

    // The square is safe since no enemy pieces were found attacking it.
    false
}
