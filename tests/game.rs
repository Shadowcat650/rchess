use rchess::{ChessGame, Color, DrawReason, GameResult, Rank, Square};

#[test]
fn repetition() {
    let mut game = ChessGame::new();
    for mv in [
        "g1f3", "b8a6", "f3g1", "a6b8", "g1f3", "b8a6", "f3g1", "a6b8",
    ] {
        let mv = game.create_str_move(mv).unwrap();
        game.make_move(mv).unwrap();
    }
    assert_eq!(
        game.result(),
        Some(GameResult::Draw {
            reason: DrawReason::ThreefoldRepetition
        })
    );
}

#[test]
fn stalemate() {
    let game = ChessGame::from_fen("1r5k/8/8/8/8/8/7r/K7 w - -").unwrap();
    assert_eq!(
        game.result(),
        Some(GameResult::Draw {
            reason: DrawReason::Stalemate
        })
    );
}

#[test]
fn checkmate() {
    let mut game = ChessGame::new();
    for mv in ["f2f3", "e7e6", "g2g4", "d8h4"] {
        assert!(game.result().is_none());
        let mv = game.create_str_move(mv).unwrap();
        game.make_move(mv).unwrap();
    }
    assert_eq!(game.result(), Some(GameResult::BlackWins));
}

#[test]
fn k_v_k() {
    let game = ChessGame::from_fen("7k/8/8/8/8/8/8/K7 w - -").unwrap();
    assert_eq!(
        game.result(),
        Some(GameResult::Draw {
            reason: DrawReason::InsufficientMaterial
        })
    );
}

#[test]
fn kn_v_k() {
    let game = ChessGame::from_fen("7k/8/8/8/8/2N5/8/K7 w - -").unwrap();
    assert_eq!(
        game.result(),
        Some(GameResult::Draw {
            reason: DrawReason::InsufficientMaterial
        })
    );
}

#[test]
fn k_v_kn() {
    let game = ChessGame::from_fen("7k/8/8/8/8/2n5/8/K7 w - -").unwrap();
    assert_eq!(
        game.result(),
        Some(GameResult::Draw {
            reason: DrawReason::InsufficientMaterial
        })
    );
}

#[test]
fn k_v_kb() {
    let game = ChessGame::from_fen("7k/8/b7/8/8/8/8/K7 w - -").unwrap();
    assert_eq!(
        game.result(),
        Some(GameResult::Draw {
            reason: DrawReason::InsufficientMaterial
        })
    );
}

#[test]
fn kb_v_k() {
    let game = ChessGame::from_fen("7k/8/B7/8/8/8/8/K7 w - -").unwrap();
    assert_eq!(
        game.result(),
        Some(GameResult::Draw {
            reason: DrawReason::InsufficientMaterial
        })
    );
}

#[test]
fn kb_v_kb_draw() {
    let game = ChessGame::from_fen("7k/8/B7/5b2/8/8/8/K7 w - -").unwrap();
    assert_eq!(
        game.result(),
        Some(GameResult::Draw {
            reason: DrawReason::InsufficientMaterial
        })
    );
}

#[test]
fn kb_v_kb_in_play() {
    let game = ChessGame::from_fen("7k/8/B7/b7/8/8/8/K7 w - -").unwrap();
    assert!(game.result().is_none());
}

#[test]
fn kn_v_kn() {
    let game = ChessGame::from_fen("7k/8/n7/N7/8/8/8/K7 w - -").unwrap();
    assert!(game.result().is_none());
}

#[test]
fn insufficient_material() {
    let mut game = ChessGame::from_fen("3k4/PK6/8/8/8/8/8/8 w - -").unwrap();
    let mv = game.create_str_move("a7a8n").unwrap();
    game.make_move(mv).unwrap();
    assert_eq!(
        game.result(),
        Some(GameResult::Draw {
            reason: DrawReason::InsufficientMaterial
        })
    );
}
