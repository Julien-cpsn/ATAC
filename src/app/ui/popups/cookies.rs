use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::{Line, Modifier, Style};
use ratatui::prelude::Color::Yellow;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};

use crate::app::app::App;
use crate::utils::centered_rect::centered_rect;
use crate::utils::cookie_table::{CookieColumns, COOKIES_COLUMNS_NUMBER};

impl App<'_> {
    pub fn render_cookies_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Cookies")
            .borders(Borders::ALL)
            .white()
            .on_dark_gray();

        let area = centered_rect(120, 25, frame.size());

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
            CookieColumns::constraints()
        )
            .split(cookies_layout[0]);

        let header_names = vec![
            CookieColumns::URL.to_string(),
            CookieColumns::Name.to_string(),
            CookieColumns::Value.to_string(),
            CookieColumns::Path.to_string(),
            CookieColumns::Expires.to_string(),
            CookieColumns::HttpOnly.to_string(),
            CookieColumns::Secure.to_string(),
            CookieColumns::SameSite.to_string(),
        ];

        for (index, header_name) in header_names.iter().enumerate() {
            let paragraph = Paragraph::new(header_name.as_str())
                .centered()
                .block(Block::new().borders(Borders::BOTTOM | Borders::RIGHT))
                .gray();

            frame.render_widget(paragraph, inner_cookies_layout[index]);
        }

        match self.cookies_popup.cookies_table.selection {
            None => {
                let cookies_lines = vec![
                    Line::default(),
                    Line::from("No cookies"),
                    Line::from("(Add one by sending a request)".gray())
                ];

                let cookies_paragraph = Paragraph::new(cookies_lines).centered();

                frame.render_widget(cookies_paragraph, cookies_layout[1]);
            },
            Some(selection) => {
                self.render_cookie_list(selection, frame, cookies_layout[1]);
                //self.render_cookie_cursor(selection, horizontal_margin + 1, frame, cookies_layout[1]);
            }
        }
    }

    fn render_cookie_list(&mut self, selection: (usize, usize), frame: &mut Frame, area: Rect) {
        let table_layout = Layout::new(
            Horizontal,
            CookieColumns::constraints()
        )
            .horizontal_margin(2)
            .split(area);

        let mut cookies: [Vec<ListItem>; COOKIES_COLUMNS_NUMBER] = [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![]
        ];

        for cookie in &self.cookies_popup.cookies_table.rows {
            for (index, value) in cookie.iter().enumerate() {
                let value = ListItem::from(value.clone());

                cookies[index].push(value);
            }
        }

        let mut list_styles = [
            Style::default(),
            Style::default(),
            Style::default(),
            Style::default(),
            Style::default(),
            Style::default(),
            Style::default(),
            Style::default()
        ];

        list_styles[selection.1] = list_styles[selection.1].fg(Yellow).add_modifier(Modifier::BOLD);

        for (index, cookie) in cookies.iter().enumerate() {
            let list = List::new(cookie.clone()).highlight_style(list_styles[index]);

            frame.render_stateful_widget(
                list,
                table_layout[index],
                &mut self.cookies_popup.cookies_table.lists_states[index].clone()
            );
        }
    }

    /*
    fn render_cookie_cursor(&mut self, selection: (usize, usize), horizontal_margin: u16, frame: &mut Frame, area: Rect) {
        if self.state == EditingCookies {
            let cell_with = area.width / COOKIES_COLUMNS_NUMBER as u16;

            let even_odd_adjustment = match area.width % 2 {
                1 => 1,
                0 => 2,
                _ => 0
            };

            let width_adjustment = cell_with - even_odd_adjustment;

            let height_adjustment = (selection.0 - self.cookies_popup.cookies_table.lists_states[0].offset()) as u16 % area.height;

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
    }*/
}