use super::rk::RkRec;
use binrw::BinRead;

/// Decode an RK record number value (same logic as in rk.rs)
fn decode_rk_num(rk: &RkRec) -> f64 {
    let v = if rk.fint() {
        // Integer value: sign-extend the 30-bit value to i32
        let num = rk.num() as u32;
        // Check if bit 29 is set (sign bit for 30-bit signed int)
        let signed = if (num & 0x20000000) != 0 {
            // Sign-extend: set bits 30-31
            (num | 0xC0000000) as i32
        } else {
            num as i32
        };
        signed as f64
    } else {
        // IEEE 754 floating-point: the 30-bit value forms the high 30 bits
        let high_bits = (rk.num() as u64) << 34; // Shift to bits 34-63 of u64
        f64::from_bits(high_bits)
    };

    if rk.fx100() {
        v / 100.0
    } else {
        v
    }
}

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
        (rk.ixfe(), decode_rk_num(rk))
    }).collect::<Vec<_>>())]
    pub values: Vec<(u16, f64)>, // (ixfe, num)
}
