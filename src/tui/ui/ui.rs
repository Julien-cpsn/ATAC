use ratatui::backend::Backend;
use ratatui::{Frame, Terminal};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::{Modifier};
use ratatui::style::{Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders};
use ratatui::widgets::block::Title;

use crate::app::app::{App};
use crate::app::files::theme::THEME;
use crate::tui::app_states::AppState::*;
use crate::tui::app_states::{AVAILABLE_EVENTS, event_available_keys_to_spans};


impl App<'_> {
    fn ui(&mut self, frame: &mut Frame) {

        if let Some(bg_color) = THEME.read().ui.app_background {
            let test = Block::new().bg(bg_color);

            frame.render_widget(test, frame.area());
        }


        // MAIN LAYOUT

        let main_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ],
        )
            .split(frame.area());


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
                let selected_request = self.get_request_as_local_from_indexes(&selection).read().clone();

                self.render_request(frame, inner_layout[1], selected_request);
            }
        }

        // FOOTER

        let state_line = self.get_state_line();
        let events = &*AVAILABLE_EVENTS.read();
        let available_keys = Line::from(event_available_keys_to_spans(
            events,
            THEME.read().ui.secondary_foreground_color,
            THEME.read().ui.secondary_background_color,
            true
        ).concat());

        let footer_left = Block::new()
            .title(Title::from(state_line)).title_alignment(Alignment::Left);
        
        let footer_right = Block::new()
            .title(Title::from(available_keys)).title_alignment(Alignment::Right);

        frame.render_widget(footer_left, main_layout[2]);
        frame.render_widget(footer_right, main_layout[2]);

        // POPUPS

        match self.state {
            DisplayingCookies | EditingCookies => self.render_cookies_popup(frame),
            ChoosingElementToCreate => self.render_creating_element_popup(frame),
            CreatingNewCollection => self.render_creating_new_collection_popup(frame),
            CreatingNewRequest => self.render_creating_new_request_popup(frame),
            DeletingCollection => self.render_deleting_collection_popup(frame),
            DeletingRequest => self.render_deleting_request_popup(frame),
            EditingRequestSettings => self.render_request_settings_popup(frame),
            RenamingCollection => self.render_renaming_collection_popup(frame),
            RenamingRequest => self.render_renaming_request_popup(frame),
            ChoosingRequestExportFormat => self.render_export_format_popup(frame),
            DisplayingRequestExport => self.display_request_export.render(frame),
            _ => {}
        }

        if self.should_display_help {
            self.render_help_popup(frame);   
        }
    }

    pub fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.draw(|frame | self.ui(frame))?;
        Ok(())
    }
}