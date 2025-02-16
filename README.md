# rchess

A Chess Library Written in Rust

---

**rchess** is a Rust-based library designed for applications that need to work with chess games or boards, such as UCI GUIs or online chess platforms.

If you encounter any bugs, have suggestions for improving code readability or performance, or would like to contribute, we encourage you to create a pull request. For significant API changes or feature requests, please open an issue on GitHub.

---

## Getting Started

Documentation for **rchess** can be found [here](https://docs.rs/rchess/2.0.0/rchess/).

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rchess = "2.3.0"
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

### Features

By default, no features are enabled.

To use magic bitboards for sliding piece move generation, enable the `magic-table` feature. On my device, this feature speeds up the benches by around 18%.

Build times will be faster without the `magic-table` feature enabled.

Use the `serde` feature to enable serialization and deserialization.

### Improving Build Time
To improve the slow build time when the `magic-table` is enabled, set your build override opt-level to 3.
```toml
[profile.dev.build-override]
opt-level = 3

[profile.test.build-override]
opt-level = 3

[profile.release.build-override]
opt-level = 3
```

---

## Contributing

We welcome contributions! Please adhere to the following guidelines:

- **Bug Reports & Feature Requests:** Open an issue on the [GitHub Issues](https://github.com/Shadowcat650/rchess/issues) page.
- **Code Contributions:** Fork the repository and create a pull request.

---

## License

This project is dual-licensed under the [MIT](https://github.com/Shadowcat650/rchess/blob/main/LICENSE-MIT) and [APACHE 2.0](https://github.com/Shadowcat650/rchess/blob/main/LICENSE-APACHE) licenses.

---
