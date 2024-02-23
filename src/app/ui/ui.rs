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
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ],
        )
            .split(main_layout[1]);

        // COLLECTION

        self.render_collection(frame, inner_layout[0]);

        // REQUEST

        match self.collections_tree.selected {
            None => self.render_homepage(frame, inner_layout[1]),
            Some(selection) => {
                let selected_request = self.collections[selection.0].requests[selection.1].clone();

                self.render_request(frame, inner_layout[1], selected_request);
            }
        }

        // NEW REQUEST DIALOG

        match self.state {
            CreatingNewRequest => {
                let popup_block = Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::DarkGray));


                let area = centered_rect(40, 20, 6, 50, frame.size());

                let new_request_layout = Layout::new(
                    Vertical,
                    vec![
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ]
                )
                    .split(area);


                let selected_collection_name = self.collections[self.new_request_popup.selected_collection].name.clone();
                let selected_collection_paragraph = Paragraph::new(selected_collection_name)
                    .block(
                        Block::new()
                            .title("Collection ↑ ↓")
                            .borders(Borders::ALL)
                    );

                let new_request_name_paragraph = Paragraph::new(self.new_request_popup.text_input.text.as_str())
                    .block(
                        Block::new()
                            .title("Request name")
                            .borders(Borders::ALL)
                    );

                frame.render_widget(popup_block, area);
                frame.render_widget(selected_collection_paragraph, new_request_layout[0]);
                frame.render_widget(new_request_name_paragraph, new_request_layout[1]);

                frame.set_cursor(
                    new_request_layout[1].x + self.new_request_popup.text_input.cursor_position as u16 + 1,
                    new_request_layout[1].y + 1
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