use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, PartialEq, Clone)]
pub struct BoardSet {
    pub bb: &'static str,
    pub bk: &'static str,
    pub bn: &'static str,
    pub bp: &'static str,
    pub bq: &'static str,
    pub br: &'static str,
    pub wb: &'static str,
    pub wk: &'static str,
    pub wn: &'static str,
    pub wp: &'static str,
    pub wq: &'static str,
    pub wr: &'static str,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BoardSetPreset {
    pub id: &'static str,
    pub name: &'static str,
}

const BB_NORMAL_1: &str = include_str!("../sets/normal_1/bb.svg");
const BK_NORMAL_1: &str = include_str!("../sets/normal_1/bk.svg");
const BN_NORMAL_1: &str = include_str!("../sets/normal_1/bn.svg");
const BP_NORMAL_1: &str = include_str!("../sets/normal_1/bp.svg");
const BQ_NORMAL_1: &str = include_str!("../sets/normal_1/bq.svg");
const BR_NORMAL_1: &str = include_str!("../sets/normal_1/br.svg");
const WB_NORMAL_1: &str = include_str!("../sets/normal_1/wb.svg");
const WK_NORMAL_1: &str = include_str!("../sets/normal_1/wk.svg");
const WN_NORMAL_1: &str = include_str!("../sets/normal_1/wn.svg");
const WP_NORMAL_1: &str = include_str!("../sets/normal_1/wp.svg");
const WQ_NORMAL_1: &str = include_str!("../sets/normal_1/wq.svg");
const WR_NORMAL_1: &str = include_str!("../sets/normal_1/wr.svg");

const BB_NORMAL_2: &str = include_str!("../sets/normal_2/bb.svg");
const BK_NORMAL_2: &str = include_str!("../sets/normal_2/bk.svg");
const BN_NORMAL_2: &str = include_str!("../sets/normal_2/bn.svg");
const BP_NORMAL_2: &str = include_str!("../sets/normal_2/bp.svg");
const BQ_NORMAL_2: &str = include_str!("../sets/normal_2/bq.svg");
const BR_NORMAL_2: &str = include_str!("../sets/normal_2/br.svg");
const WB_NORMAL_2: &str = include_str!("../sets/normal_2/wb.svg");
const WK_NORMAL_2: &str = include_str!("../sets/normal_2/wk.svg");
const WN_NORMAL_2: &str = include_str!("../sets/normal_2/wn.svg");
const WP_NORMAL_2: &str = include_str!("../sets/normal_2/wp.svg");
const WQ_NORMAL_2: &str = include_str!("../sets/normal_2/wq.svg");
const WR_NORMAL_2: &str = include_str!("../sets/normal_2/wr.svg");

const BB_SET_FANTASY: &str = include_str!("../sets/fantasy/bb.svg");
const BK_SET_FANTASY: &str = include_str!("../sets/fantasy/bk.svg");
const BN_SET_FANTASY: &str = include_str!("../sets/fantasy/bn.svg");
const BP_SET_FANTASY: &str = include_str!("../sets/fantasy/bp.svg");
const BQ_SET_FANTASY: &str = include_str!("../sets/fantasy/bq.svg");
const BR_SET_FANTASY: &str = include_str!("../sets/fantasy/br.svg");
const WB_SET_FANTASY: &str = include_str!("../sets/fantasy/wb.svg");
const WK_SET_FANTASY: &str = include_str!("../sets/fantasy/wk.svg");
const WN_SET_FANTASY: &str = include_str!("../sets/fantasy/wn.svg");
const WP_SET_FANTASY: &str = include_str!("../sets/fantasy/wp.svg");
const WQ_SET_FANTASY: &str = include_str!("../sets/fantasy/wq.svg");
const WR_SET_FANTASY: &str = include_str!("../sets/fantasy/wr.svg");

const BB_SET_SPATIAL: &str = include_str!("../sets/spatial/bb.svg");
const BK_SET_SPATIAL: &str = include_str!("../sets/spatial/bk.svg");
const BN_SET_SPATIAL: &str = include_str!("../sets/spatial/bn.svg");
const BP_SET_SPATIAL: &str = include_str!("../sets/spatial/bp.svg");
const BQ_SET_SPATIAL: &str = include_str!("../sets/spatial/bq.svg");
const BR_SET_SPATIAL: &str = include_str!("../sets/spatial/br.svg");
const WB_SET_SPATIAL: &str = include_str!("../sets/spatial/wb.svg");
const WK_SET_SPATIAL: &str = include_str!("../sets/spatial/wk.svg");
const WN_SET_SPATIAL: &str = include_str!("../sets/spatial/wn.svg");
const WP_SET_SPATIAL: &str = include_str!("../sets/spatial/wp.svg");
const WQ_SET_SPATIAL: &str = include_str!("../sets/spatial/wq.svg");
const WR_SET_SPATIAL: &str = include_str!("../sets/spatial/wr.svg");

pub fn board_set_normal_1() -> BoardSet {
    BoardSet {
        bb: BB_NORMAL_1,
        bk: BK_NORMAL_1,
        bn: BN_NORMAL_1,
        bp: BP_NORMAL_1,
        bq: BQ_NORMAL_1,
        br: BR_NORMAL_1,
        wb: WB_NORMAL_1,
        wk: WK_NORMAL_1,
        wn: WN_NORMAL_1,
        wp: WP_NORMAL_1,
        wq: WQ_NORMAL_1,
        wr: WR_NORMAL_1,
    }
}

pub fn board_set_normal_2() -> BoardSet {
    BoardSet {
        bb: BB_NORMAL_2,
        bk: BK_NORMAL_2,
        bn: BN_NORMAL_2,
        bp: BP_NORMAL_2,
        bq: BQ_NORMAL_2,
        br: BR_NORMAL_2,
        wb: WB_NORMAL_2,
        wk: WK_NORMAL_2,
        wn: WN_NORMAL_2,
        wp: WP_NORMAL_2,
        wq: WQ_NORMAL_2,
        wr: WR_NORMAL_2,
    }
}

pub fn board_set_fantasy() -> BoardSet {
    BoardSet {
        bb: BB_SET_FANTASY,
        bk: BK_SET_FANTASY,
        bn: BN_SET_FANTASY,
        bp: BP_SET_FANTASY,
        bq: BQ_SET_FANTASY,
        br: BR_SET_FANTASY,
        wb: WB_SET_FANTASY,
        wk: WK_SET_FANTASY,
        wn: WN_SET_FANTASY,
        wp: WP_SET_FANTASY,
        wq: WQ_SET_FANTASY,
        wr: WR_SET_FANTASY,
    }
}

pub fn board_set_spatial() -> BoardSet {
    BoardSet {
        bb: BB_SET_SPATIAL,
        bk: BK_SET_SPATIAL,
        bn: BN_SET_SPATIAL,
        bp: BP_SET_SPATIAL,
        bq: BQ_SET_SPATIAL,
        br: BR_SET_SPATIAL,
        wb: WB_SET_SPATIAL,
        wk: WK_SET_SPATIAL,
        wn: WN_SET_SPATIAL,
        wp: WP_SET_SPATIAL,
        wq: WQ_SET_SPATIAL,
        wr: WR_SET_SPATIAL,
    }
}

static PRESETS: LazyLock<HashMap<String, BoardSet>> = LazyLock::new(|| {
    HashMap::from([
        (String::from("normal_1"), board_set_normal_1()),
        (String::from("normal_2"), board_set_normal_2()),
        (String::from("fantasy"), board_set_fantasy()),
        (String::from("spatial"), board_set_spatial()),
    ])
});

pub fn try_get_board_set(preset: &str) -> Option<BoardSet> {
    PRESETS.get(preset).cloned()
}

pub fn get_board_set(preset: &str) -> BoardSet {
    try_get_board_set(preset).unwrap()
}

pub fn get_board_set_presets() -> Vec<BoardSetPreset> {
    vec![
        BoardSetPreset { id: "normal_1", name: "Normal 1" },
        BoardSetPreset { id: "normal_2", name: "Normal 2" },
        BoardSetPreset { id: "fantasy", name: "Fantasy" },
        BoardSetPreset { id: "spatial", name: "Spatial" },
    ]
}
