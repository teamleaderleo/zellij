// Fieldwork execution-only tests for zellij-org/zellij draft PR 4800.
// Appended to zellij-server/src/panes/unit/grid_tests.rs by the owned-fork workflow.

use unicode_width::UnicodeWidthChar;

fn fieldwork_scalar_width(text: &str) -> usize {
    text.chars()
        .map(|c| UnicodeWidthChar::width(c).unwrap_or(0))
        .sum()
}

#[test]
fn fieldwork_telugu_critical_cluster_preserves_text_but_diverges_from_scalar_columns() {
    let text = "ద్యం";
    let grid = create_grid_with_content(text);

    assert_eq!(row_text(&grid.viewport[0]), text, "draft must retain the full grapheme text");
    assert_eq!(grid.cursor.x, 1, "draft internal cursor policy gives ద్యం one column");
    assert_eq!(fieldwork_scalar_width(text), 3, "unicode-width scalar policy gives ద్యం three columns");
}

#[test]
fn fieldwork_telugu_control_cluster_has_no_width_drift() {
    let text = "వ్రా";
    let grid = create_grid_with_content(text);

    assert_eq!(row_text(&grid.viewport[0]), text);
    assert_eq!(grid.cursor.x, 2, "draft internal cursor policy gives వ్రా two columns");
    assert_eq!(fieldwork_scalar_width(text), 2, "scalar policy agrees for the control cluster");
}

#[test]
fn fieldwork_telugu_reporter_residual_has_nine_internal_columns_and_eleven_scalar_columns() {
    let text = "ఒక పద్యం వ్రాయి";
    let grid = create_grid_with_content(text);

    assert_eq!(row_text(&grid.viewport[0]), text, "draft must retain the exact reporter residual text");
    assert_eq!(grid.cursor.x, 9, "draft internal cursor policy for the reporter residual");
    assert_eq!(fieldwork_scalar_width(text), 11, "scalar terminal policy for the same serialized text");
}

#[test]
fn fieldwork_telugu_full_reporter_line_has_twenty_four_internal_columns_and_thirty_two_scalar_columns() {
    let text = "వసంత ఋతువు గురించి ఒక కంద పద్యం వ్రాయి";
    let grid = create_grid_with_content(text);

    assert_eq!(row_text(&grid.viewport[0]), text, "draft must retain the exact full reporter line");
    assert_eq!(grid.cursor.x, 24, "draft internal cursor policy for the full reporter line");
    assert_eq!(fieldwork_scalar_width(text), 32, "scalar terminal policy for the same serialized text");
}
