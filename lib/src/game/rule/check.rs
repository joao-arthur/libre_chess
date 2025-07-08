use crate::{
    game::{Game, mov::GameMov, rule::turn::evaluate_turn},
    piece::Type,
};

pub fn is_in_check(game: &Game) -> bool {
    let curr_turn = evaluate_turn(&game.history);
    let board_it = game.board.iter();
    for (pos, piece) in board_it {
        if piece.t == Type::King && piece.color == curr_turn {
            for player in game.players.values() {
                if player.color != curr_turn {
                    let moves_it = player.moves.iter();
                    for (_, moves) in moves_it {
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
            Game, board::board_of_str, mode::standard_chess, player::GamePlayer,
            rule::allowed_moves::allowed_moves_of_player,
        },
        mov::Mov,
    };

    use super::is_in_check;

    #[test]
    fn is_in_check_false() {
        let mode = standard_chess();
        let bounds = mode.bounds;
        let board = board_of_str(
            &bounds,
            [
                "    ♚   ",
                "    ♟   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "    ♙   ",
                "    ♔   ",
            ],
        );
        let history = Vec::new();
        let players = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: allowed_moves_of_player(&board, &bounds, &history, &Color::Black),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: allowed_moves_of_player(&board, &bounds, &history, &Color::White),
                },
            ),
        ]);
        assert!(!is_in_check(&Game { board, bounds, players, history }));
    }

    #[test]
    fn is_in_check_true() {
        let mode = standard_chess();
        let bounds = mode.bounds;
        let board = board_of_str(
            &bounds,
            [
                "    ♚   ",
                "   ♙♟   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "    ♔   ",
            ],
        );
        let history = vec![Mov::of('♙', "D6", "D7")];
        let players = HashMap::from([
            (
                Color::Black,
                GamePlayer {
                    color: Color::Black,
                    captures: Vec::new(),
                    moves: allowed_moves_of_player(&board, &bounds, &history, &Color::Black),
                },
            ),
            (
                Color::White,
                GamePlayer {
                    color: Color::White,
                    captures: Vec::new(),
                    moves: allowed_moves_of_player(&board, &bounds, &history, &Color::White),
                },
            ),
        ]);
        assert!(is_in_check(&Game { board, bounds, players, history }));
    }
}
