//! Tests for workbook functionality, ported from Python xlrd test_workbook.py

use std::path::PathBuf;

fn sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("samples")
        .join(name)
}

/// Test that the correct number of sheets is read from profiles.xls
/// Python test expects 5 sheets
#[test]
fn test_nsheets() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");
    let sheet_count = workbook.get_sheet_count();
    assert_eq!(sheet_count, 5, "Expected 5 sheets, got {}", sheet_count);
}

/// Test accessing sheets by name
/// Python test expects sheets: PROFILEDEF, AXISDEF, TRAVERSALCHAINAGE, AXISDATUMLEVELS, PROFILELEVELS
#[test]
fn test_sheet_by_name() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");

    let expected_names = [
        "PROFILEDEF",
        "AXISDEF",
        "TRAVERSALCHAINAGE",
        "AXISDATUMLEVELS",
        "PROFILELEVELS",
    ];

    for name in expected_names {
        let sheet = workbook.get_sheet_by_name(name);
        assert!(sheet.is_some(), "Sheet '{}' not found", name);
    }
}

/// Test accessing sheets by index
#[test]
fn test_sheet_by_index() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");

    // umya-spreadsheet uses 0-based indexing internally but get_sheet returns an iterator
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert_eq!(sheets.len(), 5, "Expected 5 sheets");

    let expected_names = [
        "PROFILEDEF",
        "AXISDEF",
        "TRAVERSALCHAINAGE",
        "AXISDATUMLEVELS",
        "PROFILELEVELS",
    ];

    for (i, expected) in expected_names.iter().enumerate() {
        assert_eq!(
            sheets[i].get_name(),
            *expected,
            "Sheet at index {} should be named '{}'",
            i,
            expected
        );
    }
}

/// Test sheet names collection
#[test]
fn test_sheet_names() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");

    let names: Vec<_> = workbook
        .get_sheet_collection()
        .iter()
        .map(|s| s.get_name().to_string())
        .collect();

    let expected = vec![
        "PROFILEDEF",
        "AXISDEF",
        "TRAVERSALCHAINAGE",
        "AXISDATUMLEVELS",
        "PROFILELEVELS",
    ];

    assert_eq!(names, expected);
}

/// Test iteration over sheets
#[test]
fn test_iter_sheets() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");

    let mut count = 0;
    for sheet in workbook.get_sheet_collection().iter() {
        assert!(!sheet.get_name().is_empty(), "Sheet name should not be empty");
        count += 1;
    }

    assert_eq!(count, 5, "Should iterate over 5 sheets");
}
