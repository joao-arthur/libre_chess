use crate::{
    board::pos::Pos,
    game::{board::GameBoard, game::GameHistory, movement::movement::CastlingMovement},
    geometry::poligon::rect::RectU8,
    movement::Movement,
    piece::Type,
};

pub fn movements(
    board: &GameBoard,
    bounds: &RectU8,
    history: &GameHistory,
    pos: &Pos,
) -> Vec<CastlingMovement> {
    let mut result: Vec<CastlingMovement> = Vec::new();
    if let Some(piece) = board.get(pos) {
        let mut col_index = 0;
        loop {
            col_index += 1;
            let curr_pos = Pos { row: pos.row, col: pos.col + col_index };
            if curr_pos.col < bounds.x1
                || curr_pos.col > bounds.x2
                || curr_pos.row < bounds.y1
                || curr_pos.row > bounds.y2
            {
                break;
            }
            if let Some(maybe_rook) = board.get(&curr_pos) {
                if maybe_rook.t == Type::Rook
                    && maybe_rook.color == piece.color
                    && !history.iter().any(|mov| mov.piece == *maybe_rook)
                {
                    result.push(CastlingMovement::from(Movement {
                        piece: *piece,
                        from: pos.clone(),
                        to: curr_pos,
                    }));
                }
                break;
            }
        }
        loop {
            col_index += 1;
            if col_index > pos.col {
                break;
            }
            let curr_pos = Pos { row: pos.row, col: pos.col - col_index };
            if curr_pos.col < bounds.x1
                || curr_pos.col > bounds.x2
                || curr_pos.row < bounds.y1
                || curr_pos.row > bounds.y2
            {
                continue;
            }
            if let Some(maybe_rook) = board.get(&curr_pos) {
                if maybe_rook.t == Type::Rook
                    && maybe_rook.color == piece.color
                    && !history.iter().any(|mov| mov.piece == *maybe_rook)
                {
                    result.push(CastlingMovement::from(Movement {
                        piece: *piece,
                        from: pos.clone(),
                        to: curr_pos,
                    }));
                }
                break;
            }
        }
    }

    result
}
