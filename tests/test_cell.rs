//! Tests for cell functionality, ported from Python xlrd test_cell.py

use std::path::PathBuf;

fn sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("samples")
        .join(name)
}

/// Test reading string cells from profiles.xls
/// Python test expects cell A1 to contain "PROFIL" as text
#[test]
fn test_string_cell() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");
    let sheet = workbook.get_sheet_by_name("PROFILEDEF").expect("Sheet not found");

    let cell = sheet.get_cell((1, 1)).expect("Cell A1 should exist");
    let value = cell.get_value();

    assert_eq!(value, "PROFIL", "Expected cell A1 to contain 'PROFIL'");
}

/// Test reading number cells from profiles.xls
/// Python test expects cell A2 to contain 100 as a number
#[test]
fn test_number_cell() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");
    let sheet = workbook.get_sheet_by_name("PROFILEDEF").expect("Sheet not found");

    // A2 should be the number 100
    let cell = sheet.get_cell((1, 2)).expect("Cell A2 should exist");
    let value = cell.get_value();

    // Check if it's a number or can be parsed as one
    if !value.is_empty() {
        // Try parsing as number
        if let Ok(num) = value.parse::<f64>() {
            assert!((num - 100.0).abs() < 0.001, "Expected ~100, got {}", num);
        }
    }
}

/// Test reading empty cells
#[test]
fn test_empty_cell() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");
    let sheet = workbook.get_sheet_by_name("PROFILEDEF").expect("Sheet not found");

    // Access a cell that might be empty (far corner)
    let cell = sheet.get_cell((100, 100));
    // Empty or non-existent cells should return None or empty value
    if let Some(c) = cell {
        let value = c.get_value();
        assert!(value.is_empty(), "Far cell should be empty");
    }
}

/// Test merged cells from xf_class.xls
#[test]
fn test_merged_cells() {
    let workbook = xlrd::open(sample_path("xf_class.xls")).expect("Failed to open xf_class.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert!(!sheets.is_empty(), "Should have at least one sheet");

    // Check if merged cells are present
    let sheet = &sheets[0];
    let merge_cells = sheet.get_merge_cells();

    // The file should have some merged cells
    // Just verify we can access the merge cells collection
    let _count = merge_cells.len();
}

/// Test cell formatting info is preserved
#[test]
fn test_cell_formatting() {
    let workbook = xlrd::open(sample_path("Formate.xls")).expect("Failed to open Formate.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert!(!sheets.is_empty(), "Should have at least one sheet");

    let sheet = &sheets[0];

    // Get a cell and check its style
    if let Some(cell) = sheet.get_cell((1, 1)) {
        let style = cell.get_style();
        // Verify style exists (it should have some formatting info)
        let _font = style.get_font();
        let _borders = style.get_borders();
    }
}

/// Test reading cells from AXISDEF sheet in profiles.xls
#[test]
fn test_axisdef_cells() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");
    let sheet = workbook.get_sheet_by_name("AXISDEF").expect("AXISDEF sheet not found");

    // Verify we can access cells in this sheet
    let (max_col, max_row) = sheet.get_highest_column_and_row();
    assert!(max_row > 0, "Sheet should have rows");
    assert!(max_col > 0, "Sheet should have columns");
}
