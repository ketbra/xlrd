use binrw::BinRead;
use modular_bitfield::{bitfield, prelude::B30};

// 2.4.220
#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 10))]
    _len: u16,

    pub row: u16,
    pub col: u16,

    _rkrec: RkRec,

    #[br(calc = _rkrec.ixfe())]
    pub ixfe: u16,
    #[br(calc = decode_rk_value(&_rkrec))]
    pub num: f64,
}

/// Decode an RK value according to Excel BIFF8 format.
/// - If fint is true: the 30-bit value is a signed integer
/// - If fint is false: the 30-bit value is the high 30 bits of an IEEE 754 double
/// - If fx100 is true: the result is divided by 100
pub fn decode_rk_value(rk: &RkRec) -> f64 {
    let raw = rk.num();

    let value = if rk.fint() {
        // Integer: treat the 30-bit value as a signed integer
        let v = raw as i32;
        // Sign extend from 30 bits
        let v = if v & 0x20000000 != 0 {
            v | !0x3FFFFFFF // Sign extend negative values
        } else {
            v
        };
        v as f64
    } else {
        // Float: the 30 bits are the high 30 bits of an IEEE 754 double
        // Reconstruct by shifting left by 34 bits (the low 34 bits are zero)
        let bits = (raw as u64) << 34;
        f64::from_bits(bits)
    };

    if rk.fx100() {
        value / 100.0
    } else {
        value
    }
}

#[bitfield]
#[derive(Debug, BinRead)]
#[br(map = Self::from_bytes)]
pub struct RkRec {
    #[skip(setters)]
    pub ixfe: u16,
    #[skip(setters)]
    pub fx100: bool,
    #[skip(setters)]
    pub fint: bool,
    #[skip(setters)]
    pub num: B30,
}
