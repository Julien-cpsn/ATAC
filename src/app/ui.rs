use ratatui::backend::Backend;
use ratatui::{Frame, Terminal};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::{Modifier, Style};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Borders, List, ListItem, Padding, Paragraph};
use ratatui::widgets::block::Title;
use tui_big_text::{BigTextBuilder, PixelSize};
use crate::app::app::{App};
use crate::app::app_states::{AppState, get_available_keys};
use crate::app::request_ui::views::RequestView;
use crate::request::method::get_method_bg;
use crate::request::request::Request;
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
            AppState::CreatingNewRequest => {
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

        let state_text = match self.state {
            AppState::Normal | AppState::CreatingNewRequest => self.state.to_string(),
            AppState::SelectedRequest | AppState::EditingRequestUrl | AppState::EditingRequestBody => {
                let selected_request_index = self.collection.selected.unwrap();
                let selected_request = &self.collection.items[selected_request_index];

                if self.state == AppState::SelectedRequest {
                    self.state.to_string()
                }
                else {
                    format!("Request > {} > {}", selected_request.name, self.state.to_string())
                }
            }
        };

        let available_keys = get_available_keys(self.state);

        let footer = Block::new()
            .title(Title::from(state_text).alignment(Alignment::Left))
            .title(Title::from(available_keys.dark_gray()).alignment(Alignment::Right));

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

    fn render_request(&mut self, frame: &mut Frame, rect: Rect, request: Request) {
        let request_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Fill(1)
            ],
        )
            .split(rect);

        // REQUEST NAME

        let request_name = request.name;

        let request_name_paragraph = Paragraph::new(request_name)
            .centered();

        frame.render_widget(request_name_paragraph, request_layout[0]);

        // REQUEST HEADER LAYOUT

        let request_header_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage(7),
                Constraint::Percentage(93)
            ],
        )
            .split(request_layout[1]);

        // REQUEST METHOD

        let method = &request.method;

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
            AppState::EditingRequestUrl => {
                frame.set_cursor(
                    request_header_layout[1].x + self.url_text_input.cursor_position as u16 + 2,
                    request_header_layout[1].y + 1
                )
            }
            _ => {}
        }

        // REQUEST MAIN LAYOUT

        let request_main_layout_constraints = match self.request_view {
            RequestView::Normal => [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ],
            RequestView::OnlyResult => [
                Constraint::Percentage(0),
                Constraint::Percentage(100)
            ],
            RequestView::OnlyParams => [
                Constraint::Percentage(100),
                Constraint::Percentage(0)
            ]
        };

        let request_main_layout = Layout::new(
            Horizontal,
            request_main_layout_constraints,
        )
            .split(request_layout[2]);


        // REQUEST PARAMS

        let params_block = Block::new().borders(Borders::RIGHT);
        let request_params_area = params_block.inner(request_main_layout[0]);

        frame.render_widget(params_block, request_main_layout[0]);
        self.render_request_params(frame, request_params_area, &request);

        // REQUEST RESULT LAYOUT

        let result_block = Block::new();
        let result_block_area = result_block.inner(request_main_layout[1]);

        frame.render_widget(result_block, request_main_layout[1]);
        self.render_request_result(frame, result_block_area, &request);
    }

    fn render_homepage(&mut self, frame: &mut Frame, rect: Rect) {
        let block = Block::new();

        let inner_block_area = block.inner(rect);

        let inner_layout = Layout::new(
            Vertical,
            [
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Length(4),
                Constraint::Length(1),
                Constraint::Percentage(50)
            ]
        )
            .split(inner_block_area);

        let title_length = 16;

        let title_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage((100-title_length)/2+2),
                Constraint::Length(title_length),
                Constraint::Percentage((100-title_length)/2),
            ]
        )
            .split(inner_layout[2]);

        let title = BigTextBuilder::default()
            .pixel_size(PixelSize::Quadrant)
            .lines([
                "ATAC".into(),
            ])
            .build()
            .unwrap();


        let welcome_to = Paragraph::new("Welcome to").centered();
        let description = Paragraph::new("{A}rguably a {T}UI {A}PI {C}lient").centered();

        frame.render_widget(block, rect);
        frame.render_widget(welcome_to, inner_layout[1]);
        frame.render_widget(title, title_layout[1]);
        frame.render_widget(description, inner_layout[3]);
    }

    pub fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.draw(|frame | self.ui(frame))?;
        Ok(())
    }
}