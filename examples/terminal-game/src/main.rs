mod game_controller;
mod input_getter;

use crate::game_controller::GameController;

fn main() {
    GameController::new().run();
}