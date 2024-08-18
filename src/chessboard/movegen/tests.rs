use crate::chessboard::movegen::MoveGen;
use crate::chessboard::tables;
use crate::chessboard::ChessBoard;
use std::thread;

#[test]
fn test_perft() {
    let chessboard = ChessBoard::new();
    let p_startpos = thread::spawn(move || MoveGen::perft(chessboard, 6));

    let p_2_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let chessboard = ChessBoard::from_fen(p_2_fen).unwrap();
    let p_2 = thread::spawn(move || MoveGen::perft(chessboard, 4));

    let p_3_fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
    let chessboard = ChessBoard::from_fen(p_3_fen).unwrap();
    let p_3 = thread::spawn(move || MoveGen::perft(chessboard, 6));

    let p_4_fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq -";
    let chessboard = ChessBoard::from_fen(p_4_fen).unwrap();
    let p_4 = thread::spawn(move || MoveGen::perft(chessboard, 5));

    let p_4_mirror_fen = "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ -";
    let chessboard = ChessBoard::from_fen(p_4_mirror_fen).unwrap();
    let p_4_mirror = thread::spawn(move || MoveGen::perft(chessboard, 5));

    let p_5_fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    let chessboard = ChessBoard::from_fen(p_5_fen).unwrap();
    let p_5 = thread::spawn(move || MoveGen::perft(chessboard, 5));

    let p_6_fen = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - -";
    let chessboard = ChessBoard::from_fen(p_6_fen).unwrap();
    let p_6 = thread::spawn(move || MoveGen::perft(chessboard, 5));

    assert_eq!(p_startpos.join().unwrap(), 119_060_324, "position startpos");
    assert_eq!(p_2.join().unwrap(), 4_085_603, "position fen {}", p_2_fen);
    assert_eq!(p_3.join().unwrap(), 11_030_083, "position fen {}", p_3_fen);
    assert_eq!(p_4.join().unwrap(), 15_833_292, "position fen {}", p_4_fen);
    assert_eq!(
        p_4_mirror.join().unwrap(),
        15_833_292,
        "position fen {}",
        p_4_mirror_fen
    );
    assert_eq!(p_5.join().unwrap(), 89_941_194, "position fen {}", p_5_fen);
    assert_eq!(p_6.join().unwrap(), 164_075_551, "position fen {}", p_6_fen);
}
