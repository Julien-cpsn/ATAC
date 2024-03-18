use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::{Line, Modifier, Style};
use ratatui::prelude::Color::Yellow;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};

use crate::app::app::App;
use crate::app::app_states::AppState::EditingCookies;
use crate::utils::centered_rect::centered_rect;

impl App<'_> {
    pub fn render_cookies_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Cookies")
            .borders(Borders::ALL)
            .white()
            .on_dark_gray();

        let area = centered_rect(100, 25, frame.size());

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);

        let horizontal_margin = 1;

        let cookies_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(2),
                Constraint::Fill(1)
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(horizontal_margin)
            .split(area);

        let inner_cookies_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ]
        )
            .split(cookies_layout[0]);

        let cookie_url = Paragraph::new("URL")
            .centered()
            .block(Block::new().borders(Borders::BOTTOM | Borders::RIGHT))
            .gray();
        let cookies_value = Paragraph::new("Value")
            .centered()
            .block(Block::new().borders(Borders::BOTTOM))
            .gray();

        frame.render_widget(cookie_url, inner_cookies_layout[0]);
        frame.render_widget(cookies_value, inner_cookies_layout[1]);

        match self.cookies_popup.cookies_table.selection {
            None => {
                let cookies_lines = vec![
                    Line::default(),
                    Line::from("No cookies"),
                    Line::from("(Add one with n or via sending a request)".gray())
                ];

                let cookies_paragraph = Paragraph::new(cookies_lines).centered();

                frame.render_widget(cookies_paragraph, cookies_layout[1]);
            },
            Some(selection) => {
                self.render_cookie_list(selection, frame, cookies_layout[1]);
                self.render_cookie_cursor(selection, horizontal_margin + 1, frame, cookies_layout[1]);
            }
        }
    }

    fn render_cookie_list(&mut self, selection: (usize, usize), frame: &mut Frame, area: Rect) {
        let table_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ]
        )
            .horizontal_margin(2)
            .split(area);

        let mut cookies: Vec<ListItem> = vec![];
        let mut values: Vec<ListItem> = vec![];

        for cookie in &self.cookies_popup.cookies_table.rows {
            let mut key = ListItem::from(cookie.data.0.clone());
            let mut value = ListItem::from(cookie.data.1.clone());

            if !cookie.enabled {
                key = key.gray().dim();
                value = value.gray().dim();
            }

            cookies.push(key);
            values.push(value);
        }

        let mut left_list_style = Style::default();
        let mut right_list_style = Style::default();

        match selection.1 {
            0 => left_list_style = left_list_style.fg(Yellow).add_modifier(Modifier::BOLD),
            1 => right_list_style = right_list_style.fg(Yellow).add_modifier(Modifier::BOLD),
            _ => {}
        }

        let left_list = List::new(cookies)
            .highlight_style(left_list_style);

        let right_list = List::new(values)
            .highlight_style(right_list_style);

        frame.render_stateful_widget(left_list, table_layout[0], &mut self.cookies_popup.cookies_table.left_state.clone());
        frame.render_stateful_widget(right_list, table_layout[1], &mut self.cookies_popup.cookies_table.right_state.clone());
    }

    fn render_cookie_cursor(&mut self, selection: (usize, usize), horizontal_margin: u16, frame: &mut Frame, area: Rect) {
        if self.state == EditingCookies {
            let cell_with = area.width / 2;

            let width_adjustment = match selection.1 {
                0 => 0,
                1 => {
                    let even_odd_adjustment = match area.width % 2 {
                        1 => 1,
                        0 => 2,
                        _ => 0
                    };
                    cell_with - even_odd_adjustment
                },
                _ => 0
            };

            let height_adjustment = (selection.0 - self.cookies_popup.cookies_table.left_state.offset()) as u16 % area.height;

            let selection_position_x = area.x + width_adjustment + horizontal_margin;
            let selection_position_y = area.y + height_adjustment;

            let param_text = self.cookies_popup.cookies_table.selection_text_input.text.clone();

            let text_input = Paragraph::new(format!("{:fill$}", param_text, fill = (cell_with - horizontal_margin) as usize));
            let text_rect = Rect::new(selection_position_x, selection_position_y, cell_with, 1);

            frame.render_widget(text_input, text_rect);

            frame.set_cursor(
                selection_position_x + self.cookies_popup.cookies_table.selection_text_input.cursor_position as u16,
                selection_position_y
            );
        }
    }
}