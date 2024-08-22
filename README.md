# RChess

---

A Chess Library Written In Rust

---

RChess is a chess library designed to be used for applications that need to access chess games and or chess boards. For
example, a UCI GUI or an online chess game application.

Note: RChess is still in early development with many features planned. The API may change.

If you find any bugs, or have any suggestions that will make the code easier to read or faster, please feel free create 
a pull request, or and issue. 

For big API changes or new features, please create an issue in the github issues tab.

---

## Examples

```rust
use rchess::ChessGame;

fn main() {
    // Create a new chess game.
    let mut game = ChessGame::new();
    
    // Get the legal moves.
    let moves = game.moves();
    
    // Make a move.
    game.make_move(moves[0]);
}
```

## License

RChess is licensed under the MIT licence.