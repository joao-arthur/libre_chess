use crate::{
    game::{Game, rule::turn::evaluate_turn},
    piece::Type,
};

pub fn is_in_check(play: &Game) -> bool {
    let curr_turn = evaluate_turn(play);
    for (pos, piece) in play.board.iter() {
        if piece.t == Type::King && piece.color == curr_turn {
            for player in play.players.values() {
                if player.color != curr_turn && player.possible_movements.contains(pos) {
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
    use std::collections::{HashMap, HashSet};

    use crate::{
        board::pos::pos_of_str_slice,
        color::Color,
        game::{Game, board::of_str, movement::Movement, player::Player},
    };

    use super::is_in_check;

    #[test]
    fn is_in_check_false() {
        assert_eq!(
            is_in_check(&Game {
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
                players: HashMap::from([
                    (
                        Color::White,
                        Player {
                            color: Color::White,
                            captured_pieces: Vec::new(),
                            possible_movements: pos_of_str_slice([
                                "D1", "D2", "F1", "F2", "E3", "E4"
                            ])
                            .into_iter()
                            .collect(),
                        },
                    ),
                    (
                        Color::Black,
                        Player {
                            color: Color::Black,
                            captured_pieces: Vec::new(),
                            possible_movements: pos_of_str_slice([
                                "D8", "D7", "F8", "F7", "E6", "E5"
                            ])
                            .into_iter()
                            .collect(),
                        },
                    ),
                ]),
                history: Vec::new(),
            }),
            false
        );
    }

    #[test]
    fn is_in_check_true() {
        assert_eq!(
            is_in_check(&Game {
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
                players: HashMap::from([
                    (
                        Color::White,
                        Player {
                            color: Color::White,
                            captured_pieces: Vec::new(),
                            possible_movements: pos_of_str_slice([
                                "D1", "D2", "E2", "F1", "F2", "D8", "E8", "F8"
                            ])
                            .into_iter()
                            .collect(),
                        },
                    ),
                    (
                        Color::Black,
                        Player {
                            color: Color::Black,
                            captured_pieces: Vec::new(),
                            possible_movements: pos_of_str_slice([
                                "D8", "D7", "F8", "F7", "E6", "E5"
                            ])
                            .into_iter()
                            .collect(),
                        },
                    ),
                ]),
                history: vec![Movement::of_str("♙", "D6", "D7")],
            }),
            true
        );
    }
}
