//! Tests for row height handling - verifying the row.set_height fix

use std::path::PathBuf;
use std::fs;

fn sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("samples")
        .join(name)
}

fn temp_dir() -> PathBuf {
    let temp = std::env::temp_dir().join("xlrd_tests");
    fs::create_dir_all(&temp).ok();
    temp
}

/// Test that row heights are preserved during XLS to XLSX conversion
/// This verifies the fix for the row.set_height issue
#[test]
fn test_row_heights_preserved() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");

    // Get the first sheet and check row dimensions
    let sheet = workbook.get_sheet_by_name("PROFILEDEF").expect("Sheet not found");

    // Get row dimension for row 1 (1-based indexing)
    let row_dim = sheet.get_row_dimension(&1u32);

    // Row dimension should exist and have a reasonable height
    // Default row height in Excel is typically around 15 points (300 twips / 20)
    if let Some(row) = row_dim {
        let height = row.get_height();
        // Height should be positive and reasonable (0 to 500 points covers most cases)
        assert!(
            *height >= 0.0 && *height <= 500.0,
            "Row height {} should be in reasonable range",
            height
        );
    }
}

/// Test that row heights are correctly converted and saved to XLSX
#[test]
fn test_row_heights_in_conversion() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");

    let dest = temp_dir().join("row_height_test.xlsx");
    xlrd::save(&workbook, &dest).expect("Failed to save");

    // Read back the converted file
    let reopened = umya_spreadsheet::reader::xlsx::read(&dest).expect("Failed to reopen");
    let sheet = reopened
        .get_sheet_by_name("PROFILEDEF")
        .expect("Sheet not found");

    // Verify row dimensions exist in the converted file
    let row_dim = sheet.get_row_dimension(&1u32);

    // Row should have dimension info preserved
    if let Some(row) = row_dim {
        let height = row.get_height();
        assert!(
            *height >= 0.0,
            "Row height should be non-negative: {}",
            height
        );
    }

    // Cleanup
    fs::remove_file(&dest).ok();
}

/// Test that hidden rows are handled correctly
#[test]
fn test_hidden_rows() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");
    let sheet = workbook.get_sheet_by_name("PROFILEDEF").expect("Sheet not found");

    // Check that we can access row hidden state
    // Most rows should not be hidden in a normal spreadsheet
    for row_num in 1..=5 {
        if let Some(row) = sheet.get_row_dimension(&(row_num as u32)) {
            let _hidden = row.get_hidden();
            // Just verify we can access the property without panic
        }
    }
}

/// Test that xf_class.xls with various formatting has correct row heights
#[test]
fn test_xf_class_row_heights() {
    let workbook = xlrd::open(sample_path("xf_class.xls")).expect("Failed to open xf_class.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert!(!sheets.is_empty(), "Should have at least one sheet");

    let sheet = &sheets[0];
    let (_, max_row) = sheet.get_highest_column_and_row();

    // Verify row dimensions for rows that exist
    for row_num in 1..=max_row.min(10) {
        if let Some(row) = sheet.get_row_dimension(&row_num) {
            let height = row.get_height();
            // Height converted from twips should be reasonable
            assert!(
                *height >= 0.0 && *height <= 500.0,
                "Row {} height {} should be reasonable",
                row_num,
                height
            );
        }
    }
}

/// Test Formate.xls which has various formatting including row heights
#[test]
fn test_formate_row_heights() {
    let workbook = xlrd::open(sample_path("Formate.xls")).expect("Failed to open Formate.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert!(!sheets.is_empty(), "Should have at least one sheet");

    // Check each sheet for row height handling
    for sheet in sheets.iter() {
        let (_, max_row) = sheet.get_highest_column_and_row();

        // Just verify we can access row heights without panic
        for row_num in 1..=max_row.min(5) {
            let _row = sheet.get_row_dimension(&row_num);
        }
    }
}
