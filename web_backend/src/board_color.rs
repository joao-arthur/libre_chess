use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, PartialEq, Clone)]
pub struct BoardColor {
    pub dark: &'static str,
    pub light: &'static str,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BoardColorPreset {
    pub id: &'static str,
    pub name: &'static str,
}

pub fn board_color_brown() -> BoardColor {
    BoardColor { dark: "#b88762", light: "#edd6b0" }
}

pub fn board_color_green() -> BoardColor {
    BoardColor { dark: "#739552", light: "#ebecd0" }
}

pub fn board_color_red() -> BoardColor {
    BoardColor { dark: "#bb5746", light: "#f5dbc3" }
}

pub fn board_color_orange() -> BoardColor {
    BoardColor { dark: "#d18815", light: "#fae4ae" }
}

pub fn board_color_blue() -> BoardColor {
    BoardColor { dark: "#4b7399", light: "#eae9d2" }
}

pub fn board_color_purple() -> BoardColor {
    BoardColor { dark: "#8476ba", light: "#f0f1f0" }
}

static PRESETS: LazyLock<HashMap<&str, BoardColor>> = LazyLock::new(|| {
    HashMap::from([
        ("brown", board_color_brown()),
        ("green", board_color_green()),
        ("red", board_color_red()),
        ("orange", board_color_orange()),
        ("blue", board_color_blue()),
        ("purple", board_color_purple()),
    ])
});

pub fn try_get_board_color(preset: &str) -> Option<BoardColor> {
    PRESETS.get(preset).cloned()
}

pub fn get_board_color(preset: &str) -> BoardColor {
    try_get_board_color(preset).unwrap()
}

pub fn get_board_color_presets() -> Vec<BoardColorPreset> {
    vec![
        BoardColorPreset { id: "brown", name: "Brown" },
        BoardColorPreset { id: "green", name: "Green" },
        BoardColorPreset { id: "red", name: "Red" },
        BoardColorPreset { id: "orange", name: "Orange" },
        BoardColorPreset { id: "blue", name: "Blue" },
        BoardColorPreset { id: "purple", name: "Purple" },
    ]
}
