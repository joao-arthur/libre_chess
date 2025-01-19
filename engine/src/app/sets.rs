const BB_SET_1: &str = include_str!("../../sets/set_1/bb.svg");
const BK_SET_1: &str = include_str!("../../sets/set_1/bb.svg");
const BN_SET_1: &str = include_str!("../../sets/set_1/bb.svg");
const BP_SET_1: &str = include_str!("../../sets/set_1/bb.svg");
const BQ_SET_1: &str = include_str!("../../sets/set_1/bb.svg");
const BR_SET_1: &str = include_str!("../../sets/set_1/bb.svg");
const WB_SET_1: &str = include_str!("../../sets/set_1/bb.svg");
const WK_SET_1: &str = include_str!("../../sets/set_1/bb.svg");
const WN_SET_1: &str = include_str!("../../sets/set_1/bb.svg");
const WP_SET_1: &str = include_str!("../../sets/set_1/bb.svg");
const WQ_SET_1: &str = include_str!("../../sets/set_1/bb.svg");
const WR_SET_1: &str = include_str!("../../sets/set_1/bb.svg");

const BB_SET_2: &str = include_str!("../../sets/set_2/bb.svg");
const BK_SET_2: &str = include_str!("../../sets/set_2/bb.svg");
const BN_SET_2: &str = include_str!("../../sets/set_2/bb.svg");
const BP_SET_2: &str = include_str!("../../sets/set_2/bb.svg");
const BQ_SET_2: &str = include_str!("../../sets/set_2/bb.svg");
const BR_SET_2: &str = include_str!("../../sets/set_2/bb.svg");
const WB_SET_2: &str = include_str!("../../sets/set_2/bb.svg");
const WK_SET_2: &str = include_str!("../../sets/set_2/bb.svg");
const WN_SET_2: &str = include_str!("../../sets/set_2/bb.svg");
const WP_SET_2: &str = include_str!("../../sets/set_2/bb.svg");
const WQ_SET_2: &str = include_str!("../../sets/set_2/bb.svg");
const WR_SET_2: &str = include_str!("../../sets/set_2/bb.svg");

const BB_SET_FANTASY: &str = include_str!("../../sets/maurizio_monge_fantasy/bb.svg");
const BK_SET_FANTASY: &str = include_str!("../../sets/maurizio_monge_fantasy/bb.svg");
const BN_SET_FANTASY: &str = include_str!("../../sets/maurizio_monge_fantasy/bb.svg");
const BP_SET_FANTASY: &str = include_str!("../../sets/maurizio_monge_fantasy/bb.svg");
const BQ_SET_FANTASY: &str = include_str!("../../sets/maurizio_monge_fantasy/bb.svg");
const BR_SET_FANTASY: &str = include_str!("../../sets/maurizio_monge_fantasy/bb.svg");
const WB_SET_FANTASY: &str = include_str!("../../sets/maurizio_monge_fantasy/bb.svg");
const WK_SET_FANTASY: &str = include_str!("../../sets/maurizio_monge_fantasy/bb.svg");
const WN_SET_FANTASY: &str = include_str!("../../sets/maurizio_monge_fantasy/bb.svg");
const WP_SET_FANTASY: &str = include_str!("../../sets/maurizio_monge_fantasy/bb.svg");
const WQ_SET_FANTASY: &str = include_str!("../../sets/maurizio_monge_fantasy/bb.svg");
const WR_SET_FANTASY: &str = include_str!("../../sets/maurizio_monge_fantasy/bb.svg");

const BB_SET_SPATIAL: &str = include_str!("../../sets/maurizio_monge_spatial/bb.svg");
const BK_SET_SPATIAL: &str = include_str!("../../sets/maurizio_monge_spatial/bb.svg");
const BN_SET_SPATIAL: &str = include_str!("../../sets/maurizio_monge_spatial/bb.svg");
const BP_SET_SPATIAL: &str = include_str!("../../sets/maurizio_monge_spatial/bb.svg");
const BQ_SET_SPATIAL: &str = include_str!("../../sets/maurizio_monge_spatial/bb.svg");
const BR_SET_SPATIAL: &str = include_str!("../../sets/maurizio_monge_spatial/bb.svg");
const WB_SET_SPATIAL: &str = include_str!("../../sets/maurizio_monge_spatial/bb.svg");
const WK_SET_SPATIAL: &str = include_str!("../../sets/maurizio_monge_spatial/bb.svg");
const WN_SET_SPATIAL: &str = include_str!("../../sets/maurizio_monge_spatial/bb.svg");
const WP_SET_SPATIAL: &str = include_str!("../../sets/maurizio_monge_spatial/bb.svg");
const WQ_SET_SPATIAL: &str = include_str!("../../sets/maurizio_monge_spatial/bb.svg");
const WR_SET_SPATIAL: &str = include_str!("../../sets/maurizio_monge_spatial/bb.svg");

pub struct BoardSet {
    pub bb: String,
    pub bk: String,
    pub bn: String,
    pub bp: String,
    pub bq: String,
    pub br: String,
    pub wb: String,
    pub wk: String,
    pub wn: String,
    pub wp: String,
    pub wq: String,
    pub wr: String,
}

pub fn set_maurizio_monge_fantasy() -> BoardSet {
    BoardSet {
        bb: String::from(BB_SET_FANTASY),
        bk: String::from(BK_SET_FANTASY),
        bn: String::from(BN_SET_FANTASY),
        bp: String::from(BP_SET_FANTASY),
        bq: String::from(BQ_SET_FANTASY),
        br: String::from(BR_SET_FANTASY),
        wb: String::from(WB_SET_FANTASY),
        wk: String::from(WK_SET_FANTASY),
        wn: String::from(WN_SET_FANTASY),
        wp: String::from(WP_SET_FANTASY),
        wq: String::from(WQ_SET_FANTASY),
        wr: String::from(WR_SET_FANTASY), 
    }
}

pub fn set_maurizio_monge_spatial() -> BoardSet {
    BoardSet {
        bb: String::from(BB_SET_SPATIAL),
        bk: String::from(BK_SET_SPATIAL),
        bn: String::from(BN_SET_SPATIAL),
        bp: String::from(BP_SET_SPATIAL),
        bq: String::from(BQ_SET_SPATIAL),
        br: String::from(BR_SET_SPATIAL),
        wb: String::from(WB_SET_SPATIAL),
        wk: String::from(WK_SET_SPATIAL),
        wn: String::from(WN_SET_SPATIAL),
        wp: String::from(WP_SET_SPATIAL),
        wq: String::from(WQ_SET_SPATIAL),
        wr: String::from(WR_SET_SPATIAL), 
    }
}

pub fn set_1() -> BoardSet {
    BoardSet {
        bb: String::from(BB_SET_1),
        bk: String::from(BK_SET_1),
        bn: String::from(BN_SET_1),
        bp: String::from(BP_SET_1),
        bq: String::from(BQ_SET_1),
        br: String::from(BR_SET_1),
        wb: String::from(WB_SET_1),
        wk: String::from(WK_SET_1),
        wn: String::from(WN_SET_1),
        wp: String::from(WP_SET_1),
        wq: String::from(WQ_SET_1),
        wr: String::from(WR_SET_1), 
    }
}

pub fn set_2() -> BoardSet {
    BoardSet {
        bb: String::from(BB_SET_2),
        bk: String::from(BK_SET_2),
        bn: String::from(BN_SET_2),
        bp: String::from(BP_SET_2),
        bq: String::from(BQ_SET_2),
        br: String::from(BR_SET_2),
        wb: String::from(WB_SET_2),
        wk: String::from(WK_SET_2),
        wn: String::from(WN_SET_2),
        wp: String::from(WP_SET_2),
        wq: String::from(WQ_SET_2),
        wr: String::from(WR_SET_2), 
    }
}