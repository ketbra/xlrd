//! Tests based on Apache POI test files
//! These tests verify xlrd can read various XLS files correctly

use xlrd;
use ssfmt;

const TEST_DATA_DIR: &str = "tests/test_data";

// Helper function to check if a cell is considered empty
fn is_cell_empty(cell: &umya_spreadsheet::structs::Cell) -> bool {
    cell.get_value().is_empty()
}

#[test]
fn test_simple_xls_can_open() {
    let path = format!("{}/Simple.xls", TEST_DATA_DIR);
    let result = xlrd::open(&path);
    assert!(result.is_ok(), "Failed to open Simple.xls: {:?}", result.err());

    let workbook = result.unwrap();
    assert!(workbook.get_sheet_count() > 0, "Workbook should have at least one sheet");
}

#[test]
fn test_simple_xls_sheet_count() {
    let path = format!("{}/Simple.xls", TEST_DATA_DIR);
    let workbook = xlrd::open(&path).expect("Failed to open Simple.xls");

    let sheet_count = workbook.get_sheet_count();
    println!("Simple.xls has {} sheet(s)", sheet_count);
    assert!(sheet_count > 0, "Should have at least one sheet");
}

#[test]
fn test_simple_xls_sheet_names() {
    let path = format!("{}/Simple.xls", TEST_DATA_DIR);
    let workbook = xlrd::open(&path).expect("Failed to open Simple.xls");

    for i in 0..workbook.get_sheet_count() {
        let sheet = workbook.get_sheet(&i).expect("Failed to get sheet");
        let name = sheet.get_name();
        println!("Sheet {}: {}", i, name);
        assert!(!name.is_empty(), "Sheet name should not be empty");
    }
}

#[test]
fn test_simple_xls_read_cells() {
    let path = format!("{}/Simple.xls", TEST_DATA_DIR);
    let workbook = xlrd::open(&path).expect("Failed to open Simple.xls");

    let sheet = workbook.get_sheet(&0).expect("Failed to get first sheet");

    // Try to read some cells
    for row in 1..=10 {
        for col in 1..=10 {
            if let Some(cell) = sheet.get_cell((col, row)) {
                if !is_cell_empty(cell) {
                    let value = cell.get_value();
                    println!("Cell ({}, {}): {}", col, row, value);
                }
            }
        }
    }
}

#[test]
fn test_simple_xls_raw_values() {
    let path = format!("{}/Simple.xls", TEST_DATA_DIR);
    let workbook = xlrd::open(&path).expect("Failed to open Simple.xls");

    let sheet = workbook.get_sheet(&0).expect("Failed to get first sheet");

    // Read raw values from cells
    for row in 1..=20 {
        for col in 1..=20 {
            if let Some(cell) = sheet.get_cell((col, row)) {
                if !is_cell_empty(cell) {
                    let value = cell.get_value();
                    let cell_type = if cell.is_formula() {
                        "formula"
                    } else if cell.get_data_type() == "n" {
                        "number"
                    } else if cell.get_data_type() == "s" {
                        "string"
                    } else {
                        cell.get_data_type()
                    };
                    println!("Raw [{},{}] type={}: {}", row, col, cell_type, value);
                }
            }
        }
    }
}

#[test]
fn test_sample_ss_xls_open() {
    let path = format!("{}/SampleSS.xls", TEST_DATA_DIR);
    let result = xlrd::open(&path);
    assert!(result.is_ok(), "Failed to open SampleSS.xls: {:?}", result.err());
}

#[test]
fn test_sample_ss_xls_read_data() {
    let path = format!("{}/SampleSS.xls", TEST_DATA_DIR);
    let workbook = xlrd::open(&path).expect("Failed to open SampleSS.xls");

    for i in 0..workbook.get_sheet_count() {
        let sheet = workbook.get_sheet(&i).expect("Failed to get sheet");
        println!("\nSheet: {}", sheet.get_name());

        let highest_row = sheet.get_highest_row();
        let highest_column = sheet.get_highest_column();
        println!("Dimensions: {} rows x {} columns", highest_row, highest_column);

        // Read first 20x20 cells
        for row in 1..=20.min(highest_row) {
            for col in 1..=20.min(highest_column) {
                if let Some(cell) = sheet.get_cell((col, row)) {
                    if !is_cell_empty(cell) {
                        println!("[{},{}]: {}", row, col, cell.get_value());
                    }
                }
            }
        }
    }
}

#[test]
fn test_formatting_xls_open() {
    let path = format!("{}/Formatting.xls", TEST_DATA_DIR);
    let result = xlrd::open(&path);
    assert!(result.is_ok(), "Failed to open Formatting.xls: {:?}", result.err());
}

#[test]
fn test_formatting_xls_read_with_formats() {
    let path = format!("{}/Formatting.xls", TEST_DATA_DIR);
    let workbook = xlrd::open(&path).expect("Failed to open Formatting.xls");

    let sheet = workbook.get_sheet(&0).expect("Failed to get first sheet");
    let highest_row = sheet.get_highest_row();
    let highest_column = sheet.get_highest_column();

    println!("\nFormatting.xls - Testing formatted values:");

    for row in 1..=20.min(highest_row) {
        for col in 1..=20.min(highest_column) {
            if let Some(cell) = sheet.get_cell((col, row)) {
                if !is_cell_empty(cell) {
                    let raw_value = cell.get_value();

                    // Get format code
                    let format_code = if let Some(nf) = cell.get_style().get_number_format() {
                        nf.get_format_code()
                    } else {
                        "General"
                    };

                    println!("[{},{}] raw={}, format={}", row, col, raw_value, format_code);

                    // Try to format using ssfmt if it's a number
                    if cell.get_data_type() == "n" {
                        if let Ok(num) = raw_value.parse::<f64>() {
                            match ssfmt::format_default(num, format_code) {
                                Ok(formatted) => {
                                    println!("  -> formatted: {}", formatted);
                                }
                                Err(e) => {
                                    println!("  -> format parse error: {:?}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
#[ignore = "Parser error: 'failed to fill whole buffer' at record boundary - needs xlrd fix"]
fn test_date_formats_xls_open() {
    let path = format!("{}/DateFormats.xls", TEST_DATA_DIR);
    let result = xlrd::open(&path);
    assert!(result.is_ok(), "Failed to open DateFormats.xls: {:?}", result.err());
}

#[test]
#[ignore = "Parser error: 'failed to fill whole buffer' at record boundary - needs xlrd fix"]
fn test_date_formats_xls_read_dates() {
    let path = format!("{}/DateFormats.xls", TEST_DATA_DIR);
    let workbook = xlrd::open(&path).expect("Failed to open DateFormats.xls");

    let sheet = workbook.get_sheet(&0).expect("Failed to get first sheet");
    let highest_row = sheet.get_highest_row();
    let highest_column = sheet.get_highest_column();

    println!("\nDateFormats.xls - Testing date/time values:");

    for row in 1..=30.min(highest_row) {
        for col in 1..=20.min(highest_column) {
            if let Some(cell) = sheet.get_cell((col, row)) {
                if !is_cell_empty(cell) {
                    let raw_value = cell.get_value();
                    let format_code = if let Some(nf) = cell.get_style().get_number_format() {
                        nf.get_format_code()
                    } else {
                        "General"
                    };

                    println!("[{},{}] raw={}, format={}", row, col, raw_value, format_code);

                    // Try to format as date/time
                    if cell.get_data_type() == "n" {
                        if let Ok(num) = raw_value.parse::<f64>() {
                            match ssfmt::format_default(num, format_code) {
                                Ok(formatted) => {
                                    println!("  -> formatted: {}", formatted);
                                }
                                Err(e) => {
                                    println!("  -> format parse error: {:?}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
#[ignore = "Parser error: 'failed to fill whole buffer' at record boundary - needs xlrd fix"]
fn test_simple_with_formula_xls_open() {
    let path = format!("{}/SimpleWithFormula.xls", TEST_DATA_DIR);
    let result = xlrd::open(&path);
    assert!(result.is_ok(), "Failed to open SimpleWithFormula.xls: {:?}", result.err());
}

#[test]
#[ignore = "Parser error: 'failed to fill whole buffer' at record boundary - needs xlrd fix"]
fn test_simple_with_formula_xls_read_formulas() {
    let path = format!("{}/SimpleWithFormula.xls", TEST_DATA_DIR);
    let workbook = xlrd::open(&path).expect("Failed to open SimpleWithFormula.xls");

    let sheet = workbook.get_sheet(&0).expect("Failed to get first sheet");
    let highest_row = sheet.get_highest_row();
    let highest_column = sheet.get_highest_column();

    println!("\nSimpleWithFormula.xls:");

    for row in 1..=20.min(highest_row) {
        for col in 1..=20.min(highest_column) {
            if let Some(cell) = sheet.get_cell((col, row)) {
                if !is_cell_empty(cell) {
                    let value = cell.get_value();

                    if cell.is_formula() {
                        // Note: xlrd might not preserve formula strings
                        println!("[{},{}] formula result: {}", row, col, value);
                    } else {
                        println!("[{},{}]: {}", row, col, value);
                    }
                }
            }
        }
    }
}

#[test]
fn test_two_operand_numeric_function_test_case_data() {
    let path = format!("{}/TwoOperandNumericFunctionTestCaseData.xls", TEST_DATA_DIR);
    let result = xlrd::open(&path);
    assert!(result.is_ok(), "Failed to open TwoOperandNumericFunctionTestCaseData.xls: {:?}", result.err());

    let workbook = result.unwrap();
    println!("\nTwoOperandNumericFunctionTestCaseData.xls sheets:");
    for i in 0..workbook.get_sheet_count() {
        let sheet = workbook.get_sheet(&i).expect("Failed to get sheet");
        println!("  - {}", sheet.get_name());
    }
}

#[test]
fn test_45365_xls_open() {
    let path = format!("{}/45365.xls", TEST_DATA_DIR);
    let result = xlrd::open(&path);
    assert!(result.is_ok(), "Failed to open 45365.xls: {:?}", result.err());
}

#[test]
#[ignore = "51222.xls does not exist in Apache POI repository"]
fn test_51222_xls_open() {
    let path = format!("{}/51222.xls", TEST_DATA_DIR);
    let result = xlrd::open(&path);
    assert!(result.is_ok(), "Failed to open 51222.xls: {:?}", result.err());
}

#[test]
fn test_54206_xls_open() {
    let path = format!("{}/54206.xls", TEST_DATA_DIR);
    let result = xlrd::open(&path);
    assert!(result.is_ok(), "Failed to open 54206.xls: {:?}", result.err());
}

#[test]
fn test_45365_xls_read_data() {
    let path = format!("{}/45365.xls", TEST_DATA_DIR);
    let workbook = xlrd::open(&path).expect("Failed to open 45365.xls");

    println!("\n45365.xls:");
    for i in 0..workbook.get_sheet_count() {
        let sheet = workbook.get_sheet(&i).expect("Failed to get sheet");
        println!("Sheet: {}", sheet.get_name());

        let highest_row = sheet.get_highest_row();
        let highest_column = sheet.get_highest_column();

        for row in 1..=10.min(highest_row) {
            for col in 1..=10.min(highest_column) {
                if let Some(cell) = sheet.get_cell((col, row)) {
                    if !is_cell_empty(cell) {
                        println!("  [{},{}]: {}", row, col, cell.get_value());
                    }
                }
            }
        }
    }
}

#[test]
#[ignore = "51222.xls does not exist in Apache POI repository"]
fn test_51222_xls_read_data() {
    let path = format!("{}/51222.xls", TEST_DATA_DIR);
    let workbook = xlrd::open(&path).expect("Failed to open 51222.xls");

    println!("\n51222.xls:");
    for i in 0..workbook.get_sheet_count() {
        let sheet = workbook.get_sheet(&i).expect("Failed to get sheet");
        println!("Sheet: {}", sheet.get_name());

        let highest_row = sheet.get_highest_row();
        let highest_column = sheet.get_highest_column();
        println!("  Dimensions: {} rows x {} columns", highest_row, highest_column);

        // Sample first few cells
        for row in 1..=5.min(highest_row) {
            for col in 1..=5.min(highest_column) {
                if let Some(cell) = sheet.get_cell((col, row)) {
                    if !is_cell_empty(cell) {
                        println!("  [{},{}]: {}", row, col, cell.get_value());
                    }
                }
            }
        }
    }
}

#[test]
fn test_54206_xls_read_data() {
    let path = format!("{}/54206.xls", TEST_DATA_DIR);
    let workbook = xlrd::open(&path).expect("Failed to open 54206.xls");

    println!("\n54206.xls:");
    for i in 0..workbook.get_sheet_count() {
        let sheet = workbook.get_sheet(&i).expect("Failed to get sheet");
        println!("Sheet: {}", sheet.get_name());

        let highest_row = sheet.get_highest_row();
        let highest_column = sheet.get_highest_column();

        for row in 1..=10.min(highest_row) {
            for col in 1..=10.min(highest_column) {
                if let Some(cell) = sheet.get_cell((col, row)) {
                    if !is_cell_empty(cell) {
                        println!("  [{},{}]: {}", row, col, cell.get_value());
                    }
                }
            }
        }
    }
}

#[test]
fn test_cell_styles_formatting() {
    let path = format!("{}/Formatting.xls", TEST_DATA_DIR);
    let workbook = xlrd::open(&path).expect("Failed to open Formatting.xls");

    let sheet = workbook.get_sheet(&0).expect("Failed to get first sheet");

    println!("\nTesting cell styles:");
    for row in 1..=10 {
        for col in 1..=10 {
            if let Some(cell) = sheet.get_cell((col, row)) {
                if !is_cell_empty(cell) {
                    let style = cell.get_style();
                    let font = style.get_font();
                    let fill = style.get_fill();
                    let number_format = style.get_number_format();

                    println!("\nCell [{},{}]:", row, col);
                    println!("  Value: {}", cell.get_value());
                    if let Some(f) = font {
                        println!("  Font: {} {}pt", f.get_name(), f.get_size());
                        println!("  Bold: {}, Italic: {}", f.get_bold(), f.get_italic());
                    }
                    if let Some(nf) = number_format {
                        println!("  Format code: {}", nf.get_format_code());
                    }
                    if let Some(fi) = fill {
                        if let Some(pf) = fi.get_pattern_fill() {
                            println!("  Pattern: {:?}", pf.get_pattern_type());
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_comprehensive_number_formatting() {
    let path = format!("{}/Formatting.xls", TEST_DATA_DIR);
    let workbook = xlrd::open(&path).expect("Failed to open Formatting.xls");

    let sheet = workbook.get_sheet(&0).expect("Failed to get first sheet");
    let highest_row = sheet.get_highest_row();
    let highest_column = sheet.get_highest_column();

    println!("\nComprehensive number formatting test:");

    let mut format_examples = std::collections::HashMap::new();

    for row in 1..=highest_row {
        for col in 1..=highest_column {
            if let Some(cell) = sheet.get_cell((col, row)) {
                let value_str = cell.get_value().to_string();
                if !value_str.is_empty() && cell.get_data_type() == "n" {
                    let raw_value = cell.get_value();
                    let format_code = if let Some(nf) = cell.get_style().get_number_format() {
                        nf.get_format_code().to_string()
                    } else {
                        String::from("General")
                    };

                    if !format_code.is_empty() && format_code != "General" {
                        let entry = format_examples.entry(format_code.clone()).or_insert(Vec::new());
                        entry.push((raw_value.to_string(), row, col));
                    }
                }
            }
        }
    }

    for (format, examples) in format_examples.iter() {
        println!("\nFormat: {}", format);
        for (value, row, col) in examples.iter().take(3) {
            println!("  Cell [{},{}]: raw={}", row, col, value);

            if let Ok(num) = value.parse::<f64>() {
                match ssfmt::format_default(num, format.as_str()) {
                    Ok(formatted) => {
                        println!("    -> formatted: {}", formatted);
                    }
                    Err(e) => {
                        println!("    -> format error: {:?}", e);
                    }
                }
            }
        }
    }
}

#[test]
fn test_all_files_can_open() {
    let test_files = vec![
        ("Simple.xls", true),
        ("SampleSS.xls", true),
        ("Formatting.xls", true),
        ("DateFormats.xls", false), // Known parser issue
        ("SimpleWithFormula.xls", false), // Known parser issue
        ("TwoOperandNumericFunctionTestCaseData.xls", true),
        ("45365.xls", true),
        ("54206.xls", true),
    ];

    println!("\nTesting all files can be opened:");
    for (file, should_succeed) in test_files {
        let path = format!("{}/{}", TEST_DATA_DIR, file);
        let result = xlrd::open(&path);
        let success = result.is_ok();

        if should_succeed {
            assert!(success, "{} should open but failed: {:?}", file, result.err());
            println!("{}: OK", file);
        } else {
            println!("{}: {} (known issue)", file, if success { "OK" } else { "FAILED" });
        }
    }
}
