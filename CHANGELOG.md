# Changelog

- Convert return result of `ChessGame::moves` & `ChessGame::made_moves` to `&[Move]`

### 2.3.0
- Add `serde` feature 

## 2.2.1
- Update package metadata

## 2.2.0
- Add `MoveGen::piece_captures` and `MoveGen::piece_legal` which get a move bitboard for a single piece

## 2.1.0
- Add feature flag for magic bitboard move generation

## 2.0.0
- Implement `Hash` for many types
- Add new `Piece` type to make API easier to use

### Breaking
- Rename `Piece` to `PieceType`

## 1.0.1
- Initial Release
