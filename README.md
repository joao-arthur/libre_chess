# libre_chess

A **FLOSS** implementation of _Chess_.

## Architecture

### lib

A **reusable**, **generic** implementation of _Chess_ in _Rust_.

### web_backend

A _Web Assembly_ application that works as a bridge between _lib_ and _web_frontend_, using **wasm-pack**.

### web_frontend

A minimal user application, responsible for render the canvas and init the _Web Assembly_, using **NextJS**.

## TODO

## Movements

- [x] Rook
- [x] Knight
- [x] Bishop
- [x] Queen
- [x] King
    - [ ] Can't put king under check
    - [ ] Castling
- [x] Pawn
    - [ ] Promotion
    - [ ] En passant

## Special conditions

- [ ] Check
- [ ] Check mate

## Game end

- [ ] Resign
- [ ] Draw
    - [ ] 50 moves rule
    - [ ] 3 repetions
- [ ] Stalemate
- [ ] Dead position

## Run it

```sh
cd ./web_backend
sh ./build.sh
cd ../web_frontend
pnpm install
pnpm dev
```
