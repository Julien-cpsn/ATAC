use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(columns: u16, lines: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - (lines * 100 / r.height)) / 2),
            Constraint::Length(lines),
            Constraint::Percentage((100 - (lines * 100 / r.height)) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    return Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - (columns * 100 / r.width)) / 2),
            Constraint::Length(columns),
            Constraint::Percentage((100 - (columns * 100 / r.width)) / 2),
        ])
        .split(popup_layout[1])[1]; // Return the middle chunk
}
