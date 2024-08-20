use criterion::{criterion_group, criterion_main, Criterion};
use rchess::{ChessBoard, MoveGen};

pub fn perfts(c: &mut Criterion) {
    let startpos = ChessBoard::new();
    let p2 =
        ChessBoard::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -")
            .unwrap();
    let p3 = ChessBoard::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -").unwrap();
    let p4 = ChessBoard::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq -")
        .unwrap();
    let p4_mirror =
        ChessBoard::from_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ -")
            .unwrap();
    let p5 = ChessBoard::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ -").unwrap();
    let p6 =
        ChessBoard::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - -")
            .unwrap();

    let mut group = c.benchmark_group("Perfts");
    group.bench_function("startpos", |b| {
        b.iter(|| MoveGen::perft(startpos.clone(), 6))
    });
    group.bench_function("p2", |b| b.iter(|| MoveGen::perft(p2.clone(), 4)));
    group.bench_function("p3", |b| b.iter(|| MoveGen::perft(p3.clone(), 6)));
    group.bench_function("p4", |b| b.iter(|| MoveGen::perft(p4.clone(), 5)));
    group.bench_function("p4_mirror", |b| {
        b.iter(|| MoveGen::perft(p4_mirror.clone(), 5))
    });
    group.bench_function("p5", |b| b.iter(|| MoveGen::perft(p5.clone(), 5)));
    group.bench_function("p6", |b| b.iter(|| MoveGen::perft(p6.clone(), 5)));
}

criterion_group!(benches, perfts);
criterion_main!(benches);
