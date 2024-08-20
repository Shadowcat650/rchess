use rchess::ChessBoard;

#[test]
fn p1() {
    let fen = ChessBoard::from_fen("rnbq1bnr/p1ppkppp/p7/4p3/4P3/7N/PPPP1PPP/RNBQ1RK1 b - - 0 1")
        .unwrap();
    let mut moves =
        ChessBoard::from_str_moves(&["e2e4", "e7e5", "f1a6", "b7a6", "g1h3", "e8e7", "e1g1"])
            .unwrap();
    assert_eq!(fen, moves);
    assert_eq!(fen.hash(), moves.hash());
}
