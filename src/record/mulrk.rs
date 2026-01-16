use super::rk::{RkRec, decode_rk_value};
use binrw::BinRead;

// 2.4.175
#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,

    pub row: u16,
    #[br(assert(col_min <= 254))]
    pub col_min: u16,

    #[br(count = _len / 6 - 1)]
    _rks: Vec<RkRec>,

    #[br(assert(col_min < _col_max))]
    _col_max: u16,

    #[br(calc = _rks.iter().map(|rk| {
        (rk.ixfe(), decode_rk_value(rk))
    }).collect::<Vec<_>>())]
    pub values: Vec<(u16, f64)>, // (ixfe, num)
}
