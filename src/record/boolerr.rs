use binrw::BinRead;

#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 8))]
    _len: u16,

    pub row: u16,
    pub col: u16,
    pub ixfe: u16,

    boolerr: u8,
    #[br(map = |x: u8| x == 0x01)]
    is_err: bool,
}
impl Data {
    pub fn value(&self) -> String {
        if self.is_err {
            let mean = match self.boolerr {
                0x00 => "#NULL!",
                0x07 => "#DIV/0!",
                0x0F => "#VALUE!",
                0x17 => "#REF!",
                0x1D => "#NAME?",
                0x24 => "#NUM!",
                0x2A => "#N/A",
                0x2B => "#GETTING_DATA",
                _ => unreachable!(),
            };
            mean.into()
        } else {
            // TRUE or FALSE
            format!("{}", self.boolerr == 0x01).to_uppercase()
        }
    }
}
