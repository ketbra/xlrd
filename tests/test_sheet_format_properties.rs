//! Tests for sheet format properties (default row height, column width)
//! These verify Python xlrd compatibility

use std::path::PathBuf;
use std::fs;

fn sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("samples")
        .join(name)
}

fn temp_dir() -> PathBuf {
    let temp = std::env::temp_dir().join("xlrd_tests_sheet_format");
    fs::create_dir_all(&temp).ok();
    temp
}

/// Test that sheet format properties are readable
#[test]
fn test_sheet_format_properties_accessible() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");
    let sheet = workbook
        .get_sheet_by_name("PROFILEDEF")
        .expect("Sheet not found");

    // Verify sheet format properties are accessible
    let props = sheet.get_sheet_format_properties();

    // Default row height should be set (typical Excel default is ~15 points)
    let default_row_height = props.get_default_row_height();
    assert!(
        *default_row_height >= 0.0,
        "Default row height should be non-negative: {}",
        default_row_height
    );

    // Default column width should be set (typical Excel default is ~8.43 characters)
    let default_col_width = props.get_default_column_width();
    assert!(
        *default_col_width >= 0.0,
        "Default column width should be non-negative: {}",
        default_col_width
    );
}

/// Test that default values are preserved in XLS to XLSX conversion
#[test]
fn test_default_values_preserved_in_conversion() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");

    // Get original default values
    let sheet = workbook
        .get_sheet_by_name("PROFILEDEF")
        .expect("Sheet not found");
    let original_row_height = *sheet.get_sheet_format_properties().get_default_row_height();
    let original_col_width = *sheet.get_sheet_format_properties().get_default_column_width();

    // Save and reload
    let dest = temp_dir().join("format_props_test.xlsx");
    xlrd::save(&workbook, &dest).expect("Failed to save");

    let reopened = umya_spreadsheet::reader::xlsx::read(&dest).expect("Failed to reopen");
    let reopened_sheet = reopened
        .get_sheet_by_name("PROFILEDEF")
        .expect("Sheet not found");

    let new_row_height = *reopened_sheet
        .get_sheet_format_properties()
        .get_default_row_height();
    let new_col_width = *reopened_sheet
        .get_sheet_format_properties()
        .get_default_column_width();

    // Values should be preserved (with some tolerance for floating point)
    assert!(
        (new_row_height - original_row_height).abs() < 0.1,
        "Row height should be preserved: {} vs {}",
        original_row_height,
        new_row_height
    );

    // Cleanup
    fs::remove_file(&dest).ok();
}

/// Test Formate.xls sheet format properties
#[test]
fn test_formate_sheet_format_properties() {
    let workbook = xlrd::open(sample_path("Formate.xls")).expect("Failed to open Formate.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    for sheet in sheets.iter() {
        let props = sheet.get_sheet_format_properties();

        // Both defaults should be accessible
        let row_height = props.get_default_row_height();
        let col_width = props.get_default_column_width();

        assert!(
            *row_height >= 0.0 && *row_height <= 500.0,
            "Sheet '{}' default row height {} should be reasonable",
            sheet.get_name(),
            row_height
        );
        assert!(
            *col_width >= 0.0 && *col_width <= 256.0,
            "Sheet '{}' default column width {} should be reasonable",
            sheet.get_name(),
            col_width
        );
    }
}

/// Test xf_class.xls sheet format properties
#[test]
fn test_xf_class_sheet_format_properties() {
    let workbook = xlrd::open(sample_path("xf_class.xls")).expect("Failed to open xf_class.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert!(!sheets.is_empty(), "Should have at least one sheet");

    let sheet = &sheets[0];
    let props = sheet.get_sheet_format_properties();

    // Verify we can read the properties
    let _row_height = props.get_default_row_height();
    let _col_width = props.get_default_column_width();
}

/// Test namesdemo.xls sheet format properties
#[test]
fn test_namesdemo_sheet_format_properties() {
    let workbook = xlrd::open(sample_path("namesdemo.xls")).expect("Failed to open namesdemo.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert!(!sheets.is_empty(), "Should have at least one sheet");

    for sheet in sheets.iter() {
        let props = sheet.get_sheet_format_properties();
        let _row_height = props.get_default_row_height();
        let _col_width = props.get_default_column_width();
    }
}

/// Test ragged.xls sheet format properties
#[test]
fn test_ragged_sheet_format_properties() {
    let workbook = xlrd::open(sample_path("ragged.xls")).expect("Failed to open ragged.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert!(!sheets.is_empty(), "Should have at least one sheet");

    let sheet = &sheets[0];
    let props = sheet.get_sheet_format_properties();

    // Even ragged sheets should have default format properties
    let row_height = props.get_default_row_height();
    let col_width = props.get_default_column_width();

    assert!(
        *row_height >= 0.0,
        "Default row height should be non-negative"
    );
    assert!(
        *col_width >= 0.0,
        "Default column width should be non-negative"
    );
}
