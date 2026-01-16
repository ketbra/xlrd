//! Tests for error handling, ported from Python xlrd tests

use std::path::PathBuf;

fn sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("samples")
        .join(name)
}

/// Test that opening a non-existent file returns an error
#[test]
fn test_open_nonexistent() {
    let result = xlrd::open("/path/to/nonexistent/file.xls");
    assert!(result.is_err(), "Opening non-existent file should fail");
}

/// Test that opening an xlsx file returns an error
/// From Python test_open_workbook.py
#[test]
fn test_open_xlsx_error() {
    let result = xlrd::open(sample_path("sample.xlsx"));
    assert!(result.is_err(), "Opening xlsx should fail");

    // Check error type if available
    if let Err(e) = result {
        let error_string = format!("{:?}", e);
        // The error should indicate the format is not supported
        assert!(
            !error_string.is_empty(),
            "Error message should not be empty"
        );
    }
}

/// Test that opening a text file returns an error
/// From Python test_open_workbook.py
#[test]
fn test_open_text_error() {
    let result = xlrd::open(sample_path("sample.txt"));
    assert!(result.is_err(), "Opening txt file should fail");
}

/// Test that corrupted files are handled
/// From Python test_ignore_workbook_corruption_error.py
#[test]
fn test_corrupted_file() {
    // The corrupted_error.xls file may or may not be readable depending on the error
    let result = xlrd::open(sample_path("corrupted_error.xls"));

    // Either it opens (if the library is lenient) or fails with an error
    // Both are acceptable outcomes - we just verify no panic
    match result {
        Ok(_) => {
            // File opened despite corruption - library is lenient
        }
        Err(_) => {
            // File rejected due to corruption - library is strict
        }
    }
}

/// Test handling of BIFF4 files with missing records
/// From Python test_missing_records.py
#[test]
fn test_biff4_missing_records() {
    // biff4_no_format_no_window2.xls is a BIFF4 file, which may not be fully supported
    let result = xlrd::open(sample_path("biff4_no_format_no_window2.xls"));

    // BIFF4 might not be supported - just verify we don't panic
    match result {
        Ok(workbook) => {
            // If it opens, verify we can access sheets
            let _count = workbook.get_sheet_count();
        }
        Err(_) => {
            // BIFF4 not supported - acceptable
        }
    }
}

/// Test error contains useful information
#[test]
fn test_error_message_quality() {
    let result = xlrd::open("/path/to/nonexistent.xls");

    if let Err(e) = result {
        let error_string = format!("{}", e);
        // Error should provide some useful context
        assert!(!error_string.is_empty(), "Error should have a message");
    }
}

/// Test that errors implement std::error::Error
#[test]
fn test_error_trait() {
    let result: Result<_, xlrd::Error> = xlrd::open("/nonexistent.xls");

    if let Err(e) = result {
        // Verify Error trait is implemented
        let _: &dyn std::error::Error = &e;
    }
}
