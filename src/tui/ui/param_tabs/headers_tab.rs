use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::prelude::{Modifier, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::app_states::AppState::{EditingRequestHeader};
use crate::models::request::Request;

impl App<'_> {
    pub(super) fn render_headers_tab(&mut self, frame: &mut Frame, area: Rect, request: &Request, header_selection: (usize, usize)) {
        let headers_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(2),
                Constraint::Fill(1)
            ]
        )
            .split(area);

        let inner_header_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]
        )
            .split(headers_layout[0]);

        let header_title = Paragraph::new("Header")
            .centered()
            .block(
                Block::new()
                    .borders(Borders::BOTTOM | Borders::RIGHT)
                    .fg(THEME.read().ui.secondary_foreground_color)
            )
            .fg(THEME.read().ui.secondary_foreground_color);

        let header_value = Paragraph::new("Value")
            .centered()
            .block(
                Block::new()
                    .borders(Borders::BOTTOM)
                    .fg(THEME.read().ui.secondary_foreground_color)
            )
            .fg(THEME.read().ui.secondary_foreground_color);

        frame.render_widget(header_title, inner_header_layout[0]);
        frame.render_widget(header_value, inner_header_layout[1]);

        let horizontal_margin = 2;

        let table_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]
        )
            .horizontal_margin(horizontal_margin)
            .split(headers_layout[1]);

        let mut headers: Vec<ListItem> = vec![];
        let mut values: Vec<ListItem> = vec![];

        for header in request.headers.iter() {
            let key = self.tui_add_color_to_env_keys(&header.data.0);
            let value = self.tui_add_color_to_env_keys(&header.data.1);

            let mut key = ListItem::from(key);
            let mut value = ListItem::from(value);

            if !header.enabled {
                key = key.fg(THEME.read().ui.secondary_foreground_color).dim();
                value = value.fg(THEME.read().ui.secondary_foreground_color).dim();
            }

            headers.push(key);
            values.push(value);
        }

        let mut left_list_style = Style::default();
        let mut right_list_style = Style::default();

        match header_selection.1 {
            0 => left_list_style = left_list_style
                .add_modifier(Modifier::BOLD)
                .fg(THEME.read().others.selection_highlight_color),
            1 => right_list_style = right_list_style
                .add_modifier(Modifier::BOLD)
                .fg(THEME.read().others.selection_highlight_color),
            _ => {}
        }

        let left_list = List::new(headers)
            .highlight_style(left_list_style)
            .fg(THEME.read().ui.font_color);

        let right_list = List::new(values)
            .highlight_style(right_list_style)
            .fg(THEME.read().ui.font_color);

        frame.render_stateful_widget(left_list, table_layout[0], &mut self.headers_table.left_state.clone());
        frame.render_stateful_widget(right_list, table_layout[1], &mut self.headers_table.right_state.clone());

        // Header input & cursor

        if self.state == EditingRequestHeader {
            let cell_width = headers_layout[1].width / 2;

            let width_adjustment = match header_selection.1 {
                0 => 0,
                1 => {
                    let even_odd_adjustment = match headers_layout[1].width % 2 {
                        1 => 1,
                        0 => 2,
                        _ => 0
                    };
                    cell_width - even_odd_adjustment
                },
                _ => 0
            };

            let height_adjustment = (header_selection.0 - self.headers_table.left_state.offset()) as u16 % headers_layout[1].height;

            let selection_position_x = headers_layout[1].x + width_adjustment + horizontal_margin;
            let selection_position_y = headers_layout[1].y + height_adjustment;
            
            let text_rect = Rect::new(selection_position_x, selection_position_y, cell_width, 1);
            
            let adjusted_input_length = text_rect.width as usize - 2;
            let (padded_text, input_cursor_position) = self.headers_table.selection_text_input.get_padded_text_and_cursor(adjusted_input_length);
            
            let text_input = Paragraph::new(format!("{:fill$}", padded_text, fill = (cell_width - horizontal_margin) as usize));

            frame.render_widget(text_input, text_rect);

            frame.set_cursor_position(Position::new(
                selection_position_x + input_cursor_position as u16,
                selection_position_y
            ));
        }
    }
}