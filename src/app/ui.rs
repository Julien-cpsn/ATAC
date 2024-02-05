use ratatui::backend::Backend;
use ratatui::{Frame, Terminal};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Margin, Rect};
use ratatui::prelude::{Modifier, Style};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Scrollbar, ScrollbarOrientation, Tabs};
use strum::IntoEnumIterator;
use crate::app::app::{App, AppState};
use crate::app::tabs::tabs::RequestTabs;
use crate::request::method::get_method_bg;
use crate::utils::centered_rect::centered_rect;

impl App<'_> {
    fn ui(&mut self, frame: &mut Frame) {
        // MAIN LAYOUT

        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ],
        )
            .split(frame.size());


        // HEADER

        let header = Block::new()
            .title("* TUI-Quest *")
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC)
            .title_alignment(Alignment::Center)
            .borders(Borders::TOP);

        frame.render_widget(header, main_layout[0]);

        // INNER LAYOUT

        let inner_layout = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ],
        )
            .split(main_layout[1]);

        // COLLECTION

        self.render_collection(frame, inner_layout[0]);

        // REQUEST

        match self.collection.selected {
            None => self.render_homepage(frame, inner_layout[1]),
            Some(_) => self.render_request(frame, inner_layout[1]),
        }

        // NEW REQUEST DIALOG

        match self.state {
            AppState::CreatingNewRequest => {
                let popup_block = Block::default()
                    .title("Enter the new request name")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::DarkGray));


                let area = centered_rect(40, 20, frame.size());
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

        // MAIN LAYOUT

        // FOOTER

        let footer_text = match self.state {
            AppState::Normal => "Visual",
            AppState::EditingUrl | AppState::CreatingNewRequest | AppState::EditingBody => "Insert"
        };

        let footer = Block::new()
            .title(footer_text);

        frame.render_widget(footer, main_layout[2]);
    }

    fn render_collection(&mut self, frame: &mut Frame, rect: Rect) {
        let items: Vec<ListItem> = self.collection.items
            .iter()
            .map(|request| {
                request.to_list_item()
            })
            .collect();

        let list = List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .block(
                Block::default()
                    .title("Collection")
                    .borders(Borders::ALL)
            );

        frame.render_stateful_widget(
            list,
            rect,
            &mut self.collection.state
        );
    }

    fn render_request(&mut self, frame: &mut Frame, rect: Rect) {
        let request_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Fill(1)
            ],
        )
            .split(rect);

        let selected_request_index = self.collection.selected.unwrap();
        let selected_request = &self.collection.items[selected_request_index];

        // REQUEST NAME

        let request_name = selected_request.name;

        let request_name_paragraph = Paragraph::new(request_name)
            .centered();

        frame.render_widget(request_name_paragraph, request_layout[0]);

        // REQUEST HEADER LAYOUT

        let request_header_layout = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(7),
                Constraint::Percentage(93)
            ],
        )
            .split(request_layout[1]);

        // REQUEST METHOD

        let method = selected_request.method.clone();

        let method_block = Block::new()
            .title("Method").title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1));

        let method_area = method_block.inner(request_header_layout[0]);

        let method_paragraph = Paragraph::new(method.to_string())
            .bg(get_method_bg(&method))
            .centered();

        frame.render_widget(method_block, request_header_layout[0]);
        frame.render_widget(method_paragraph, method_area);

        // REQUEST URL

        let url_block = Block::new()
            .title("URL")
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1));

        let url_paragraph = Paragraph::new(self.url_text_input.text.as_str())
            .block(url_block);

        frame.render_widget(url_paragraph, request_header_layout[1]);

        match self.state {
            AppState::EditingUrl => {
                frame.set_cursor(
                    request_header_layout[1].x + self.url_text_input.cursor_position as u16 + 2,
                    request_header_layout[1].y + 1
                )
            }
            _ => {}
        }

        // REQUEST MAIN LAYOUT

        let request_main_layout = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ],
        )
            .split(request_layout[2]);


        // REQUEST PARAMS

        let params_block = Block::new()
            .borders(Borders::RIGHT)
            .padding(Padding::horizontal(1));

        let request_params_area = params_block.inner(request_main_layout[0]);

        frame.render_widget(params_block, request_main_layout[0]);

        let request_params_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Fill(1)
            ]
        )
            .split(request_params_area);

        // REQUEST PARAM TABS

        let tabs = RequestTabs::iter().map(|tab| tab.to_string());
        let selected_tab_index = self.request_tab as usize;

        let params_tab = Tabs::new(tabs)
            .highlight_style(Style::default().yellow())
            .select(selected_tab_index);

        frame.render_widget(params_tab, request_params_layout[0]);

        // REQUEST PARAM TABS CONTENT

        match self.request_tab {
            RequestTabs::Params => {}
            RequestTabs::Auth => {}
            RequestTabs::Headers => {}
            RequestTabs::Body => {
                self.body_text_area.set_line_number_style(Style::new().fg(Color::DarkGray));

                frame.render_widget(self.body_text_area.widget(), request_params_layout[1]);
            }
        }

        // REQUEST RESULT

        let result_block = Block::new()
            .title("Result").title_style(Style::new().underlined())
            .padding(Padding::horizontal(1));

        let result_block_area = result_block.inner(request_main_layout[1]);

        let result_text = match &selected_request.result {
            None => "",
            Some(result) => result
        };


        let result_paragraph = Paragraph::new(result_text)
            .scroll((self.result_scrollbar.scroll, 0));


        let result_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);


        frame.render_widget(result_block, request_main_layout[1]);
        frame.render_widget(result_paragraph, result_block_area);
        frame.render_stateful_widget(
            result_scrollbar,
            result_block_area.inner(&Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.result_scrollbar.state
        )
    }

    fn render_homepage(&mut self, frame: &mut Frame, rect: Rect) {
        frame.render_widget(
            Paragraph::new("Welcome to TUI-Quest").centered(),
            rect
        );
    }

    pub fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.draw(|frame | self.ui(frame))?;
        Ok(())
    }
}