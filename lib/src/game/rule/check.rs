use crate::{game::{rule::turn::get_turn, Game}, piece::Type};



pub fn is_in_check(play: &Game) -> bool {
    let curr_turn = get_turn(play);
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

    #[test]
    fn is_in_check_false() {
    // assert_eq!(is_in_check(
    //     &Game {
    //         board: game::board::of_str([
    //             "    ♚   ",
    //             "    ♟   ",
    //             "        ",
    //             "        ",
    //             "        ",
    //             "    ♖   ",
    //             "        ",
    //             "    ♔   ",
    //         ]),
    //         players: HashMap::from([
    //             (
    //                 Color::White,
    //                 Player {
    //                     color: Color::White,
    //                     captured_pieces: Vec::new(),
    //                     possible_movements: HashSet::new(),
    //                 },
    //             ),
    //             (
    //                 Color::Black,
    //                 Player {
    //                     color: Color::Black,
    //                     captured_pieces: Vec::new(),
    //                     possible_movements: HashSet::from([
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                         Pos::of_str(""),
    //                     ]),
    //                 },
    //             ),
    //         ]),
    //         history: Vec::new(),
    //     }
    // ), false);
       }

    #[test]
      fn is_in_check_true() {
    // assert_eq!(is_in_check(
    //     &Game {
    //         board: game::board::of_str([
    //             "    ♚   ",
    //             "   ♙ ♟  ",
    //             "        ",
    //             "        ",
    //             "        ",
    //             "        ",
    //             "        ",
    //             "    ♔   ",
    //         ]),
    //         players: HashMap::from([
    //             (
    //                 Color::White,
    //                 Player {
    //                     color: Color::White,
    //                     captured_pieces: Vec::new(),
    //                     possible_movements: HashSet::new(),
    //                 },
    //             ),
    //             (
    //                 Color::Black,
    //                 Player {
    //                     color: Color::Black,
    //                     captured_pieces: Vec::new(),
    //                     possible_movements: HashSet::from([
    //                         Pos::of_str("A6"),
    //                         Pos::of_str("B6"),
    //                         Pos::of_str("C6"),
    //                         Pos::of_str("D6"),
    //                         Pos::of_str("E6"),
    //                         Pos::of_str("F6"),
    //                         Pos::of_str("G6"),
    //                         Pos::of_str("H6"),
    //                         Pos::of_str("A5"),
    //                         Pos::of_str("B5"),
    //                         Pos::of_str("C5"),
    //                         Pos::of_str("D5"),
    //                         Pos::of_str("E5"),
    //                         Pos::of_str("F5"),
    //                         Pos::of_str("G5"),
    //                         Pos::of_str("H5"),
    //                     ]),
    //                 },
    //             ),
    //         ]),
    //         history: Vec::new(),
    //     }
    // ), true);
      }
}
