## TODO

- [ ] Special moves
    - [-] En passant
    - [ ] Check
    - [-] Castling
    - [ ] Promotion

- [ ] Game result
    - [ ] Stalemate
    - [ ] Checkmate
    - [ ] Draw
    - [ ] Resign

- [ ] Stalemate by no capture
- [ ] Stalemate by repetition

- [ ] Game modes
    - [ ] 960 chess
    - [ ] Alice Chess

## Fix

- [ ] game_of_mode_and_history
    - [ ] Captures

- [ ] allowed_moves_of_player
    - [ ] King menaces

- [ ] Selection
    - [ ] use it in web_backend

## Pseudocode

- init will init the board, history and players

- player will 'move_piece'
    - move the pieces in the board
    - add the move to history
    - evaluate players moves

- the app will handle the player moves by verifying the current turn
