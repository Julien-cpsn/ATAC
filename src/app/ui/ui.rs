use ratatui::backend::Backend;
use ratatui::{Frame, Terminal};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::{Modifier, Style};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::widgets::block::Title;
use crate::app::app::{App};
use crate::app::app_states::AppState::*;
use crate::utils::centered_rect::centered_rect;

impl App<'_> {
    fn ui(&mut self, frame: &mut Frame) {
        // MAIN LAYOUT

        let main_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ],
        )
            .split(frame.size());


        // HEADER

        let header = Block::new()
            .title("* ATAC *")
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC)
            .title_alignment(Alignment::Center)
            .borders(Borders::TOP);

        frame.render_widget(header, main_layout[0]);

        // INNER LAYOUT

        let inner_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage(18),
                Constraint::Percentage(82)
            ],
        )
            .split(main_layout[1]);

        // COLLECTION

        self.render_collection(frame, inner_layout[0]);

        // REQUEST

        match self.collection.selected {
            None => self.render_homepage(frame, inner_layout[1]),
            Some(selected_request_index) => {
                let selected_request = self.collection.items[selected_request_index].clone();

                self.render_request(frame, inner_layout[1], selected_request);
            },
        }

        // NEW REQUEST DIALOG

        match self.state {
            CreatingNewRequest => {
                let popup_block = Block::default()
                    .title("Enter the new request name")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::DarkGray));


                let area = centered_rect(40, 20, 3, 50, frame.size());
                let new_request_area = popup_block.inner(area);

                let new_request_paragraph = Paragraph::new(self.new_request_input.text.as_str());

                frame.render_widget(popup_block, area);
                frame.render_widget(new_request_paragraph, new_request_area);

                frame.set_cursor(
                    new_request_area.x + self.new_request_input.cursor_position as u16,
                    new_request_area.y
                )
            }
            _ => {}
        }

        // FOOTER

        let state_line = self.get_state_line();
        let available_keys = self.get_available_keys();

        let footer = Block::new()
            .title(Title::from(state_line).alignment(Alignment::Left))
            .title(Title::from(available_keys.dark_gray()).alignment(Alignment::Right));

        frame.render_widget(footer, main_layout[2]);
    }

    pub fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.draw(|frame | self.ui(frame))?;
        Ok(())
    }
}