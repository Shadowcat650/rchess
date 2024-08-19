use rchess::{ChessBoard, MoveGen};

#[test]
fn startpos() {
    let board = ChessBoard::new();
    let nodes = MoveGen::perft(board, 6);
    assert_eq!(nodes, 119_060_324);
}

#[test]
fn p2() {
    let board = ChessBoard::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -").unwrap();
    let nodes = MoveGen::perft(board, 4);
    assert_eq!(nodes, 4_085_603);
}

#[test]
fn p3() {
    let board = ChessBoard::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -").unwrap();
    let nodes = MoveGen::perft(board, 6);
    assert_eq!(nodes, 11_030_083);
}

#[test]
fn p4() {
    let board = ChessBoard::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq -").unwrap();
    let nodes = MoveGen::perft(board, 5);
    assert_eq!(nodes, 15_833_292);
}

#[test]
fn p4_mirror() {
    let board = ChessBoard::from_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ -").unwrap();
    let nodes = MoveGen::perft(board, 5);
    assert_eq!(nodes, 15_833_292);
}

#[test]
fn p5() {
    let board = ChessBoard::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ -").unwrap();
    let nodes = MoveGen::perft(board, 5);
    assert_eq!(nodes, 89_941_194);
}

#[test]
fn p6() {
    let board = ChessBoard::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - -").unwrap();
    let nodes = MoveGen::perft(board, 5);
    assert_eq!(nodes, 164_075_551);
}