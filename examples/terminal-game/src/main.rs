mod game_controller;
mod input_getter;

use rchess::BitBoard;
use crate::game_controller::GameController;

fn main() {
    GameController::new().run();
}
