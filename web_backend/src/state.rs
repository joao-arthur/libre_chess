struct Selection {
    
}



enum State {
        Neutral,
        SelectedEmptyCells,
        SelectedPiece,
    }

    enum Event {
        SelectedEmptyCell,
        SelectedUserPiece,
        SelectedAnotherUserPiece,
        SelectedMovement,
    }

    pub fn change_state(state: &State, event: &Event) {
        match state {
            State::Neutral => {}
            State::SelectedEmptyCells => {}
            State::SelectedPiece => {}
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

    pub fn app_click(row: u16, col: u16) {
        MODEL.with(|i| {
            let mut m = i.borrow_mut();
            let turn = evaluate_turn(&m.game.history);
            let player = &m.game.players.get(&turn).unwrap();
            let dim = m.settings.render_settings.dim as f64;
            let cell_size = dim / 8.0;
            let cell_row = (8 - ((((row as f64) / cell_size).floor() as u8) as i16)) as u8 - 1;
            let cell_col = ((col as f64) / cell_size).floor() as u8;
            let pos = Pos { row: cell_row, col: cell_col };
            if m.settings.selected_piece_movements.contains(&pos) {
                let piece =
                    m.game.board.get(m.settings.selected_piece.as_ref().unwrap()).unwrap().clone();
                let from = m.settings.selected_piece.clone().unwrap();
                let to = pos;
                app_move_piece(&mut m.game, Movement { piece: piece.clone(), from, to });
                m.settings.selected_piece = None;
                m.settings.selected_piece_movements = HashSet::new();
                m.settings.selected_squares = HashSet::new()
            } else {
                if let Some(piece) = m.game.board.get(&pos.clone()) {
                    if m.settings.selected_piece == Some(pos.clone()) {
                        m.settings.selected_piece = None;
                        m.settings.selected_piece_movements = HashSet::new();
                    } else {
                        m.settings.selected_squares.clear();
                        let movements = player.moves;
                        m.settings.selected_piece = Some(pos.clone());
                        m.settings.selected_piece_movements = movements;
                    }
                } else {
                    if m.settings.selected_piece.is_some() {
                        m.settings.selected_piece = None;
                        m.settings.selected_piece_movements = HashSet::new();
                    } else {
                        if m.settings.selected_squares.contains(&pos) {
                            m.settings.selected_squares.remove(&pos);
                        } else {
                            m.settings.selected_squares.insert(pos);
                        }
                    }
                }
            }
        });
        on_change(Prop::BoardSet);
    }
