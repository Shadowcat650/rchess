use rchess::File::C;
use rchess::{BoardBuilder, ChessBoard, Color, Piece, Square};

#[test]
fn start_pos() {
    let board = ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -");
    assert!(board.is_ok());
}

#[test]
fn double_insert() {
    let board = BoardBuilder::new()
        .piece(Square::A1, Piece::Rook, Color::Black)
        .unwrap()
        .piece(Square::A1, Piece::Knight, Color::Black); // oops
    assert!(board.is_err());
}

#[test]
fn two_kings() {
    let board = ChessBoard::from_fen("k1k5/8/8/8/8/8/8/K7 w - -");
    assert!(board.is_err());
}

#[test]
fn missing_king() {
    let board = ChessBoard::from_fen("rnbq1bnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1BNR w - - 0 1");
    assert!(board.is_err());
}

#[test]
fn missing_turn() {
    let board = ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR - KQkq -");
    assert!(board.is_err());
}

#[test]
fn bad_en_passant_sq() {
    let board =
        ChessBoard::from_fen("rnbqkbnr/1ppppppp/p7/4P3/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 1");
    assert!(board.is_err());
}

#[test]
fn incorrect_en_passant_sq() {
    let board =
        ChessBoard::from_fen("rnbqkbnr/1pppp1pp/p7/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f5 0 1");
    assert!(board.is_err());
}

#[test]
fn bad_wk_castle_right() {
    let board = ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBN1 w KQkq -");
    assert!(board.is_err());
}

#[test]
fn bad_wq_castle_right() {
    let board = ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/1NBQKBNR w KQkq -");
    assert!(board.is_err());
}

#[test]
fn bad_bk_castle_right() {
    let board = ChessBoard::from_fen("rnbqkbn1/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -");
    assert!(board.is_err());
}

#[test]
fn bad_bq_castle_right() {
    let board = ChessBoard::from_fen("1nbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -");
    assert!(board.is_err());
}

#[test]
fn can_capture_king() {
    let board =
        ChessBoard::from_fen("rnb1kbnr/1ppp1ppp/p3p3/8/6Pq/3P1P2/PPP1P2P/RNBQKBNR b KQkq - 0 1");
    assert!(board.is_err());
}

#[test]
fn touching_kings() {
    let board = ChessBoard::from_fen("8/8/4k3/3K4/8/8/8/8 w - -");
    assert!(board.is_err());
}

#[test]
fn pawn_on_last_w() {
    let board = ChessBoard::from_fen("1P2k3/8/8/8/8/8/8/4K3 b - - 0 1");
    assert!(board.is_err());
}

#[test]
fn pawn_on_last_b() {
    let board = ChessBoard::from_fen("4k3/8/8/8/8/8/8/4K1p1 b - - 0 1");
    assert!(board.is_err());
}

#[test]
fn from_moves_err() {
    let board =
        ChessBoard::from_str_moves(&["e2e3", "d7d6", "e1e2", "c8g4", "g1f3", "g4h5", "f3e1"]);
    assert!(board.is_err());
}

#[test]
fn halfmoves() {
    let board = ChessBoard::from_fen("7k/8/1r6/8/8/6R1/8/K7 w - - 50").unwrap();
    assert_eq!(board.halfmoves(), 50);
}

#[test]
fn invalid_halfmoves() {
    let board = ChessBoard::from_fen("7k/8/1r6/8/8/6R1/8/K7 w - - 101");
    assert!(board.is_err());
}
