use binrw::BinRead;
use modular_bitfield::{bitfield, prelude::B12};

#[derive(Debug, BinRead)]
pub struct Data {
    #[br(assert(_len == 4))]
    _len: u16,

    pub info: Info,
    /// Default row height in twips (1/20th of a point)
    #[br(assert(matches!(height, 0..=8179)))]
    pub height: u16,
}

#[bitfield]
#[derive(Debug, BinRead)]
#[br(map = Self::from_bytes,
    assert(self.reserved() == 0))]
pub struct Info {
    /// True if row height was set by user
    #[skip(setters)]
    pub user_set: bool,
    /// True if rows have zero height (hidden by default)
    #[skip(setters)]
    pub zero_height: bool,
    /// True if rows have extra space above
    #[skip(setters)]
    pub extra_space_above: bool,
    /// True if rows have extra space below
    #[skip(setters)]
    pub extra_space_below: bool,
    #[skip(setters)]
    reserved: B12,
}
