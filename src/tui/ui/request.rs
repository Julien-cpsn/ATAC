use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::Color::White;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

use crate::app::app::App;
use crate::models::request::Request;
use crate::tui::ui::views::RequestView;
use crate::tui::app_states::AppState::EditingRequestUrl;

impl App<'_> {
    pub(super) fn render_request(&mut self, frame: &mut Frame, rect: Rect, request: Request) {
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

        let request_name = request.name.clone();

        let request_name_paragraph = Paragraph::new(request_name)
            .centered();

        frame.render_widget(request_name_paragraph, request_layout[0]);

        // REQUEST HEADER LAYOUT

        let request_header_layout = Layout::new(
            Horizontal,
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90)
            ],
        )
            .split(request_layout[1]);

        // REQUEST METHOD

        let method = request.method.clone();

        let method_block = Block::new()
            .title("Method").title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1));

        let method_area = method_block.inner(request_header_layout[0]);

        let method_paragraph = Paragraph::new(method.to_string())
            .bg(method.get_color())
            .fg(White)
            .centered();

        frame.render_widget(method_block, request_header_layout[0]);
        frame.render_widget(method_paragraph, method_area);

        // REQUEST URL

        let url_block = Block::new()
            .title("URL")
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1));
        
        let adjusted_input_length = request_header_layout[1].width as usize - 4;
        let (padded_text, input_cursor_position) = self.url_text_input.get_padded_text_and_cursor(adjusted_input_length);
        
        let url_line = self.add_color_to_env_keys(&padded_text);
        
        let url_paragraph = Paragraph::new(url_line).block(url_block);

        frame.render_widget(url_paragraph, request_header_layout[1]);

        match self.state {
            EditingRequestUrl => {
                frame.set_cursor(
                    request_header_layout[1].x + input_cursor_position as u16 + 2,
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


        let (should_render_params, should_render_result) = match self.request_view {
            RequestView::Normal => (true, true),
            RequestView::OnlyResult => (false, true),
            RequestView::OnlyParams => (true, false)
        };
        
        // REQUEST PARAMS

        if should_render_params {
            let params_block = Block::new().borders(Borders::RIGHT);
            let request_params_area = params_block.inner(request_main_layout[0]);

            frame.render_widget(params_block, request_main_layout[0]);
            self.render_request_params(frame, request_params_area, &request);
        }

        // REQUEST RESULT LAYOUT

        if should_render_result {
            let result_block = Block::new();
            let result_block_area = result_block.inner(request_main_layout[1]);

            frame.render_widget(result_block, request_main_layout[1]);
            self.render_request_result(frame, result_block_area, &request);
        }
    }
}