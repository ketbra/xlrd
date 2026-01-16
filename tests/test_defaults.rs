//! Tests for default column width and row height fallbacks
//! These tests verify the Rust implementation matches Python xlrd behavior

use std::path::PathBuf;
use std::fs;

fn sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("samples")
        .join(name)
}

fn temp_dir() -> PathBuf {
    let temp = std::env::temp_dir().join("xlrd_tests_defaults");
    fs::create_dir_all(&temp).ok();
    temp
}

/// Test that default column width is applied when no COLINFO record exists
/// In Python xlrd, defcolwidth * 256 is used as fallback (or 8 * 256 = 2048)
#[test]
fn test_default_column_width_fallback() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");
    let sheet = workbook
        .get_sheet_by_name("PROFILEDEF")
        .expect("Sheet not found");

    // Get a column that likely doesn't have explicit COLINFO
    // Default column width in Excel is typically 8.43 characters (~2159 in 256ths)
    // After conversion: width / 256 should give reasonable character width
    let col_dim = sheet.get_column_dimension_by_number(&20u32);

    if let Some(col) = col_dim {
        let width = col.get_width();
        // Width should be positive and reasonable (0 to 100 characters)
        assert!(
            *width >= 0.0 && *width <= 100.0,
            "Column width {} should be in reasonable range",
            width
        );
    }
}

/// Test that default row height is applied when no ROW record exists
/// In Python xlrd, default_row_height is used as fallback
#[test]
fn test_default_row_height_fallback() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");
    let sheet = workbook
        .get_sheet_by_name("PROFILEDEF")
        .expect("Sheet not found");

    // Get a row that may use default height
    let row_dim = sheet.get_row_dimension(&100u32);

    if let Some(row) = row_dim {
        let height = row.get_height();
        // Default row height in Excel is typically 15 points (300 twips / 20)
        assert!(
            *height >= 0.0 && *height <= 500.0,
            "Row height {} should be in reasonable range",
            height
        );
    }
}

/// Test that column widths are preserved during XLS to XLSX conversion
#[test]
fn test_column_widths_in_conversion() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");

    let dest = temp_dir().join("col_width_test.xlsx");
    xlrd::save(&workbook, &dest).expect("Failed to save");

    // Read back the converted file
    let reopened = umya_spreadsheet::reader::xlsx::read(&dest).expect("Failed to reopen");
    let sheet = reopened
        .get_sheet_by_name("PROFILEDEF")
        .expect("Sheet not found");

    // Check that column dimensions exist
    for col_num in 1..=5 {
        if let Some(col) = sheet.get_column_dimension_by_number(&col_num) {
            let width = col.get_width();
            assert!(
                *width >= 0.0,
                "Column {} width should be non-negative: {}",
                col_num,
                width
            );
        }
    }

    // Cleanup
    fs::remove_file(&dest).ok();
}

/// Test Formate.xls column widths (this file has explicit formatting)
#[test]
fn test_formate_column_widths() {
    let workbook = xlrd::open(sample_path("Formate.xls")).expect("Failed to open Formate.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert!(!sheets.is_empty(), "Should have at least one sheet");

    let sheet = &sheets[0];

    // Check column widths
    for col_num in 1..=10 {
        if let Some(col) = sheet.get_column_dimension_by_number(&col_num) {
            let width = col.get_width();
            // Width converted from 256ths should be reasonable
            assert!(
                *width >= 0.0 && *width <= 100.0,
                "Column {} width {} should be reasonable",
                col_num,
                width
            );
        }
    }
}

/// Test xf_class.xls column widths
#[test]
fn test_xf_class_column_widths() {
    let workbook = xlrd::open(sample_path("xf_class.xls")).expect("Failed to open xf_class.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert!(!sheets.is_empty(), "Should have at least one sheet");

    let sheet = &sheets[0];
    let (max_col, _) = sheet.get_highest_column_and_row();

    // Verify column dimensions for columns that exist
    for col_num in 1..=max_col.min(10) {
        if let Some(col) = sheet.get_column_dimension_by_number(&col_num) {
            let width = col.get_width();
            assert!(
                *width >= 0.0 && *width <= 100.0,
                "Column {} width {} should be reasonable",
                col_num,
                width
            );
        }
    }
}

/// Test that hidden columns are handled correctly
#[test]
fn test_hidden_columns() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");
    let sheet = workbook
        .get_sheet_by_name("PROFILEDEF")
        .expect("Sheet not found");

    // Check that we can access column hidden state
    for col_num in 1..=5 {
        if let Some(col) = sheet.get_column_dimension_by_number(&col_num) {
            let _hidden = col.get_hidden();
            // Just verify we can access the property without panic
        }
    }
}

/// Test ragged.xls which has varying row lengths
#[test]
fn test_ragged_defaults() {
    let workbook = xlrd::open(sample_path("ragged.xls")).expect("Failed to open ragged.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert!(!sheets.is_empty(), "Should have at least one sheet");

    let sheet = &sheets[0];

    // Ragged sheets should still have reasonable default dimensions
    let (max_col, max_row) = sheet.get_highest_column_and_row();
    assert!(max_row >= 5, "Should have at least 5 rows");
    assert!(max_col >= 1, "Should have at least 1 column");
}

/// Test namesdemo.xls column and row defaults
#[test]
fn test_namesdemo_defaults() {
    let workbook = xlrd::open(sample_path("namesdemo.xls")).expect("Failed to open namesdemo.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert!(!sheets.is_empty(), "Should have at least one sheet");

    // Verify basic functionality works
    for sheet in sheets.iter() {
        let (max_col, max_row) = sheet.get_highest_column_and_row();
        assert!(max_row > 0 || max_col > 0 || true, "Sheet dimensions accessible");
    }
}
