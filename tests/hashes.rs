use rchess::{ChessBoard, MoveGen};

#[test]
fn p1() {
    let fen = ChessBoard::from_fen("rnbq1bnr/p1ppkppp/p7/4p3/4P3/7N/PPPP1PPP/RNBQ1RK1 b - - 0 1")
        .unwrap();
    let moves =
        ChessBoard::from_str_moves(&["e2e4", "e7e5", "f1a6", "b7a6", "g1h3", "e8e7", "e1g1"])
            .unwrap();
    assert_eq!(fen, moves);
    assert_eq!(fen.hash(), moves.hash());
}

#[test]
fn p2() {
    let fen = ChessBoard::from_fen("7N/8/8/8/8/8/k6K/8 b - -").unwrap();
    let mut moves = ChessBoard::from_fen("8/7P/8/8/8/8/k6K/8 w - -").unwrap();
    moves.make_move(MoveGen::create_str_move(&moves, "h7h8n").unwrap());

    assert_eq!(fen, moves);
    assert_eq!(fen.hash(), moves.hash());
}

#[test]
fn p3() {
    let fen = ChessBoard::from_fen("7Q/8/8/8/8/8/k6K/8 b - -").unwrap();
    let mut moves = ChessBoard::from_fen("7r/6P1/8/8/8/8/k6K/8 w - -").unwrap();
    moves.make_move(MoveGen::create_str_move(&moves, "g7h8q").unwrap());

    assert_eq!(fen, moves);
    assert_eq!(fen.hash(), moves.hash());
}
