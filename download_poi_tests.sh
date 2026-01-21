#!/bin/bash

# Script to download additional POI test files

BASE_URL="https://raw.githubusercontent.com/apache/poi/master/test-data/spreadsheet"
DEST_DIR="tests/test_data"

mkdir -p "$DEST_DIR"

# Download a comprehensive selection of test files
# Core/Simple files
curl -L "$BASE_URL/blankworkbook.xls" -o "$DEST_DIR/blankworkbook.xls"
curl -L "$BASE_URL/empty.xls" -o "$DEST_DIR/empty.xls"
curl -L "$BASE_URL/SimpleChart.xls" -o "$DEST_DIR/SimpleChart.xls"
curl -L "$BASE_URL/SimpleMultiCell.xls" -o "$DEST_DIR/SimpleMultiCell.xls"
curl -L "$BASE_URL/SimpleWithAutofilter.xls" -o "$DEST_DIR/SimpleWithAutofilter.xls"
curl -L "$BASE_URL/SimpleWithChoose.xls" -o "$DEST_DIR/SimpleWithChoose.xls"
curl -L "$BASE_URL/SimpleWithColours.xls" -o "$DEST_DIR/SimpleWithColours.xls"
curl -L "$BASE_URL/SimpleWithComments.xls" -o "$DEST_DIR/SimpleWithComments.xls"
curl -L "$BASE_URL/SimpleWithDataFormat.xls" -o "$DEST_DIR/SimpleWithDataFormat.xls"
curl -L "$BASE_URL/SimpleWithImages.xls" -o "$DEST_DIR/SimpleWithImages.xls"
curl -L "$BASE_URL/SimpleWithPageBreaks.xls" -o "$DEST_DIR/SimpleWithPageBreaks.xls"
curl -L "$BASE_URL/SimpleWithStyling.xls" -o "$DEST_DIR/SimpleWithStyling.xls"

# Formula test case data
curl -L "$BASE_URL/FormulaEvalTestData.xls" -o "$DEST_DIR/FormulaEvalTestData.xls"
curl -L "$BASE_URL/MatrixFormulaEvalTestData.xls" -o "$DEST_DIR/MatrixFormulaEvalTestData.xls"
curl -L "$BASE_URL/BooleanFunctionsTestCaseData.xls" -o "$DEST_DIR/BooleanFunctionsTestCaseData.xls"
curl -L "$BASE_URL/LogicalFunctionsTestCaseData.xls" -o "$DEST_DIR/LogicalFunctionsTestCaseData.xls"
curl -L "$BASE_URL/LookupFunctionsTestCaseData.xls" -o "$DEST_DIR/LookupFunctionsTestCaseData.xls"
curl -L "$BASE_URL/IndexFunctionTestCaseData.xls" -o "$DEST_DIR/IndexFunctionTestCaseData.xls"
curl -L "$BASE_URL/MatchFunctionTestCaseData.xls" -o "$DEST_DIR/MatchFunctionTestCaseData.xls"
curl -L "$BASE_URL/IfFunctionTestCaseData.xls" -o "$DEST_DIR/IfFunctionTestCaseData.xls"
curl -L "$BASE_URL/ComplexFunctionTestCaseData.xls" -o "$DEST_DIR/ComplexFunctionTestCaseData.xls"

# Formatting files
curl -L "$BASE_URL/FormatChoiceTests.xls" -o "$DEST_DIR/FormatChoiceTests.xls"
curl -L "$BASE_URL/ConditionalFormattingSamples.xls" -o "$DEST_DIR/ConditionalFormattingSamples.xls"
curl -L "$BASE_URL/WithConditionalFormatting.xls" -o "$DEST_DIR/WithConditionalFormatting.xls"
curl -L "$BASE_URL/54686_fraction_formats.xls" -o "$DEST_DIR/54686_fraction_formats.xls"

# Date/Time
curl -L "$BASE_URL/1900DateWindowing.xls" -o "$DEST_DIR/1900DateWindowing.xls"
curl -L "$BASE_URL/1904DateWindowing.xls" -o "$DEST_DIR/1904DateWindowing.xls"

# Charts
curl -L "$BASE_URL/WithChart.xls" -o "$DEST_DIR/WithChart.xls"
curl -L "$BASE_URL/WithTwoCharts.xls" -o "$DEST_DIR/WithTwoCharts.xls"
curl -L "$BASE_URL/WithThreeCharts.xls" -o "$DEST_DIR/WithThreeCharts.xls"

# Drawing/Images
curl -L "$BASE_URL/DrawingAndComments.xls" -o "$DEST_DIR/DrawingAndComments.xls"
curl -L "$BASE_URL/SheetWithDrawing.xls" -o "$DEST_DIR/SheetWithDrawing.xls"

# Embedded objects
curl -L "$BASE_URL/WithEmbeddedObjects.xls" -o "$DEST_DIR/WithEmbeddedObjects.xls"
curl -L "$BASE_URL/ole2-embedding.xls" -o "$DEST_DIR/ole2-embedding.xls"

# Hyperlinks
curl -L "$BASE_URL/WithHyperlink.xls" -o "$DEST_DIR/WithHyperlink.xls"
curl -L "$BASE_URL/WithTwoHyperLinks.xls" -o "$DEST_DIR/WithTwoHyperLinks.xls"

# Multiple sheets
curl -L "$BASE_URL/TwoSheetsNoneHidden.xls" -o "$DEST_DIR/TwoSheetsNoneHidden.xls"
curl -L "$BASE_URL/TwoSheetsOneHidden.xls" -o "$DEST_DIR/TwoSheetsOneHidden.xls"

# Common bug fix files (selection)
curl -L "$BASE_URL/35564.xls" -o "$DEST_DIR/35564.xls"
curl -L "$BASE_URL/39634.xls" -o "$DEST_DIR/39634.xls"
curl -L "$BASE_URL/41139.xls" -o "$DEST_DIR/41139.xls"
curl -L "$BASE_URL/42844.xls" -o "$DEST_DIR/42844.xls"
curl -L "$BASE_URL/43493.xls" -o "$DEST_DIR/43493.xls"
curl -L "$BASE_URL/46250.xls" -o "$DEST_DIR/46250.xls"
curl -L "$BASE_URL/47920.xls" -o "$DEST_DIR/47920.xls"
curl -L "$BASE_URL/48968.xls" -o "$DEST_DIR/48968.xls"
curl -L "$BASE_URL/49612.xls" -o "$DEST_DIR/49612.xls"
curl -L "$BASE_URL/50298.xls" -o "$DEST_DIR/50298.xls"
curl -L "$BASE_URL/51143.xls" -o "$DEST_DIR/51143.xls"
curl -L "$BASE_URL/53109.xls" -o "$DEST_DIR/53109.xls"
curl -L "$BASE_URL/55982.xls" -o "$DEST_DIR/55982.xls"
curl -L "$BASE_URL/56450.xls" -o "$DEST_DIR/56450.xls"
curl -L "$BASE_URL/59264.xls" -o "$DEST_DIR/59264.xls"

# Special features
curl -L "$BASE_URL/StringFormulas.xls" -o "$DEST_DIR/StringFormulas.xls"
curl -L "$BASE_URL/SharedFormulaTest.xls" -o "$DEST_DIR/SharedFormulaTest.xls"
curl -L "$BASE_URL/3dFormulas.xls" -o "$DEST_DIR/3dFormulas.xls"
curl -L "$BASE_URL/FormulaRefs.xls" -o "$DEST_DIR/FormulaRefs.xls"
curl -L "$BASE_URL/SingleLetterRanges.xls" -o "$DEST_DIR/SingleLetterRanges.xls"

# Merge cells
curl -L "$BASE_URL/RepeatingRowsCols.xls" -o "$DEST_DIR/RepeatingRowsCols.xls"

# Unicode/DBCS
curl -L "$BASE_URL/DBCSSheetName.xls" -o "$DEST_DIR/DBCSSheetName.xls"
curl -L "$BASE_URL/chinese-provinces.xls" -o "$DEST_DIR/chinese-provinces.xls"

# Edge cases
curl -L "$BASE_URL/NoGutsRecords.xls" -o "$DEST_DIR/NoGutsRecords.xls"
curl -L "$BASE_URL/MissingBits.xls" -o "$DEST_DIR/MissingBits.xls"

echo "Download complete!"
