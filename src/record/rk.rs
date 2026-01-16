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

/// Decode an RK value according to the BIFF specification.
///
/// RK values are compressed numbers stored in 4 bytes:
/// - Bit 0 (fx100): If set, divide the result by 100
/// - Bit 1 (fint): If set, value is a signed 30-bit integer; otherwise, it's
///   the high 30 bits of an IEEE 754 64-bit floating-point number
/// - Bits 2-31: The 30-bit data value
pub fn decode_rk_value(rk: &RkRec) -> f64 {
    let value = if rk.fint() {
        // Signed 30-bit integer
        // The num field contains the 30-bit value, we need to sign-extend it
        let num = rk.num() as i32;
        // Sign extend from 30 bits to 32 bits
        let signed = if num & 0x2000_0000 != 0 {
            // Negative: extend sign
            num | (0xC000_0000_u32 as i32)
        } else {
            num
        };
        signed as f64
    } else {
        // IEEE 754 float: the 30 bits are the high 30 bits of a 64-bit double
        // Low 34 bits are zero (we have bits 34-63, i.e., the high 30 bits)
        let high_bits = (rk.num() as u64) << 34;
        f64::from_bits(high_bits)
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
