use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::layout::Direction::Vertical;
use ratatui::prelude::{Color, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use crate::app::app::App;
use crate::request::request::Request;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum RequestParamsTabs {
    #[default]
    #[strum(to_string = "Params")]
    Params,
    #[strum(to_string = "Auth")]
    Auth,
    #[strum(to_string = "Headers")]
    Headers,
    #[strum(to_string = "Body")]
    Body,
    #[strum(to_string = "Cookies")]
    Cookies
}

impl App<'_> {
    pub fn next_request_param_tab(&mut self) {
        self.request_param_tab = match self.request_param_tab {
            RequestParamsTabs::Params => RequestParamsTabs::Auth,
            RequestParamsTabs::Auth => RequestParamsTabs::Headers,
            RequestParamsTabs::Headers => RequestParamsTabs::Body,
            RequestParamsTabs::Body => RequestParamsTabs::Cookies,
            RequestParamsTabs::Cookies => RequestParamsTabs::Params
        };
    }

    pub(crate) fn render_request_params(&mut self, frame: &mut Frame, rect: Rect, request: &Request) {
        let request_params_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(2),
                Constraint::Fill(1)
            ]
        )
            .split(rect);

        // REQUEST PARAM TABS

        let param_tabs = RequestParamsTabs::iter().map(|tab| tab.to_string());
        let selected_param_tab_index = self.request_param_tab as usize;

        let params_tabs = Tabs::new(param_tabs)
            .highlight_style(Style::default().yellow())
            .select(selected_param_tab_index)
            .block(
                Block::new().borders(Borders::BOTTOM)
            );

        frame.render_widget(params_tabs, request_params_layout[0]);

        // REQUEST PARAM TABS CONTENT

        match self.request_param_tab {
            RequestParamsTabs::Params => {}
            RequestParamsTabs::Auth => {}
            RequestParamsTabs::Headers => {}
            RequestParamsTabs::Body => {
                match request.body {
                    None => {
                        let body_paragraph = Paragraph::new("\nNo body").centered();
                        frame.render_widget(body_paragraph, request_params_layout[1]);
                    }
                    Some(_) => {
                        self.body_text_area.set_line_number_style(Style::new().fg(Color::DarkGray));
                        frame.render_widget(self.body_text_area.widget(), request_params_layout[1]);
                    }
                }
            }
            RequestParamsTabs::Cookies => {}
        }
    }
}