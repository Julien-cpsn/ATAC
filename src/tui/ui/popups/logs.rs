use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::app::log::LOGS;
use crate::tui::utils::centered_rect::centered_rect;
use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation};
use ratatui::Frame;
use tracing::Level;

impl App<'_> {
    pub fn render_logs_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Logs")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color)
            .bg(THEME.read().ui.secondary_background_color);

        let area = centered_rect(120, 25, frame.area());

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);

        let logger_layout = Layout::new(
            Vertical,
            vec![
                Constraint::Fill(1)
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(2)
            .split(area);

        let logs = LOGS.lock();
        let mut lines = vec![];

        let main_background_color = THEME.read().ui.main_background_color;
        let font_color = THEME.read().ui.font_color;

        for log in logs.iter() {
            let level_color = match log.1 {
                Level::ERROR => Color::Red,
                Level::WARN => Color::Yellow,
                Level::INFO => Color::Green,
                Level::DEBUG => Color::Blue,
                Level::TRACE => Color::Magenta,
            };
            
            lines.push(
                Line::from(vec![
                    Span::raw(&log.0).fg(main_background_color),
                    Span::raw(format!(" {:>5} ", log.1)).fg(level_color),
                    Span::raw(format!("{}: ", log.2)).fg(main_background_color),
                    Span::raw(&log.3).fg(font_color),
                ])
            );
        }

        lines.reverse();

        let logs_paragraph = Paragraph::new(lines)
            .scroll((
                self.logs_vertical_scrollbar.scroll,
                self.logs_horizontal_scrollbar.scroll,
            ));

        frame.render_widget(logs_paragraph, logger_layout[0]);

        // For mysterious reasons, they won't show up...
        let logger_vertical_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::new().fg(font_color));
        let logger_horizontal_scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
            .style(Style::new().fg(font_color))
            .thumb_symbol("■");
        
        frame.render_stateful_widget(
            logger_vertical_scrollbar,
            area,
            &mut self.logs_vertical_scrollbar.state,
        );

        frame.render_stateful_widget(
            logger_horizontal_scrollbar,
            area,
            &mut self.logs_horizontal_scrollbar.state
        );
    }
}
