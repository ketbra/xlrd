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
    #[br(calc = decode_rk_num(&_rkrec))]
    pub num: f64,
}

/// Decode an RK record number value
///
/// RK encoding:
/// - Bit 0: fx100 (if set, divide by 100)
/// - Bit 1: fInt (if set, value is integer in high 30 bits; if clear, value is IEEE 754 float with low 2 bits of mantissa cleared)
/// - Bits 2-31: num (30-bit value)
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
        // We need to shift it left by 2 to form bits 2-31 of a 32-bit value,
        // then reinterpret as the high 32 bits of a 64-bit IEEE 754 double
        let high_bits = (rk.num() as u64) << 34; // Shift to bits 34-63 of u64
        f64::from_bits(high_bits)
    };

    if rk.fx100() {
        v / 100.0
    } else {
        v
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
