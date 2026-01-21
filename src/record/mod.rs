// modular-bitfield's warning
#![allow(unused_parens)]
#![allow(dead_code)]

mod blank;
pub mod bof;
mod boolerr;
pub mod boundsheet8;
mod codepage;
mod colinfo;
mod date1904;
mod defaultrowheight;
mod defautlcolwidth;
mod dimensions;
mod filepass;
pub mod font;
pub mod format;
mod label;
mod labelsst;
mod mergecells;
mod mulblank;
mod mulrk;
mod number;
mod palette;
mod rk;
mod rowinfo;
mod sst;
pub mod style;
mod styleext;
pub mod xf;
mod xfext;

use binrw::{BinRead, BinResult};
use encoding_rs::Encoding;
use enum_display::EnumDisplay;
use std::borrow::Cow;
use std::io::{Read, Seek};

// Wrapper for Data that handles EOF gracefully
#[derive(Debug)]
pub struct IgnoreData {
    pub data: Data,
}

impl BinRead for IgnoreData {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        // Peek at remaining bytes to handle EOF gracefully
        use std::io::SeekFrom;
        let pos_before = reader.stream_position()?;
        reader.seek(SeekFrom::End(0))?;
        let end_pos = reader.stream_position()?;
        reader.seek(SeekFrom::Start(pos_before))?;

        let remaining = (end_pos - pos_before) as usize;

        // If there are fewer than 4 bytes remaining (minimum for type + len),
        // we're at EOF with padding. Return an EOF error to stop parsing.
        if remaining < 4 {
            return Err(binrw::Error::Io(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Not enough bytes for a complete record",
            )));
        }

        // Read type (2 bytes)
        let mut type_buf = [0u8; 2];
        reader.read_exact(&mut type_buf)?;
        let r#type = match endian {
            binrw::Endian::Little => u16::from_le_bytes(type_buf),
            binrw::Endian::Big => u16::from_be_bytes(type_buf),
        };

        // Read length (2 bytes)
        let mut len_buf = [0u8; 2];
        reader.read_exact(&mut len_buf)?;
        let _len = match endian {
            binrw::Endian::Little => u16::from_le_bytes(len_buf),
            binrw::Endian::Big => u16::from_be_bytes(len_buf),
        };

        // Read data bytes
        let mut bytes = vec![0u8; _len as usize];
        reader.read_exact(&mut bytes)?;

        Ok(IgnoreData {
            data: Data { r#type, _len, bytes },
        })
    }
}

// Custom parser for Records that handles EOF padding gracefully
fn parse_records<R: Read + Seek>(
    reader: &mut R,
    endian: binrw::Endian,
    _: (),
) -> BinResult<Vec<Record>> {
    let mut records = Vec::new();
    loop {
        match <Record as BinRead>::read_options(reader, endian, ()) {
            Ok(record) => records.push(record),
            Err(e) => {
                // Check if this is an EOF error (either direct or wrapped)
                let error_str = format!("{:?}", e);
                let is_eof = match &e {
                    binrw::Error::Io(io_err) => {
                        io_err.kind() == std::io::ErrorKind::UnexpectedEof
                    }
                    binrw::Error::NoVariantMatch { .. } => {
                        // When "no variants matched", it's likely because we're at EOF with padding
                        true
                    }
                    _ => {
                        // Check if the error message contains "no variants matched" or "EOF"
                        // This handles cases where the error is wrapped in another error type
                        let contains_no_variant = error_str.contains("no variants matched");
                        let contains_eof = error_str.to_lowercase().contains("eof");
                        contains_no_variant || (contains_eof && error_str.contains("0xffd"))
                    }
                };

                if is_eof {
                    break;
                }

                return Err(e);
            }
        }
    }
    Ok(records)
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Records(#[br(parse_with = parse_records)] pub Vec<Record>);

#[derive(Debug, BinRead, EnumDisplay)]
pub enum Record {
    // global
    #[br(magic(0x0809u16))] // BIFF8
    Bof(bof::Data),
    #[br(magic(0x0085u16))]
    Boundsheet8(boundsheet8::Data),
    #[br(magic(0x0042u16))]
    CodePage(codepage::Data),
    #[br(magic(0x0022u16))]
    Date1904(date1904::Data),
    #[br(magic(0x002Fu16))]
    FilePass(filepass::Data),
    #[br(magic(0x0031u16))]
    Font(font::Data),
    #[br(magic(0x041Eu16))]
    Format(format::Data),
    #[br(magic(0x0092u16))]
    Palette(palette::Data),
    #[br(magic(0x00FCu16))]
    Sst(sst::Data),
    #[br(magic(0x0293u16))]
    Style(style::Data),
    #[br(magic(0x00E0u16))]
    XF(xf::Data),
    #[br(magic(0x087Du16))]
    XFExt(xfext::Data),
    #[br(magic(0x000Au16))]
    Eof(Empty),
    // sheet
    #[br(magic(0x0201u16))]
    Blank(blank::Data),
    #[br(magic(0x0205u16))]
    BoolErr(boolerr::Data),
    #[br(magic(0x007Du16))]
    ColInfo(colinfo::Data),
    #[br(magic(0x0055u16))]
    DefaultColWidth(defautlcolwidth::Data),
    #[br(magic(0x0225u16))]
    DefaultRowHeight(defaultrowheight::Data),
    #[br(magic(0x0200u16))]
    Dimensions(dimensions::Data),
    #[br(magic(0x0204u16))]
    Label(label::Data),
    #[br(magic(0x00FDu16))]
    LabelSST(labelsst::Data),
    #[br(magic(0x00E5u16))]
    MergeCells(mergecells::Data),
    #[br(magic(0x00BEu16))]
    MulBlank(mulblank::Data),
    #[br(magic(0x00BDu16))]
    MulRk(mulrk::Data),
    #[br(magic(0x0203u16))]
    Number(number::Data),
    #[br(magic(0x027Eu16))]
    Rk(rk::Data),
    #[br(magic(0x0208u16))]
    RowInfo(rowinfo::Data),

    Ignore(IgnoreData),
}

// #[allow(dead_code)]
#[derive(Debug, BinRead)]
pub struct Data {
    pub r#type: u16,
    _len: u16,
    #[br(count = _len)]
    pub bytes: Vec<u8>,
}

/* #[cfg(debug_assertions)]
#[derive(Debug, BinRead)]
pub struct Common {
    pub len: u16,
    #[br(count = len)]
    pub bytes: Vec<u8>,
} */

#[derive(Debug, BinRead)]
pub struct Empty {
    #[br(assert(_len == 0))]
    _len: u16,
}

#[derive(Debug, BinRead)]
pub struct XLUnicodeString {
    _cch: u16,

    #[br(assert(_reserved == 0x00 || _reserved == 0x01))]
    _reserved: u8,
    #[br(calc = _reserved == 0x00)]
    hbyte: bool,

    #[br(count = if hbyte { _cch } else { _cch * 2 })]
    bytes: Vec<u8>,
}

#[derive(Debug, BinRead)]
struct ShortXLUnicodeString {
    _cch: u8,
    #[br(map = |x: u8| x == 0x00)]
    hbyte: bool,
    #[br(count = if hbyte { _cch } else { _cch * 2 })]
    bytes: Vec<u8>,
}

fn xlstring(encoding: &'static Encoding, hbyte: bool, bytes: &[u8]) -> String {
    let bytes = if hbyte {
        let bytes = bytes.iter().flat_map(|b| [*b, 0x00]).collect::<Vec<_>>();
        Cow::Owned(bytes)
    } else {
        Cow::Borrowed(bytes)
    };
    encoding.decode(&bytes).0.to_string()
}

#[cfg(feature = "tracing")]
use std::{collections::HashMap, sync::LazyLock};
#[cfg(feature = "tracing")]

pub static RECORDS: LazyLock<HashMap<u16, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        (0x0006, "Formula"),
        (0x000A, "EOF"),
        (0x000C, "CalcCount"),
        (0x000D, "CalcMode"),
        (0x000E, "CalcPrecision"),
        (0x000F, "CalcRefMode"),
        (0x0010, "CalcDelta"),
        (0x0011, "CalcIter"),
        (0x0012, "Protect"),
        (0x0013, "Password"),
        (0x0014, "Header"),
        (0x0015, "Footer"),
        (0x0017, "ExternSheet"),
        (0x0018, "Lbl"),
        (0x0019, "WinProtect"),
        (0x001A, "VerticalPageBreaks"),
        (0x001B, "HorizontalPageBreaks"),
        (0x001C, "Note"),
        (0x001D, "Selection"),
        (0x0022, "Date1904"),
        (0x0023, "ExternName"),
        (0x0026, "LeftMargin"),
        (0x0027, "RightMargin"),
        (0x0028, "TopMargin"),
        (0x0029, "BottomMargin"),
        (0x002A, "PrintRowCol"),
        (0x002B, "PrintGrid"),
        (0x002F, "FilePass"),
        (0x0031, "Font"),
        (0x0033, "PrintSize"),
        (0x003C, "Continue"),
        (0x003D, "Window1"),
        (0x0040, "Backup"),
        (0x0041, "Pane"),
        (0x0042, "CodePage"),
        (0x004D, "Pls"),
        (0x0050, "DCon"),
        (0x0051, "DConRef"),
        (0x0052, "DConName"),
        (0x0055, "DefColWidth"),
        (0x0059, "XCT"),
        (0x005A, "CRN"),
        (0x005B, "FileSharing"),
        (0x005C, "WriteAccess"),
        (0x005D, "Obj"),
        (0x005E, "Uncalced"),
        (0x005F, "CalcSaveRecalc"),
        (0x0060, "Template"),
        (0x0061, "Intl"),
        (0x0063, "ObjProtect"),
        (0x007D, "ColInfo"),
        (0x0080, "Guts"),
        (0x0081, "WsBool"),
        (0x0082, "GridSet"),
        (0x0083, "HCenter"),
        (0x0084, "VCenter"),
        (0x0085, "BoundSheet8"),
        (0x0086, "WriteProtect"),
        (0x008C, "Country"),
        (0x008D, "HideObj"),
        (0x0090, "Sort"),
        (0x0092, "Palette"),
        (0x0097, "Sync"),
        (0x0098, "LPr"),
        (0x0099, "DxGCol"),
        (0x009A, "FnGroupName"),
        (0x009B, "FilterMode"),
        (0x009C, "BuiltInFnGroupCount"),
        (0x009D, "AutoFilterInfo"),
        (0x009E, "AutoFilter"),
        (0x00A0, "Scl"),
        (0x00A1, "Setup"),
        (0x00AE, "ScenMan"),
        (0x00AF, "SCENARIO"),
        (0x00B0, "SxView"),
        (0x00B1, "Sxvd"),
        (0x00B2, "SXVI"),
        (0x00B4, "SxIvd"),
        (0x00B5, "SXLI"),
        (0x00B6, "SXPI"),
        (0x00B8, "DocRoute"),
        (0x00B9, "RecipName"),
        (0x00BD, "MulRk"),
        (0x00BE, "MulBlank"),
        (0x00C1, "Mms"),
        (0x00C5, "SXDI"),
        (0x00C6, "SXDB"),
        (0x00C7, "SXFDB"),
        (0x00C8, "SXDBB"),
        (0x00C9, "SXNum"),
        (0x00CA, "SxBool"),
        (0x00CB, "SxErr"),
        (0x00CC, "SXInt"),
        (0x00CD, "SXString"),
        (0x00CE, "SXDtr"),
        (0x00CF, "SxNil"),
        (0x00D0, "SXTbl"),
        (0x00D1, "SXTBRGIITM"),
        (0x00D2, "SxTbpg"),
        (0x00D3, "ObProj"),
        (0x00D5, "SXStreamID"),
        (0x00D7, "DBCell"),
        (0x00D8, "SXRng"),
        (0x00D9, "SxIsxoper"),
        (0x00DA, "BookBool"),
        (0x00DC, "DbOrParamQry"),
        (0x00DD, "ScenarioProtect"),
        (0x00DE, "OleObjectSize"),
        (0x00E0, "XF"),
        (0x00E1, "InterfaceHdr"),
        (0x00E2, "InterfaceEnd"),
        (0x00E3, "SXVS"),
        (0x00E5, "MergeCells"),
        (0x00E9, "BkHim"),
        (0x00EB, "MsoDrawingGroup"),
        (0x00EC, "MsoDrawing"),
        (0x00ED, "MsoDrawingSelection"),
        (0x00EF, "PhoneticInfo"),
        (0x00F0, "SxRule"),
        (0x00F1, "SXEx"),
        (0x00F2, "SxFilt"),
        (0x00F4, "SxDXF"),
        (0x00F5, "SxItm"),
        (0x00F6, "SxName"),
        (0x00F7, "SxSelect"),
        (0x00F8, "SXPair"),
        (0x00F9, "SxFmla"),
        (0x00FB, "SxFormat"),
        (0x00FC, "SST"),
        (0x00FD, "LabelSst"),
        (0x00FF, "ExtSST"),
        (0x0100, "SXVDEx"),
        (0x0103, "SXFormula"),
        (0x0122, "SXDBEx"),
        (0x0137, "RRDInsDel"),
        (0x0138, "RRDHead"),
        (0x013B, "RRDChgCell"),
        (0x013D, "RRTabId"),
        (0x013E, "RRDRenSheet"),
        (0x013F, "RRSort"),
        (0x0140, "RRDMove"),
        (0x014A, "RRFormat"),
        (0x014B, "RRAutoFmt"),
        (0x014D, "RRInsertSh"),
        (0x014E, "RRDMoveBegin"),
        (0x014F, "RRDMoveEnd"),
        (0x0150, "RRDInsDelBegin"),
        (0x0151, "RRDInsDelEnd"),
        (0x0152, "RRDConflict"),
        (0x0153, "RRDDefName"),
        (0x0154, "RRDRstEtxp"),
        (0x015F, "LRng"),
        (0x0160, "UsesELFs"),
        (0x0161, "DSF"),
        (0x0191, "CUsr"),
        (0x0192, "CbUsr"),
        (0x0193, "UsrInfo"),
        (0x0194, "UsrExcl"),
        (0x0195, "FileLock"),
        (0x0196, "RRDInfo"),
        (0x0197, "BCUsrs"),
        (0x0198, "UsrChk"),
        (0x01A9, "UserBView"),
        (0x01AA, "UserSViewBegin"),
        (0x01AB, "UserSViewEnd"),
        (0x01AC, "RRDUserView"),
        (0x01AD, "Qsi"),
        (0x01AE, "SupBook"),
        (0x01AF, "Prot4Rev"),
        (0x01B0, "CondFmt"),
        (0x01B1, "CF"),
        (0x01B2, "DVal"),
        (0x01B5, "DConBin"),
        (0x01B6, "TxO"),
        (0x01B7, "RefreshAll"),
        (0x01B8, "HLink"),
        (0x01B9, "Lel"),
        (0x01BA, "CodeName"),
        (0x01BB, "SXFDBType"),
        (0x01BC, "Prot4RevPass"),
        (0x01BD, "ObNoMacros"),
        (0x01BE, "Dv"),
        (0x01C0, "Excel9File"),
        (0x01C1, "RecalcId"),
        (0x01C2, "EntExU2"),
        (0x0200, "Dimensions"),
        (0x0201, "Blank"),
        (0x0203, "Number"),
        (0x0204, "Label"),
        (0x0205, "BoolErr"),
        (0x0207, "String"),
        (0x0208, "Row"),
        (0x020B, "Index"),
        (0x0221, "Array"),
        (0x0225, "DefaultRowHeight"),
        (0x0236, "Table"),
        (0x023E, "Window2"),
        (0x027E, "RK"),
        (0x0293, "Style"),
        (0x0418, "BigName"),
        (0x041E, "Format"),
        (0x043C, "ContinueBigName"),
        (0x04BC, "ShrFmla"),
        (0x0800, "HLinkTooltip"),
        (0x0801, "WebPub"),
        (0x0802, "QsiSXTag"),
        (0x0803, "DBQueryExt"),
        (0x0804, "ExtString"),
        (0x0805, "TxtQry"),
        (0x0806, "Qsir"),
        (0x0807, "Qsif"),
        (0x0808, "RRDTQSIF"),
        (0x0809, "BOF"),
        (0x080A, "OleDbConn"),
        (0x080B, "WOpt"),
        (0x080C, "SXViewEx"),
        (0x080D, "SXTH"),
        (0x080E, "SXPIEx"),
        (0x080F, "SXVDTEx"),
        (0x0810, "SXViewEx9"),
        (0x0812, "ContinueFrt"),
        (0x0813, "RealTimeData"),
        (0x0850, "ChartFrtInfo"),
        (0x0851, "FrtWrapper"),
        (0x0852, "StartBlock"),
        (0x0853, "EndBlock"),
        (0x0854, "StartObject"),
        (0x0855, "EndObject"),
        (0x0856, "CatLab"),
        (0x0857, "YMult"),
        (0x0858, "SXViewLink"),
        (0x0859, "PivotChartBits"),
        (0x085A, "FrtFontList"),
        (0x0862, "SheetExt"),
        (0x0863, "BookExt"),
        (0x0864, "SXAddl"),
        (0x0865, "CrErr"),
        (0x0866, "HFPicture"),
        (0x0867, "FeatHdr"),
        (0x0868, "Feat"),
        (0x086A, "DataLabExt"),
        (0x086B, "DataLabExtContents"),
        (0x086C, "CellWatch"),
        (0x0871, "FeatHdr11"),
        (0x0872, "Feature11"),
        (0x0874, "DropDownObjIds"),
        (0x0875, "ContinueFrt11"),
        (0x0876, "DConn"),
        (0x0877, "List12"),
        (0x0878, "Feature12"),
        (0x0879, "CondFmt12"),
        (0x087A, "CF12"),
        (0x087B, "CFEx"),
        (0x087C, "XFCRC"),
        (0x087D, "XFExt"),
        (0x087E, "AutoFilter12"),
        (0x087F, "ContinueFrt12"),
        (0x0884, "MDTInfo"),
        (0x0885, "MDXStr"),
        (0x0886, "MDXTuple"),
        (0x0887, "MDXSet"),
        (0x0888, "MDXProp"),
        (0x0889, "MDXKPI"),
        (0x088A, "MDB"),
        (0x088B, "PLV"),
        (0x088C, "Compat12"),
        (0x088D, "DXF"),
        (0x088E, "TableStyles"),
        (0x088F, "TableStyle"),
        (0x0890, "TableStyleElement"),
        (0x0892, "StyleExt"),
        (0x0893, "NamePublish"),
        (0x0894, "NameCmt"),
        (0x0895, "SortData"),
        (0x0896, "Theme"),
        (0x0897, "GUIDTypeLib"),
        (0x0898, "FnGrp12"),
        (0x0899, "NameFnGrp12"),
        (0x089A, "MTRSettings"),
        (0x089B, "CompressPictures"),
        (0x089C, "HeaderFooter"),
        (0x089D, "CrtLayout12"),
        (0x089E, "CrtMlFrt"),
        (0x089F, "CrtMlFrtContinue"),
        (0x08A3, "ForceFullCalculation"),
        (0x08A4, "ShapePropsStream"),
        (0x08A5, "TextPropsStream"),
        (0x08A6, "RichTextStream"),
        (0x08A7, "CrtLayout12A"),
        (0x1001, "Units"),
        (0x1002, "Chart"),
        (0x1003, "Series"),
        (0x1006, "DataFormat"),
        (0x1007, "LineFormat"),
        (0x1009, "MarkerFormat"),
        (0x100A, "AreaFormat"),
        (0x100B, "PieFormat"),
        (0x100C, "AttachedLabel"),
        (0x100D, "SeriesText"),
        (0x1014, "ChartFormat"),
        (0x1015, "Legend"),
        (0x1016, "SeriesList"),
        (0x1017, "Bar"),
        (0x1018, "Line"),
        (0x1019, "Pie"),
        (0x101A, "Area"),
        (0x101B, "Scatter"),
        (0x101C, "CrtLine"),
        (0x101D, "Axis"),
        (0x101E, "Tick"),
        (0x101F, "ValueRange"),
        (0x1020, "CatSerRange"),
        (0x1021, "AxisLine"),
        (0x1022, "CrtLink"),
        (0x1024, "DefaultText"),
        (0x1025, "Text"),
        (0x1026, "FontX"),
        (0x1027, "ObjectLink"),
        (0x1032, "Frame"),
        (0x1033, "Begin"),
        (0x1034, "End"),
        (0x1035, "PlotArea"),
        (0x103A, "Chart3d"),
        (0x103C, "PicF"),
        (0x103D, "DropBar"),
        (0x103E, "Radar"),
        (0x103F, "Surf"),
        (0x1040, "RadarArea"),
        (0x1041, "AxisParent"),
        (0x1043, "LegendException"),
        (0x1044, "ShtProps"),
        (0x1045, "SerToCrt"),
        (0x1046, "AxesUsed"),
        (0x1048, "SBaseRef"),
        (0x104A, "SerParent"),
        (0x104B, "SerAuxTrend"),
        (0x104E, "IFmtRecord"),
        (0x104F, "Pos"),
        (0x1050, "AlRuns"),
        (0x1051, "BRAI"),
        (0x105B, "SerAuxErrBar"),
        (0x105C, "ClrtClient"),
        (0x105D, "SerFmt"),
        (0x105F, "Chart3DBarShape"),
        (0x1060, "Fbi"),
        (0x1061, "BopPop"),
        (0x1062, "AxcExt"),
        (0x1063, "Dat"),
        (0x1064, "PlotGrowth"),
        (0x1065, "SIIndex"),
        (0x1066, "GelFrame"),
        (0x1067, "BopPopCustom"),
        (0x1068, "Fbi2"),
    ])
});
