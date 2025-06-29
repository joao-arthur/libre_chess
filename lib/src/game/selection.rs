use std::collections::HashSet;

use crate::{
    board::pos::Pos,
    game::{GameBoard, movement::movement::GameMovement},
};

#[derive(Debug, PartialEq)]
struct Selection {
    selected_squares: HashSet<Pos>,
    selected_piece: Option<Pos>,
    selected_piece_movements: Vec<GameMovement>,
}

pub fn toggle(selection: &mut Selection, board: &GameBoard, pos: Pos) {
    if let Some(piece) = board.get(&pos) {
        selection.selected_squares.clear();
        selection.selected_piece = Some(pos);
    } else {
        if selection.selected_squares.contains(&pos) {
            selection.selected_squares.remove(&pos);
        } else {
            selection.selected_squares.insert(pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{board::pos::Pos, game::mode::standard_chess};

    use super::{Selection, toggle};

    #[test]
    fn toggle_select_square() {
        let mut selection = Selection {
            selected_squares: HashSet::new(),
            selected_piece: None,
            selected_piece_movements: Vec::new(),
        };
        let board = standard_chess().initial_board;
        toggle(&mut selection, &board, Pos::of_str("D4"));
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::from([Pos::of_str("D4")]),
                selected_piece: None,
                selected_piece_movements: Vec::new(),
            }
        );
    }

    #[test]
    fn toggle_unselect_square() {
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: None,
            selected_piece_movements: Vec::new(),
        };
        let board = standard_chess().initial_board;
        toggle(&mut selection, &board, Pos::of_str("D4"));
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::new(),
                selected_piece: None,
                selected_piece_movements: Vec::new(),
            }
        );
    }

    #[test]
    fn select_user_piece() {
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: None,
            selected_piece_movements: Vec::new(),
        };
        let board = standard_chess().initial_board;
        toggle(&mut selection, &board, Pos::of_str("B2"));
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::new(),
                selected_piece: Some(Pos::of_str("B2")),
                selected_piece_movements: Vec::new(),
            }
        );
    }

    #[test]
    fn select_other_user_piece() {
        let mut selection = Selection {
            selected_squares: HashSet::from([Pos::of_str("D4")]),
            selected_piece: None,
            selected_piece_movements: Vec::new(),
        };
        let board = standard_chess().initial_board;
        toggle(&mut selection, &board, Pos::of_str("G7"));
        assert_eq!(
            selection,
            Selection {
                selected_squares: HashSet::new(),
                selected_piece: None,
                selected_piece_movements: Vec::new(),
            }
        );
    }
}

/*
Neutral => {
    se selecionou uma célula vazia, seleciona ela; o estado vai ser SelectedEmptyCells
    se selecionou uma célula com peça
        se é da cor do jogador que pode jogar, seleciona a peça e renderiza os movimentos; o estado vai ser SelectedPiece
}
SelectedEmptyCells => {
    se selecionou uma célula vazia, adiciona ela à seleçao; estado continua em SelectedEmptyCells
    se selecionou uma celula com peça
        limpa selecao;
        se é da cor do jogador que pode jogar, seleciona a peça e renderiza os movimentos; o estado vai ser SelectedPiece
        senao
            estado vai ser neutro
}

SelectedPiece => {
    se selecionou um movimento possível da peça, realiza o movimento, limpa seleçao; estado vai ser neutro;
    se selecionou uma celula vazia, limpa seleçao, estado vai ser neutro;
    se selecionou uma peça
        se foi a mesma peça limpa seleçao, estado vai ser neutro;
        se foi outra peça da mesma cor; seleciona ela, estado vai ser SelectedPiece
        se foi peça de outra cor, limpa seleçao, estado vai ser neutro
}
*/
