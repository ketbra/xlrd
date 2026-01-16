use binrw::BinRead;

#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 2))]
    _len: u16,

    /// Default column width in characters (0-255)
    #[br(assert(width <= 0x00FF))]
    pub width: u16,
}
