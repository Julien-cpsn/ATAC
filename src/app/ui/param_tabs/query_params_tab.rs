use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Color::Yellow;
use ratatui::prelude::{Modifier, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use crate::app::app::App;
use crate::app::app_states::AppState::EditingRequestParam;
use crate::request::request::Request;

impl App<'_> {
    pub(super) fn render_query_params_tab(&mut self, frame: &mut Frame, area: Rect, request: &Request, param_selection: (usize, usize)) {
        let params_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(2),
                Constraint::Fill(1)
            ]
        )
            .split(area);

        let header_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]
        )
            .split(params_layout[0]);

        let header_param = Paragraph::new("Param")
            .centered()
            .block(Block::new().borders(Borders::BOTTOM | Borders::RIGHT))
            .dark_gray();
        let header_value = Paragraph::new("Value")
            .centered()
            .block(Block::new().borders(Borders::BOTTOM))
            .dark_gray();

        frame.render_widget(header_param, header_layout[0]);
        frame.render_widget(header_value, header_layout[1]);

        let horizontal_margin = 2;

        let table_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]
        )
            .horizontal_margin(horizontal_margin)
            .split(params_layout[1]);

        let mut params: Vec<ListItem> = vec![];
        let mut values: Vec<ListItem> = vec![];

        for param in request.params.iter() {
            let key = self.add_color_to_env_keys(&param.data.0);
            let value = self.add_color_to_env_keys(&param.data.1);

            let mut key = ListItem::from(key);
            let mut value = ListItem::from(value);

            if !param.enabled {
                key = key.dark_gray().dim();
                value = value.dark_gray().dim();
            }

            params.push(key);
            values.push(value);
        }

        let mut left_list_style = Style::default();
        let mut right_list_style = Style::default();

        match param_selection.1 {
            0 => left_list_style = left_list_style.fg(Yellow).add_modifier(Modifier::BOLD),
            1 => right_list_style = right_list_style.fg(Yellow).add_modifier(Modifier::BOLD),
            _ => {}
        }

        let left_list = List::new(params)
            .highlight_style(left_list_style);

        let right_list = List::new(values)
            .highlight_style(right_list_style);

        frame.render_stateful_widget(left_list, table_layout[0], &mut self.query_params_table.left_state.clone());
        frame.render_stateful_widget(right_list, table_layout[1], &mut self.query_params_table.right_state.clone());

        // Param input & cursor

        if self.state == EditingRequestParam {
            let cell_width = params_layout[1].width / 2;

            let width_adjustment = match param_selection.1 {
                0 => 0,
                1 => {
                    let even_odd_adjustment = match params_layout[1].width % 2 {
                        1 => 1,
                        0 => 2,
                        _ => 0
                    };
                    cell_width - even_odd_adjustment
                },
                _ => 0
            };

            let height_adjustment = (param_selection.0 - self.query_params_table.left_state.offset()) as u16 % params_layout[1].height;

            let selection_position_x = params_layout[1].x + width_adjustment + horizontal_margin;
            let selection_position_y = params_layout[1].y + height_adjustment;

            let text_rect = Rect::new(selection_position_x, selection_position_y, cell_width, 1);

            let adjusted_input_length = text_rect.width as usize - 2;
            let (padded_text, input_cursor_position) = self.query_params_table.selection_text_input.get_padded_text_and_cursor(adjusted_input_length);

            let text_input = Paragraph::new(format!("{:fill$}", padded_text, fill = (cell_width - horizontal_margin) as usize));

            frame.render_widget(text_input, text_rect);

            frame.set_cursor(
                selection_position_x + input_cursor_position as u16,
                selection_position_y
            );
        }
    }
}