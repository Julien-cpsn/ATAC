use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Color::Yellow;
use ratatui::prelude::{Modifier, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::app::app::App;
use crate::tui::app_states::AppState::{EditingRequestBodyTable};
use crate::models::request::{KeyValue};

impl App<'_> {
    pub(super) fn render_form_body_tab(&mut self, frame: &mut Frame, area: Rect, form: &Vec<KeyValue>, form_selection: (usize, usize)) {
        let form_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(2),
                Constraint::Fill(1)
            ]
        )
            .split(area);

        let inner_form_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]
        )
            .split(form_layout[0]);

        let form_title = Paragraph::new("Key")
            .centered()
            .block(Block::new().borders(Borders::BOTTOM | Borders::RIGHT))
            .dark_gray();

        let form_value = Paragraph::new("Value")
            .centered()
            .block(Block::new().borders(Borders::BOTTOM))
            .dark_gray();

        frame.render_widget(form_title, inner_form_layout[0]);
        frame.render_widget(form_value, inner_form_layout[1]);

        let horizontal_margin = 2;

        let table_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]
        )
            .horizontal_margin(horizontal_margin)
            .split(form_layout[1]);

        let mut keys: Vec<ListItem> = vec![];
        let mut values: Vec<ListItem> = vec![];

        for form_data in form.iter() {
            let key = self.add_color_to_env_keys(&form_data.data.0);
            let value = self.add_color_to_env_keys(&form_data.data.1);

            let mut key = ListItem::from(key);
            let mut value = ListItem::from(value);

            if !form_data.enabled {
                key = key.dark_gray().dim();
                value = value.dark_gray().dim();
            }

            keys.push(key);
            values.push(value);
        }

        let mut left_list_style = Style::default();
        let mut right_list_style = Style::default();

        match form_selection.1 {
            0 => left_list_style = left_list_style.fg(Yellow).add_modifier(Modifier::BOLD),
            1 => right_list_style = right_list_style.fg(Yellow).add_modifier(Modifier::BOLD),
            _ => {}
        }

        let left_list = List::new(keys).highlight_style(left_list_style);

        let right_list = List::new(values).highlight_style(right_list_style);

        frame.render_stateful_widget(left_list, table_layout[0], &mut self.body_form_table.left_state.clone());
        frame.render_stateful_widget(right_list, table_layout[1], &mut self.body_form_table.right_state.clone());

        // Form input & cursor

        if self.state == EditingRequestBodyTable {
            let cell_with = form_layout[1].width / 2;

            let width_adjustment = match form_selection.1 {
                0 => 0,
                1 => {
                    let even_odd_adjustment = match form_layout[1].width % 2 {
                        1 => 1,
                        0 => 2,
                        _ => 0
                    };
                    cell_with - even_odd_adjustment
                },
                _ => 0
            };

            let height_adjustment = (form_selection.0 - self.body_form_table.left_state.offset()) as u16 % form_layout[1].height;

            let selection_position_x = form_layout[1].x + width_adjustment + horizontal_margin;
            let selection_position_y = form_layout[1].y + height_adjustment;

            let form_data_text = self.body_form_table.selection_text_input.text.clone();

            let text_input = Paragraph::new(format!("{:fill$}", form_data_text, fill = (cell_with - horizontal_margin) as usize));
            let text_rect = Rect::new(selection_position_x, selection_position_y, cell_with, 1);

            frame.render_widget(text_input, text_rect);

            frame.set_cursor(
                selection_position_x + self.body_form_table.selection_text_input.cursor_position as u16,
                selection_position_y
            );
        }
    }
}