use ratatui::backend::Backend;
use ratatui::{Frame, Terminal};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::{Modifier};
use ratatui::style::{Stylize};
use ratatui::widgets::{Block, Borders};
use ratatui::widgets::block::Title;
use crate::app::app::{App};
use crate::app::app_states::AppState::*;

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

        // LEFT LAYOUT

        match self.environments.is_empty() {
            // No environments
            true => {
                let env_and_collections_layout = Layout::new(
                    Vertical,
                    [
                        Constraint::Fill(1)
                    ]
                )
                    .split(inner_layout[0]);

                // COLLECTION

                self.render_collections(frame, env_and_collections_layout[0]);
            }
            // At least one environment
            false => {
                let env_and_collections_layout = Layout::new(
                    Vertical,
                    [
                        Constraint::Length(3),
                        Constraint::Fill(1)
                    ]
                )
                    .split(inner_layout[0]);

                // ENVIRONMENTS

                self.render_environments(frame, env_and_collections_layout[0]);

                // COLLECTION

                self.render_collections(frame, env_and_collections_layout[1]);
            }
        }

        // REQUEST

        match self.collections_tree.selected {
            None => self.render_homepage(frame, inner_layout[1]),
            Some(selection) => {
                let selected_request = self.get_request_as_local_from_indexes(&selection).read().unwrap().clone();

                self.render_request(frame, inner_layout[1], selected_request);
            }
        }

        // POPUPS

        match self.state {
            CreatingNewCollection => self.render_creating_new_collection_popup(frame),
            CreatingNewRequest => self.render_creating_new_request_popup(frame),
            DeletingCollection => self.render_deleting_collection_popup(frame),
            DeletingRequest => self.render_deleting_request_popup(frame),
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