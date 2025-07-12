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

- [ ] Remove Selection.selected_piece_moves
- [ ] { from: Pos, to: Pos, mov: GameMovOld } 






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



```js
{
    board: Map<
        { row: u8, col: u8 },
        { t: Type, color: Color }
    >,
    bounds: { x1: u8, y1: u8, x2: u8, y2: u8 },
    players: Map<
        Color,
        {
            color: Color,
            captures: Vec<GameCapture>,
            moves: HashMap<Pos, Vec<GameMovOld>>
        }
    >,
    history: Vec<
        {
            piece: Piece,
            from: Pos,
            to: Pos,
        }
    >,
}
```

hoje o gamemov carrega toda a informação, qual a peça, de onde para onde...

mas eu não preciso disso tudo...

no selection eu vou ter a posição selecionada (origem)
o usuário vai selecionar a posição de destino (to)










// remover comentários do código e jogar pra cá