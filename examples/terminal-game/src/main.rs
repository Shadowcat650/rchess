mod game_controller;
mod input_getter;

use rchess::{BitBoard, ChessBoard, MoveGen};
use crate::game_controller::GameController;

fn main() {
    GameController::new().run();
}
