use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::app_states::AppState::{SelectedRequest, EditingRequestAuthDigestUsername, EditingRequestAuthDigestPassword, EditingRequestAuthDigestDomains, EditingRequestAuthDigestRealm, EditingRequestAuthDigestNonce, EditingRequestAuthDigestOpaque};
use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Position, Size, StatefulWidget, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use tui_scrollview::{ScrollView, ScrollViewState};

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

        let mut username_block = Block::new()
            .title("Username")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);


        let mut password_block = Block::new()
            .title("Password")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut domains_block = Block::new()
            .title("Domains")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut realm_block = Block::new()
            .title("Realm")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut nonce_block = Block::new()
            .title("Nonce")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

        let mut opaque_block = Block::new()
            .title("Opaque")
            .borders(Borders::ALL)
            .fg(THEME.read().ui.main_foreground_color);

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

        let username_adjusted_input_length = digest_auth_layout[0].width as usize - 2;
        let (username_padded_text, username_input_cursor_position) = self.auth_digest_username_text_input.get_padded_text_and_cursor(username_adjusted_input_length);

        let password_adjusted_input_length = digest_auth_layout[1].width as usize - 2;
        let (password_padded_text, password_input_cursor_position) = self.auth_digest_password_text_input.get_padded_text_and_cursor(password_adjusted_input_length);

        let domains_adjusted_input_length = digest_auth_layout[2].width as usize - 2;
        let (domains_padded_text, domains_input_cursor_position) = self.auth_digest_domains_text_input.get_padded_text_and_cursor(domains_adjusted_input_length);

        let realm_adjusted_input_length = digest_auth_layout[3].width as usize - 2;
        let (realm_padded_text, realm_input_cursor_position) = self.auth_digest_realm_text_input.get_padded_text_and_cursor(realm_adjusted_input_length);

        let nonce_adjusted_input_length = digest_auth_layout[4].width as usize - 2;
        let (nonce_padded_text, nonce_input_cursor_position) = self.auth_digest_nonce_text_input.get_padded_text_and_cursor(nonce_adjusted_input_length);

        let opaque_adjusted_input_length = digest_auth_layout[5].width as usize - 2;
        let (opaque_padded_text, opaque_input_cursor_position) = self.auth_digest_opaque_text_input.get_padded_text_and_cursor(opaque_adjusted_input_length);


        let username_line = self.tui_add_color_to_env_keys(&username_padded_text);
        let password_line = self.tui_add_color_to_env_keys(&password_padded_text);
        let domains_line = self.tui_add_color_to_env_keys(&domains_padded_text);
        let realm_line = self.tui_add_color_to_env_keys(&realm_padded_text);
        let nonce_line = self.tui_add_color_to_env_keys(&nonce_padded_text);
        let opaque_line = self.tui_add_color_to_env_keys(&opaque_padded_text);


        let mut username_paragraph = Paragraph::new(username_line)
            .fg(THEME.read().ui.font_color);
        let mut password_paragraph = Paragraph::new(password_line)
            .fg(THEME.read().ui.font_color);
        let mut domains_paragraph = Paragraph::new(domains_line)
            .fg(THEME.read().ui.font_color);
        let mut realm_paragraph = Paragraph::new(realm_line)
            .fg(THEME.read().ui.font_color);
        let mut nonce_paragraph = Paragraph::new(nonce_line)
            .fg(THEME.read().ui.font_color);
        let mut opaque_paragraph = Paragraph::new(opaque_line)
            .fg(THEME.read().ui.font_color);
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

        let input_cursor_position = match input_selected {
            0 if should_color_blocks => {
                username_block = username_block.fg(THEME.read().others.selection_highlight_color);
                username_paragraph = username_paragraph.fg(THEME.read().others.selection_highlight_color);

                username_input_cursor_position
            },
            1 if should_color_blocks => {
                password_block = password_block.fg(THEME.read().others.selection_highlight_color);
                password_paragraph = password_paragraph.fg(THEME.read().others.selection_highlight_color);

                password_input_cursor_position
            },
            2 if should_color_blocks => {
                domains_block = domains_block.fg(THEME.read().others.selection_highlight_color);
                domains_paragraph = domains_paragraph.fg(THEME.read().others.selection_highlight_color);

                domains_input_cursor_position
            },
            3 if should_color_blocks => {
                realm_block = realm_block.fg(THEME.read().others.selection_highlight_color);
                realm_paragraph = realm_paragraph.fg(THEME.read().others.selection_highlight_color);

                realm_input_cursor_position
            },
            4 if should_color_blocks => {
                nonce_block = nonce_block.fg(THEME.read().others.selection_highlight_color);
                nonce_paragraph = nonce_paragraph.fg(THEME.read().others.selection_highlight_color);

                nonce_input_cursor_position
            },
            5 if should_color_blocks => {
                opaque_block = opaque_block.fg(THEME.read().others.selection_highlight_color);
                opaque_paragraph = opaque_paragraph.fg(THEME.read().others.selection_highlight_color);

                opaque_input_cursor_position
            },
            6 if should_color_blocks => {
                stale_paragraph = stale_paragraph.fg(THEME.read().others.selection_highlight_color);
                stale_block = stale_block.fg(THEME.read().others.selection_highlight_color);

                0
            },
            7 if should_color_blocks => {
                algorithm_block = algorithm_block.fg(THEME.read().others.selection_highlight_color);
                algorithm_paragraph = algorithm_paragraph.fg(THEME.read().others.selection_highlight_color);

                0
            },
            8 if should_color_blocks => {
                qop_block = qop_block.fg(THEME.read().others.selection_highlight_color);
                qop_paragraph = qop_paragraph.fg(THEME.read().others.selection_highlight_color);

                0
            },
            9 if should_color_blocks => {
                user_hash_block = user_hash_block.fg(THEME.read().others.selection_highlight_color);
                user_hash_paragraph = user_hash_paragraph.fg(THEME.read().others.selection_highlight_color);

                0
            },
            10 if should_color_blocks => {
                charset_block = charset_block.fg(THEME.read().others.selection_highlight_color);
                charset_paragraph = charset_paragraph.fg(THEME.read().others.selection_highlight_color);

                0
            },
            _ => 0
        };

        username_paragraph = username_paragraph.block(username_block);
        password_paragraph = password_paragraph.block(password_block);
        domains_paragraph = domains_paragraph.block(domains_block);
        realm_paragraph = realm_paragraph.block(realm_block);
        nonce_paragraph = nonce_paragraph.block(nonce_block);
        opaque_paragraph = opaque_paragraph.block(opaque_block);
        stale_paragraph = stale_paragraph.block(stale_block);
        algorithm_paragraph = algorithm_paragraph.block(algorithm_block);
        qop_paragraph = qop_paragraph.block(qop_block);
        user_hash_paragraph = user_hash_paragraph.block(user_hash_block);
        charset_paragraph = charset_paragraph.block(charset_block);

        digest_auth_scroll_view.render_widget(username_paragraph, digest_auth_layout[0]);
        digest_auth_scroll_view.render_widget(password_paragraph, digest_auth_layout[1]);
        digest_auth_scroll_view.render_widget(domains_paragraph, digest_auth_layout[2]);
        digest_auth_scroll_view.render_widget(realm_paragraph, digest_auth_layout[3]);
        digest_auth_scroll_view.render_widget(nonce_paragraph, digest_auth_layout[4]);
        digest_auth_scroll_view.render_widget(opaque_paragraph, digest_auth_layout[5]);
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

        if should_display_cursor {
            frame.set_cursor_position(Position::new(
                area.x + digest_auth_layout[input_selected].x + input_cursor_position as u16 + 1,
                (area.y + digest_auth_layout[input_selected].y + 1).saturating_sub(scroll_offset)
            ));
        }

        digest_auth_scroll_view.render(area, frame.buffer_mut(), &mut scrollbar_state)
    }
}
