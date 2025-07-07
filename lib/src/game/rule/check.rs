use crate::{
    game::{Game, mov::GameMov, rule::turn::evaluate_turn},
    piece::Type,
};

pub fn is_in_check(game: &Game) -> bool {
    let curr_turn = evaluate_turn(&game.history);
    for (pos, piece) in game.board.iter() {
        if piece.t == Type::King && piece.color == curr_turn {
            for player in game.players.values() {
                if player.color != curr_turn {
                    let it = player.moves.iter();
                    for (_, moves) in it {
                        for mov in moves {
                            let maybe_pos = match mov {
                                GameMov::Default(m) => Some(&m.mov.to),
                                GameMov::Capture(m) => Some(&m.mov.to),
                                _ => None,
                            };
                            if let Some(menace_pos) = maybe_pos {
                                if menace_pos == pos {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            break;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        color::Color,
        game::{
            Game, board::board_of_str, game::GameBounds, mode::standard_chess, player::GamePlayer,
        },
        mov::Mov,
    };

    use super::is_in_check;

    #[test]
    fn is_in_check_false() {
        let mode = standard_chess();
        assert!(!is_in_check(&Game {
            board: board_of_str(
                &mode.bounds,
                [
                    "    ♚   ",
                    "    ♟   ",
                    "        ",
                    "        ",
                    "        ",
                    "        ",
                    "    ♙   ",
                    "    ♔   ",
                ]
            ),
            bounds: GameBounds { x1: 0, y1: 0, x2: 7, y2: 7 },
            players: HashMap::from([
                (
                    Color::White,
                    GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() },
                ),
                (
                    Color::Black,
                    GamePlayer { color: Color::Black, captures: Vec::new(), moves: HashMap::new() },
                ),
            ]),
            history: Vec::new(),
        }));
    }

    //#[test]
    //fn is_in_check_true() {
    //    let mode = standard_chess();
    //    assert!(is_in_check(&Game {
    //        board: board_of_str(
    //            &mode.bounds,
    //            [
    //                "    ♚   ",
    //                "   ♙♟   ",
    //                "        ",
    //                "        ",
    //                "        ",
    //                "        ",
    //                "        ",
    //                "    ♔   ",
    //            ]
    //        ),
    //        bounds: GameBounds { x1: 0, y1: 0, x2: 7, y2: 7 },
    //        players: HashMap::from([
    //            (
    //                Color::White,
    //                GamePlayer { color: Color::White, captures: Vec::new(), moves: HashMap::new() },
    //            ),
    //            (
    //                Color::Black,
    //                GamePlayer { color: Color::Black, captures: Vec::new(), moves: HashMap::new() },
    //            ),
    //        ]),
    //        history: vec![Mov::of('♙', "D6", "D7")],
    //    }));
    //}
}
