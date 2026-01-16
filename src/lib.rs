mod error;
mod model;
mod record;

pub use error::Error;

use binrw::BinRead;
use encoding_rs::UTF_16LE;
use error::Result;
use model::{Global, Value};
use record::{Record, Records, bof::StreamType, boundsheet8::SheetType};
use std::{
    io::{Seek, SeekFrom},
    path::Path,
};
use umya_spreadsheet::{
    Color, Spreadsheet, Style, Worksheet, helper::coordinate::coordinate_from_index,
    new_file_empty_worksheet, writer::xlsx,
};

#[cfg(feature = "tracing")]
use std::collections::HashMap;

/// Converts an XLS file to XLSX format.
///
/// # Arguments
///
/// * `path` - Path to the XLS file to convert
///
/// # Returns
///
/// * `Result<impl AsRef<Path>>` - Path to the converted XLSX file path on success, or an error on failure
pub fn xls2xlsx(path: impl AsRef<Path>) -> Result<impl AsRef<Path>> {
    if path.as_ref().extension() != Some("xls".as_ref()) {
        return Err(Error::XlsExt);
    }

    let workbook = open(&path)?;
    let xpath = path.as_ref().with_extension("xlsx");
    save(&workbook, &xpath)?;

    Ok(xpath)
}

/// Saves a umya_spreadsheet::Spreadsheet structure to an XLSX file.
///
/// # Arguments
///
/// * `workbook` - umya_spreadsheet::Spreadsheet structure to save
/// * `path` - Path to save the XLSX file path to
///
/// # Returns
///
/// * `Result<()>` - Ok(()) on success, or an error on failure
pub fn save(workbook: &Spreadsheet, path: impl AsRef<Path>) -> Result<()> {
    xlsx::write(workbook, path)?;
    Ok(())
}

/// Opens and reads an XLS file, returning its contents as a umya_spreadsheet::Spreadsheet structure.
///
/// # Arguments
///
/// * `path` - Path to the XLS file to open
///
/// # Returns
///
/// * `Result<umya_spreadsheet::Spreadsheet>` - umya_spreadsheet::Spreadsheet structure containing the file contents on success, or an error on failure
pub fn open(path: impl AsRef<Path>) -> Result<Spreadsheet> {
    let mut compound_file = cfb::open(path)?;

    let mut stream = compound_file.open_stream("/Workbook")?;

    let book_records = Records::read(&mut stream)?;

    let mut sheets = Vec::new();
    let mut encoding = UTF_16LE;
    let mut sst = Vec::new();
    let mut global = Global::default();

    let mut workbook = new_file_empty_worksheet();

    #[cfg(feature = "tracing")]
    let mut book_ignores = HashMap::new();

    for record in book_records.0 {
        #[cfg(feature = "tracing")]
        let rname = &record.to_string();

        match record {
            Record::Bof(data) => {
                #[cfg(feature = "tracing")]
                tracing::info!("Workbook [{}] {:?}\n", rname, data);

                if !matches!(data.stream_type, StreamType::Workbook) {
                    let error = Error::StreamType {
                        expect: StreamType::Workbook,
                        actual: data.stream_type,
                    };

                    #[cfg(feature = "tracing")]
                    tracing::error!("Workbook [{}] {}\n", rname, error);

                    return Err(error);
                }
            }
            Record::Boundsheet8(mut data) => {
                data.decode(encoding);

                #[cfg(feature = "tracing")]
                tracing::info!("Workbook [{}] {:?}\n", rname, data);

                // just worksheet
                if matches!(data.r#type, SheetType::Worksheet) {
                    sheets.push(data);
                }
            }
            Record::CodePage(mut data) => {
                data.decode();
                let Some(enc) = data.encoding else {
                    let error = Error::CodePage(data.value);

                    #[cfg(feature = "tracing")]
                    tracing::error!("Workbook [{}] {}\n", rname, error);

                    return Err(error);
                };

                #[cfg(feature = "tracing")]
                tracing::info!("Workbook [{}] {:?}\n", rname, data);

                encoding = enc;
            }
            Record::Date1904(data) => {
                #[cfg(feature = "tracing")]
                tracing::info!("Workbook [{}] {:?}\n", rname, data);

                global.date1904 = data.is1904;
            }
            Record::FilePass(_data) => {
                let error = Error::FillPass;

                #[cfg(feature = "tracing")]
                tracing::error!("Workbook [{}] {:?}, {}\n", rname, _data, error);

                return Err(error);
            }
            Record::Font(mut data) => {
                data.decode(encoding);

                #[cfg(feature = "tracing")]
                {
                    let ifnt = if global.fonts.len() < 4 {
                        global.fonts.len()
                    } else {
                        global.fonts.len() + 1
                    };
                    tracing::info!("Workbook [{}] ifnt: {}, {:?}\n", rname, ifnt, data);
                }

                global.fonts.push(data);
            }
            Record::Format(mut data) => {
                data.decode(encoding);

                #[cfg(feature = "tracing")]
                tracing::info!("Workbook [{}] {:?}\n", rname, data);

                global.formats.insert(data.ifmt, data.code);
            }
            Record::Palette(mut data) => {
                data.decode();

                #[cfg(feature = "tracing")]
                tracing::info!("Workbook [{}] {:?}\n", rname, data);

                global.palette.replace(data.colors);
            }
            Record::Sst(mut data) => {
                data.decode(encoding)?;

                #[cfg(feature = "tracing")]
                tracing::info!("Workbook [{}] {:?}\n", rname, data);

                sst.extend(data.strs);
            }
            Record::Style(mut data) => {
                data.decode(encoding);

                #[cfg(feature = "tracing")]
                tracing::info!("Workbook [{}] {:?}\n", rname, data);
                // global.styles.push(data);
            }
            Record::XF(data) => {
                #[cfg(feature = "tracing")]
                tracing::info!(
                    "Workbook [{}] ixfe: {}, {:?}\n",
                    rname,
                    global.xfs.len(),
                    data
                );
                global.xfs.push(data);
            }
            Record::XFExt(_data) => {
                #[cfg(feature = "tracing")]
                tracing::info!("Workbook [{}] {:?}\n", rname, _data);
            }
            Record::Eof(_data) => {
                #[cfg(feature = "tracing")]
                tracing::info!("Workbook [{}] {:?}\n", rname, _data);

                break;
            }
            Record::Ignore(_data) => {
                #[cfg(feature = "tracing")]
                {
                    *book_ignores.entry(_data.r#type).or_insert(0) += 1;
                }
            }
            _ => (),
        }
    }

    #[cfg(feature = "tracing")]
    {
        use record::RECORDS;
        tracing::info!("\n");
        for (k, v) in book_ignores {
            let Some(name) = RECORDS.get(&k) else {
                continue;
            };
            #[cfg(feature = "tracing")]
            tracing::info!("Workbook [Ignored] occurs:{:2}, 0x{:04X} - {}", v, k, name);
        }
        tracing::info!("\n");
    }

    for sheet in sheets {
        stream.seek(SeekFrom::Start(sheet.pos as u64))?;
        let sheet_records = Records::read(&mut stream)?;

        let worksheet = workbook.new_sheet(&sheet.name).map_err(Error::msg)?;
        worksheet.set_sheet_state(sheet.state.state().to_string());

        #[cfg(feature = "tracing")]
        let mut sheet_ignores = HashMap::new();

        for record in sheet_records.0 {
            #[cfg(feature = "tracing")]
            let rname = &record.to_string();

            match record {
                Record::Bof(_data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, _data);
                }
                Record::Blank(_data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, _data);
                }
                Record::BoolErr(data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, data);

                    handle_cell(
                        worksheet,
                        &global,
                        Value::String(&data.value()),
                        data.ixfe.into(),
                        data.row.into(),
                        data.col.into(),
                    )?;
                }
                Record::ColInfo(data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, data);

                    for c in data.col_min..=data.col_max {
                        let col = worksheet.get_column_dimension_by_number_mut(&(c + 1).into());

                        col.set_hidden(data.info.hidden())
                            .set_best_fit(data.info.best_fit());
                        if data.info.user_set() {
                            col.set_width(data.width as f64 / 256.);
                        }
                    }
                }
                Record::DefaultColWidth(data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, data);

                    // Set default column width for the worksheet
                    // Python xlrd: defcolwidth is in characters
                    // Only set if StandardWidth hasn't been set (StandardWidth takes precedence)
                    // umya-spreadsheet expects width in characters
                    let current = worksheet.get_sheet_format_properties().get_default_column_width();
                    if *current == 0.0 {
                        let default_width = data.width as f64;
                        worksheet
                            .get_sheet_format_properties_mut()
                            .set_default_column_width(default_width);
                    }
                }
                Record::StandardWidth(data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, data);

                    // Set standard column width for the worksheet
                    // Python xlrd: standardwidth is in 1/256ths of a character
                    // This takes precedence over DefaultColWidth
                    // umya-spreadsheet expects width in characters
                    let standard_width = data.width as f64 / 256.0;
                    worksheet
                        .get_sheet_format_properties_mut()
                        .set_default_column_width(standard_width);
                }
                Record::DefaultRowHeight(data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, data);

                    // Set default row height for the worksheet
                    // Height is in twips (1/20th of a point), convert to points
                    let default_height = data.height as f64 / 20.0;
                    worksheet
                        .get_sheet_format_properties_mut()
                        .set_default_row_height(default_height);
                }
                Record::Dimensions(_data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, _data);
                }
                Record::Label(mut data) => {
                    data.decode(encoding);

                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, data);

                    handle_cell(
                        worksheet,
                        &global,
                        Value::String(&data.content),
                        data.ixfe.into(),
                        data.row.into(),
                        data.col.into(),
                    )?;
                }
                Record::LabelSST(data) => {
                    let content = sst.get(data.isst as usize).ok_or(Error::msg("sst get"))?;

                    #[cfg(feature = "tracing")]
                    tracing::info!(
                        "{} [{}] {:?}, content: {}\n",
                        &sheet.name,
                        rname,
                        data,
                        content
                    );

                    handle_cell(
                        worksheet,
                        &global,
                        Value::String(content),
                        data.ixfe.into(),
                        data.row.into(),
                        data.col.into(),
                    )?;
                }
                Record::MergeCells(data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, data);

                    data.refs
                        .iter()
                        .map(|ref8| {
                            format!(
                                "{}:{}",
                                coordinate_from_index(
                                    &(ref8.col_min + 1).into(),
                                    &(ref8.row_min + 1).into()
                                ),
                                coordinate_from_index(
                                    &(ref8.col_max + 1).into(),
                                    &(ref8.row_max + 1).into()
                                )
                            )
                        })
                        .for_each(|range| {
                            worksheet.add_merge_cells(range);
                        });
                }
                Record::MulBlank(_data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, _data);
                }
                Record::MulRk(data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, data);

                    let mut col = 0;
                    for (ixfe, num) in data.values {
                        handle_cell(
                            worksheet,
                            &global,
                            Value::Number(num),
                            ixfe.into(),
                            data.row.into(),
                            (data.col_min + col).into(),
                        )?;
                        col += 1;
                    }
                }
                Record::Number(data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, data);

                    handle_cell(
                        worksheet,
                        &global,
                        Value::Number(data.num),
                        data.ixfe.into(),
                        data.row.into(),
                        data.col.into(),
                    )?;
                }
                Record::Rk(data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, data);

                    handle_cell(
                        worksheet,
                        &global,
                        Value::Number(data.num),
                        data.ixfe.into(),
                        data.row.into(),
                        data.col.into(),
                    )?;
                }
                Record::RowInfo(data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, data);

                    let row = worksheet.get_row_dimension_mut(&(data.row + 1).into());
                    if data.info.hidden() {
                        row.set_hidden(true);
                    } else {
                        // issure with `set_height` not working
                        row.set_height(data.height as f64 / 20.);
                    }
                    row.set_thick_bot(data.info.top_bdr());
                }
                Record::Eof(_data) => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("{} [{}] {:?}\n", sheet.name, rname, _data);

                    break;
                }
                Record::Ignore(_data) => {
                    #[cfg(feature = "tracing")]
                    {
                        *sheet_ignores.entry(_data.r#type).or_insert(0) += 1;
                    }
                }
                _ => (),
            }
        }

        #[cfg(feature = "tracing")]
        {
            use record::RECORDS;
            tracing::info!("\n");
            for (k, v) in sheet_ignores {
                let Some(name) = RECORDS.get(&k) else {
                    continue;
                };
                tracing::info!("Worksheet [Ignored] occurs:{:2}, 0x{:04X} - {}", v, k, name);
            }
            tracing::info!("\n");
        }
    }

    Ok(workbook)
}

fn handle_cell(
    worksheet: &mut Worksheet,
    global: &Global,
    mut value: Value,
    ixfe: usize,
    row: u32,
    col: u32,
) -> Result<()> {
    let cell = worksheet.get_cell_mut((col + 1, row + 1));

    let xstyle = cell.get_style_mut();
    let isdt = handle_style(xstyle, global, ixfe);

    // handle 1904 system
    if let Value::Number(ref mut num) = value
        && global.date1904
        && isdt
    {
        *num += 1462.0;
    }

    match value {
        Value::String(s) => {
            cell.set_value_string(s);
        }
        Value::Number(n) => {
            cell.set_value_number(n);
        }
    }

    Ok(())
}

fn handle_color(color: &mut Color, palette: &Option<Vec<String>>, icv: u16) {
    if icv == 0x7FFF {
        return;
    }
    if let Some(palette) = palette
        && let Some(argb) = palette.get(icv as usize)
    {
        color.set_argb(argb);
    } else {
        color.set_indexed(icv as u32);
    }
}

fn handle_style(xstyle: &mut Style, global: &Global, ixfe: usize) -> bool {
    let mut isdt = false;
    if let Some(xf) = global.xfs.get(ixfe) {
        // number format
        if matches!(xf.ifmt, 1..=4 | 9..=22 | 27..=40 | 45..=62 | 67..=81) {
            isdt = matches!(xf.ifmt, 14..=22 | 27..=36 | 45..=47 | 50..=58 | 71..=81);
            xstyle
                .get_number_format_mut()
                .set_number_format_id(xf.ifmt.into());
        } else if let Some(format) = global.formats.get(&xf.ifmt.into()) {
            isdt = ["y", "m", "d", "h", "s", "a", "p"]
                .iter()
                .any(|s| format.to_lowercase().contains(s));
            xstyle.get_number_format_mut().set_format_code(format);
        }

        // font
        let ifnt = if xf.ifnt < 4 { xf.ifnt } else { xf.ifnt - 1 };
        if let Some(sfont) = global.fonts.get(ifnt as usize) {
            let xfont = xstyle.get_font_mut();
            xfont
                .set_size((sfont.height as f64) / 20.0)
                .set_name(&sfont.name)
                .set_family(sfont.family.into())
                .set_charset(sfont.charset.into())
                .set_bold(sfont.bold)
                .set_italic(sfont.info.italic())
                .set_strikethrough(sfont.info.strike_out())
                .set_underline(sfont.underline.to_string());
            handle_color(xfont.get_color_mut(), &global.palette, sfont.icv);
        }

        // protection
        let protection = xstyle.get_protection_mut();
        protection.set_locked(xf.protection.locked());
        protection.set_hidden(xf.protection.hidden());

        // alignment
        let alignment = xstyle.get_alignment_mut();
        alignment.set_horizontal(xf.alignment.horiz_align().into());
        alignment.set_vertical(xf.alignment.vert_align().into());
        alignment.set_wrap_text(xf.alignment.warp_text());
        alignment.set_text_rotation(xf.alignment.text_rotation().into());

        // borders
        let borders = xstyle.get_borders_mut();
        let border_left = borders.get_left_mut();
        border_left.set_border_style(xf.borders.left_style().to_string());
        handle_color(
            border_left.get_color_mut(),
            &global.palette,
            xf.borders.left_icv().into(),
        );
        let border_top = borders.get_top_mut();
        border_top.set_border_style(xf.borders.top_style().to_string());
        handle_color(
            border_top.get_color_mut(),
            &global.palette,
            xf.borders.top_icv().into(),
        );
        let border_right = borders.get_right_mut();
        border_right.set_border_style(xf.borders.right_style().to_string());
        handle_color(
            border_right.get_color_mut(),
            &global.palette,
            xf.borders.right_icv().into(),
        );
        let border_bottom = borders.get_bottom_mut();
        border_bottom.set_border_style(xf.borders.bottom_style().to_string());
        handle_color(
            border_bottom.get_color_mut(),
            &global.palette,
            xf.borders.bottom_icv().into(),
        );

        if xf.borders.diagonal_type() > 0 {
            borders.set_diagonal_down(true);
        }
        if xf.borders.diagonal_type() > 1 {
            borders.set_diagonal_up(true);
        }
        let border_diagonal = borders.get_diagonal_mut();
        border_diagonal.set_border_style(xf.borders.diagonal_style().to_string());
        handle_color(
            border_diagonal.get_color_mut(),
            &global.palette,
            xf.borders.diagonal_icv().into(),
        );

        // fill
        let fill_pattern = xstyle.get_fill_mut().get_pattern_fill_mut();
        fill_pattern.set_pattern_type(xf.fill.pattern().into());
        handle_color(
            fill_pattern.get_foreground_color_mut(),
            &global.palette,
            xf.fill.fore_icv().into(),
        );
        handle_color(
            fill_pattern.get_background_color_mut(),
            &global.palette,
            xf.fill.back_icv().into(),
        );
    }
    isdt
}
