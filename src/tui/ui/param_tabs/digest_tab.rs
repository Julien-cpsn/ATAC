use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::app_states::AppState::{SelectedRequest, EditingRequestAuthDigestUsername, EditingRequestAuthDigestPassword, EditingRequestAuthDigestDomains, EditingRequestAuthDigestRealm, EditingRequestAuthDigestNonce, EditingRequestAuthDigestOpaque};
use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Position, Size, StatefulWidget, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use tui_scrollview::{ScrollView, ScrollViewState};
use crate::tui::utils::stateful::text_input::SingleLineTextInput;

impl App<'_> {
    pub(super) fn render_digest_tab(&mut self, frame: &mut Frame, area: Rect) {
        let scroll_view_size = Size::new(area.width.saturating_sub(1), 35);
        let mut digest_auth_scroll_view = ScrollView::new(scroll_view_size);

        let digest_auth_layout = Layout::new(
            Vertical,
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
        )
            .vertical_margin(1)
            .horizontal_margin(4)
            .split(digest_auth_scroll_view.area());

        let digest_auth = {
            let local_selected_request = self.get_selected_request_as_local();
            let selected_request = local_selected_request.read();

            selected_request.auth.get_digest().clone()
        };

        let mut stale_block = Block::new()
            .title("Stale ← →")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut algorithm_block = Block::new()
            .title("Algorithm ← →")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut qop_block = Block::new()
            .title("QOP ← →")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut user_hash_block = Block::new()
            .title("User hash ← →")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut charset_block = Block::new()
            .title("Charset ← →")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);


        let mut should_color_blocks = false;
        let mut should_display_cursor = false;

        // Prevent from rendering the cursor while no input text has been selected
        match self.state {
            SelectedRequest => {
                should_color_blocks = true;
            },
            EditingRequestAuthDigestUsername | EditingRequestAuthDigestPassword | EditingRequestAuthDigestDomains | EditingRequestAuthDigestRealm | EditingRequestAuthDigestNonce | EditingRequestAuthDigestOpaque => {
                should_color_blocks = true;
                should_display_cursor = true;
            },
            _ => {}
        };

        let mut highlight_username = false;
        let mut display_username_cursor = false;
        let mut highlight_password = false;
        let mut display_password_cursor = false;
        let mut highlight_domains = false;
        let mut display_domains_cursor = false;
        let mut highlight_realm = false;
        let mut display_realm_cursor = false;
        let mut highlight_nonce = false;
        let mut display_nonce_cursor = false;
        let mut highlight_opaque = false;
        let mut display_opaque_cursor = false;

        let mut stale_paragraph = Paragraph::new(digest_auth.stale.to_string())
            .fg(THEME.read().ui.font_color);
        let mut algorithm_paragraph = Paragraph::new(digest_auth.algorithm.to_string())
            .fg(THEME.read().ui.font_color);
        let mut qop_paragraph = Paragraph::new(digest_auth.qop.to_string())
            .fg(THEME.read().ui.font_color);
        let mut user_hash_paragraph = Paragraph::new(digest_auth.user_hash.to_string())
            .fg(THEME.read().ui.font_color);
        let mut charset_paragraph = Paragraph::new(digest_auth.charset.to_string())
            .fg(THEME.read().ui.font_color);

        let input_selected = self.auth_text_input_selection.selected;

        match input_selected {
            0 if should_color_blocks => {
                highlight_username = true;
                display_username_cursor = should_display_cursor;
            },
            1 if should_color_blocks => {
                highlight_password = true;
                display_password_cursor = should_display_cursor;
            },
            2 if should_color_blocks => {
                highlight_domains = true;
                display_domains_cursor = should_display_cursor;
            },
            3 if should_color_blocks => {
                highlight_realm = true;
                display_realm_cursor = should_display_cursor;
            },
            4 if should_color_blocks => {
                highlight_nonce = true;
                display_nonce_cursor = should_display_cursor;
            },
            5 if should_color_blocks => {
                highlight_opaque = true;
                display_opaque_cursor = should_display_cursor;
            },
            6 if should_color_blocks => {
                stale_paragraph = stale_paragraph.fg(THEME.read().others.selection_highlight_color);
                stale_block = stale_block.fg(THEME.read().others.selection_highlight_color);
            },
            7 if should_color_blocks => {
                algorithm_block = algorithm_block.fg(THEME.read().others.selection_highlight_color);
                algorithm_paragraph = algorithm_paragraph.fg(THEME.read().others.selection_highlight_color);
            },
            8 if should_color_blocks => {
                qop_block = qop_block.fg(THEME.read().others.selection_highlight_color);
                qop_paragraph = qop_paragraph.fg(THEME.read().others.selection_highlight_color);
            },
            9 if should_color_blocks => {
                user_hash_block = user_hash_block.fg(THEME.read().others.selection_highlight_color);
                user_hash_paragraph = user_hash_paragraph.fg(THEME.read().others.selection_highlight_color);
            },
            10 if should_color_blocks => {
                charset_block = charset_block.fg(THEME.read().others.selection_highlight_color);
                charset_paragraph = charset_paragraph.fg(THEME.read().others.selection_highlight_color);
            },
            _ => {}
        }

        self.auth_digest_username_text_input.highlight_text = highlight_username;
        self.auth_digest_username_text_input.highlight_block = highlight_username;
        self.auth_digest_username_text_input.display_cursor = display_username_cursor;
        self.auth_digest_password_text_input.highlight_text = highlight_password;
        self.auth_digest_password_text_input.highlight_block = highlight_password;
        self.auth_digest_password_text_input.display_cursor = display_password_cursor;
        self.auth_digest_domains_text_input.highlight_text = highlight_domains;
        self.auth_digest_domains_text_input.highlight_block = highlight_domains;
        self.auth_digest_domains_text_input.display_cursor = display_domains_cursor;
        self.auth_digest_realm_text_input.highlight_text = highlight_realm;
        self.auth_digest_realm_text_input.highlight_block = highlight_realm;
        self.auth_digest_realm_text_input.display_cursor = display_realm_cursor;
        self.auth_digest_nonce_text_input.highlight_text = highlight_nonce;
        self.auth_digest_nonce_text_input.highlight_block = highlight_nonce;
        self.auth_digest_nonce_text_input.display_cursor = display_nonce_cursor;
        self.auth_digest_opaque_text_input.highlight_text = highlight_opaque;
        self.auth_digest_opaque_text_input.highlight_block = highlight_opaque;
        self.auth_digest_opaque_text_input.display_cursor = display_opaque_cursor;
        stale_paragraph = stale_paragraph.block(stale_block);
        algorithm_paragraph = algorithm_paragraph.block(algorithm_block);
        qop_paragraph = qop_paragraph.block(qop_block);
        user_hash_paragraph = user_hash_paragraph.block(user_hash_block);
        charset_paragraph = charset_paragraph.block(charset_block);

        digest_auth_scroll_view.render_widget(SingleLineTextInput(&mut self.auth_digest_username_text_input), digest_auth_layout[0]);
        digest_auth_scroll_view.render_widget(SingleLineTextInput(&mut self.auth_digest_password_text_input), digest_auth_layout[1]);
        digest_auth_scroll_view.render_widget(SingleLineTextInput(&mut self.auth_digest_domains_text_input), digest_auth_layout[2]);
        digest_auth_scroll_view.render_widget(SingleLineTextInput(&mut self.auth_digest_realm_text_input), digest_auth_layout[3]);
        digest_auth_scroll_view.render_widget(SingleLineTextInput(&mut self.auth_digest_nonce_text_input), digest_auth_layout[4]);
        digest_auth_scroll_view.render_widget(SingleLineTextInput(&mut self.auth_digest_opaque_text_input), digest_auth_layout[5]);
        digest_auth_scroll_view.render_widget(stale_paragraph, digest_auth_layout[6]);
        digest_auth_scroll_view.render_widget(algorithm_paragraph, digest_auth_layout[7]);
        digest_auth_scroll_view.render_widget(qop_paragraph, digest_auth_layout[8]);
        digest_auth_scroll_view.render_widget(user_hash_paragraph, digest_auth_layout[9]);
        digest_auth_scroll_view.render_widget(charset_paragraph, digest_auth_layout[10]);

        let mut scrollbar_state = ScrollViewState::new();

        let scroll_adjustment = match area.height {
            0 => 0,
            _ => area.height / 3
        };

        let scroll_offset = match input_selected {
            0 => 0,
            _ => ((input_selected as u16 + 1) * 3).saturating_sub(area.height.saturating_sub(scroll_adjustment + 2))
        };

        scrollbar_state.set_offset(Position::new(0, scroll_offset));

        digest_auth_scroll_view.render(area, frame.buffer_mut(), &mut scrollbar_state)
    }
}
