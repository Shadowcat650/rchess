# RChess

A Chess Library Written in Rust

---

**RChess** is a Rust-based library designed for applications that need to work with chess games or boards, such as UCI GUIs or online chess platforms.

> **Note:** RChess is currently under active development, with many more features planned. The API is subject to change.

If you encounter any bugs, have suggestions for improving code readability or performance, or would like to contribute, we encourage you to create a pull request. For significant API changes or feature requests, please open an issue on GitHub.

---

## Getting Started

Documentation for **RChess** can be found [here](https://docs.rs/rchess/1.0.1/rchess/).

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rchess = "2.0.0"
```

### Usage Example

```rust
use rchess::ChessGame;

fn main() {
    // Create a new chess game.
    let mut game = ChessGame::new();
    
    // Get the legal moves for the current position.
    let moves = game.moves();
    
    // Make a move.
    game.make_move(moves[0]);
}
```

---

## Contributing

We welcome contributions! Please adhere to the following guidelines:

- **Bug Reports & Feature Requests:** Open an issue on the [GitHub Issues](https://github.com/Shadowcat650/rchess/issues) page.
- **Code Contributions:** Fork the repository and create a pull request.

---

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/Shadowcat650/rchess/blob/main/LICENSE) file for details.

---
