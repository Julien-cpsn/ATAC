use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(columns: u16, lines: u16, r: Rect) -> Rect {
    let x_space = (100u16.saturating_sub(lines * 100 / r.height)) / 2;
    let y_space = (100u16.saturating_sub(columns * 100 / r.width)) / 2;

    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(x_space),
            Constraint::Length(lines),
            Constraint::Percentage(x_space),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    return Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(y_space),
            Constraint::Length(columns),
            Constraint::Percentage(y_space),
        ])
        .split(popup_layout[1])[1]; // Return the middle chunk
}
