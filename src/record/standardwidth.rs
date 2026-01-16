use binrw::BinRead;

/// STANDARDWIDTH record (0x0099)
///
/// Default width of columns in 1/256 of the width of the zero character,
/// using the default font (first FONT record in the file).
///
/// This differs from DEFCOLWIDTH which is in whole character units.
#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 2))]
    _len: u16,

    /// Standard column width in 1/256ths of a character
    pub width: u16,
}
