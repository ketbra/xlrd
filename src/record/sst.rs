use super::Data as ContinueData;
use crate::error::Result;
use binrw::{
    BinRead,
    helpers::{until, until_eof},
};
use encoding_rs::Encoding;
use std::io::Cursor;

// 2.4.265
#[derive(Debug, BinRead)]
pub struct Data {
    _len: u16,

    _total: i32,
    _unique: i32,

    #[br(count = _len - 8)]
    bytes: Vec<u8>,

    #[br(parse_with = until(|cd: &ContinueData| cd.r#type != 0x003C))]
    continues: Vec<ContinueData>,

    #[br(ignore)]
    pub strs: Vec<String>,
}

impl Data {
    pub fn decode(&mut self, encoding: &'static Encoding) -> Result<()> {
        let mut bytes = self.bytes.clone();
        let continue_bytes = self
            .continues
            .iter()
            .flat_map(|c| c.bytes.clone())
            .collect::<Vec<_>>();
        bytes.extend(continue_bytes);

        let mut stream = Cursor::new(bytes);
        let xlstrs = XLStrings::read(&mut stream)?;

        self.strs = xlstrs
            .0
            .iter()
            .map(|s| super::xlstring(encoding, s.hbyte, &s.bytes))
            .collect::<Vec<_>>();
        Ok(())
    }
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct XLStrings(#[br(parse_with = until_eof)] Vec<XLUnicodeRichExtendedString>);

#[derive(Debug, BinRead)]
struct XLUnicodeRichExtendedString {
    _cch: u16,

    _byte: u8,
    #[br(calc = _byte & 0x01 == 0x00)]
    hbyte: bool,
    /// _reserved1 bits = 1
    #[br(calc = _byte >> 2 & 0x01 == 0x01)]
    _hext: bool,
    #[br(calc = _byte >> 3 & 0x01 == 0x01)]
    _hrun: bool,
    /// _reserved2 bits = 4

    #[br(if(_hrun))]
    _crun: u16,
    #[br(if(_hext))]
    _cext: i32,

    #[br(count = if hbyte { _cch as usize } else { (_cch as usize) * 2 })]
    bytes: Vec<u8>,

    #[br(if(_hrun), count = _crun)]
    _brun: Vec<(u16, u16)>, // FormatRun(ich, ifnt)
    #[br(if(_hext), count = _cext)]
    _bext: Vec<u8>,
}
