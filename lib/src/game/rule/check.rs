use crate::{
    game::{Game, rule::turn::evaluate_turn},
    piece::Type,
};

pub fn is_in_check(game: &Game) -> bool {
    let curr_turn = evaluate_turn(game);
    for (pos, piece) in game.board.iter() {
        if piece.t == Type::King && piece.color == curr_turn {
            for player in game.players.values() {
                if player.color != curr_turn && player.menace.contains(pos) {
                    return true;
                }
            }
            break;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, hash::Hash};

    use crate::{
        board::pos::pos_of_str_slice,
        color::Color,
        game::{Game, board::of_str, player::GamePlayer},
        geometry::poligon::rect::RectU8,
        movement::Movement,
    };

    use super::is_in_check;

    #[test]
    fn is_in_check_false() {
        assert!(!is_in_check(&Game {
            board: of_str([
                "    ♚   ",
                "    ♟   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "    ♙   ",
                "    ♔   ",
            ]),
            bounds: RectU8 { x1: 0, y1: 0, x2: 7, y2: 7 },
            players: HashMap::from([
                (
                    Color::White,
                    GamePlayer {
                        color: Color::White,
                        captures: Vec::new(),
                        menace: pos_of_str_slice(["D1", "D2", "F1", "F2", "E3", "E4"])
                            .into_iter()
                            .collect(),
                        moves: HashMap::new()
                    },
                ),
                (
                    Color::Black,
                    GamePlayer {
                        color: Color::Black,
                        captures: Vec::new(),
                        menace: pos_of_str_slice(["D8", "D7", "F8", "F7", "E6", "E5"])
                            .into_iter()
                            .collect(),
                        moves: HashMap::new()
                    },
                ),
            ]),
            history: Vec::new(),
        }));
    }

    #[test]
    fn is_in_check_true() {
        assert!(is_in_check(&Game {
            board: of_str([
                "    ♚   ",
                "   ♙♟   ",
                "        ",
                "        ",
                "        ",
                "        ",
                "        ",
                "    ♔   ",
            ]),
            bounds: RectU8 { x1: 0, y1: 0, x2: 7, y2: 7 },
            players: HashMap::from([
                (
                    Color::White,
                    GamePlayer {
                        color: Color::White,
                        captures: Vec::new(),
                        menace: pos_of_str_slice(["D1", "D2", "E2", "F1", "F2", "D8", "E8", "F8"])
                            .into_iter()
                            .collect(),
                        moves: HashMap::new(),
                    },
                ),
                (
                    Color::Black,
                    GamePlayer {
                        color: Color::Black,
                        captures: Vec::new(),
                        menace: pos_of_str_slice(["D8", "D7", "F8", "F7", "E6", "E5"])
                            .into_iter()
                            .collect(),
                        moves: HashMap::new(),
                    },
                ),
            ]),
            history: vec![Movement::of_str("♙", "D6", "D7")],
        }));
    }
}
