use edtui::EditorMode;
use crate::app::app::App;
use crate::app::files::key_bindings::{TextAreaMode, KEY_BINDINGS};

impl App<'_> {
    pub fn reset_inputs_mode(&mut self) {
        self.env_editor_table.selection_text_input.reset_mode();
        self.new_collection_input.reset_mode();
        self.new_request_popup.text_input.reset_mode();
        self.rename_collection_input.reset_mode();
        self.rename_request_input.reset_mode();
        self.url_text_input.reset_mode();
        self.query_params_table.selection_text_input.reset_mode();
        self.auth_basic_username_text_input.reset_mode();
        self.auth_basic_password_text_input.reset_mode();
        self.auth_bearer_token_text_input.reset_mode();
        self.auth_jwt_secret_text_input.reset_mode();
        self.auth_jwt_payload_text_area.reset_mode();
        self.auth_digest_username_text_input.reset_mode();
        self.auth_digest_password_text_input.reset_mode();
        self.auth_digest_domains_text_input.reset_mode();
        self.auth_digest_realm_text_input.reset_mode();
        self.auth_digest_nonce_text_input.reset_mode();
        self.auth_digest_opaque_text_input.reset_mode();
        self.headers_table.selection_text_input.reset_mode();
        self.body_text_area.reset_mode();
        self.body_form_table.selection_text_input.reset_mode();
        self.body_file_text_input.reset_mode();
        self.message_text_area.reset_mode();
        self.script_console.pre_request_text_area.reset_mode();
        self.script_console.post_request_text_area.reset_mode();
    }

    pub fn clear_inputs(&mut self) {
        self.env_editor_table.selection_text_input.clear();
        self.new_collection_input.clear();
        self.new_request_popup.text_input.clear();
        self.rename_collection_input.clear();
        self.rename_request_input.clear();
        self.url_text_input.clear();
        self.query_params_table.selection_text_input.clear();
        self.auth_basic_username_text_input.clear();
        self.auth_basic_password_text_input.clear();
        self.auth_bearer_token_text_input.clear();
        self.auth_jwt_secret_text_input.clear();
        self.auth_jwt_payload_text_area.clear();
        self.auth_digest_username_text_input.clear();
        self.auth_digest_password_text_input.clear();
        self.auth_digest_domains_text_input.clear();
        self.auth_digest_realm_text_input.clear();
        self.auth_digest_nonce_text_input.clear();
        self.auth_digest_opaque_text_input.clear();
        self.headers_table.selection_text_input.clear();
        self.body_text_area.clear();
        self.body_form_table.selection_text_input.clear();
        self.body_file_text_input.clear();
        self.message_text_area.clear();
        self.script_console.pre_request_text_area.clear();
        self.script_console.post_request_text_area.clear();
    }

    pub fn reset_cursors(&mut self) {
        self.env_editor_table.selection_text_input.reset_cursor_position();
        self.new_collection_input.reset_cursor_position();
        self.new_request_popup.text_input.reset_cursor_position();
        self.rename_collection_input.reset_cursor_position();
        self.rename_request_input.reset_cursor_position();
        self.url_text_input.reset_cursor_position();
        self.query_params_table.selection_text_input.reset_cursor_position();
        self.auth_basic_username_text_input.reset_cursor_position();
        self.auth_basic_password_text_input.reset_cursor_position();
        self.auth_bearer_token_text_input.reset_cursor_position();
        self.auth_jwt_secret_text_input.reset_cursor_position();
        self.auth_jwt_payload_text_area.reset_cursor_position();
        self.auth_digest_username_text_input.reset_cursor_position();
        self.auth_digest_password_text_input.reset_cursor_position();
        self.auth_digest_domains_text_input.reset_cursor_position();
        self.auth_digest_realm_text_input.reset_cursor_position();
        self.auth_digest_nonce_text_input.reset_cursor_position();
        self.auth_digest_opaque_text_input.reset_cursor_position();
        self.headers_table.selection_text_input.reset_cursor_position();
        self.body_text_area.reset_cursor_position();
        self.body_form_table.selection_text_input.reset_cursor_position();
        self.body_file_text_input.reset_cursor_position();
        self.message_text_area.reset_cursor_position();
        self.script_console.pre_request_text_area.reset_cursor_position();
        self.script_console.post_request_text_area.reset_cursor_position();

        self.env_editor_table.selection_text_input.reset_selection();
        self.new_collection_input.reset_selection();
        self.new_request_popup.text_input.reset_selection();
        self.rename_collection_input.reset_selection();
        self.rename_request_input.reset_selection();
        self.url_text_input.reset_selection();
        self.query_params_table.selection_text_input.reset_selection();
        self.auth_basic_username_text_input.reset_selection();
        self.auth_basic_password_text_input.reset_selection();
        self.auth_bearer_token_text_input.reset_selection();
        self.auth_jwt_secret_text_input.reset_selection();
        self.auth_jwt_payload_text_area.reset_selection();
        self.auth_digest_username_text_input.reset_selection();
        self.auth_digest_password_text_input.reset_selection();
        self.auth_digest_domains_text_input.reset_selection();
        self.auth_digest_realm_text_input.reset_selection();
        self.auth_digest_nonce_text_input.reset_selection();
        self.auth_digest_opaque_text_input.reset_selection();
        self.headers_table.selection_text_input.reset_selection();
        self.body_text_area.reset_selection();
        self.body_form_table.selection_text_input.reset_selection();
        self.body_file_text_input.reset_selection();
        self.message_text_area.reset_selection();
        self.script_console.pre_request_text_area.reset_selection();
        self.script_console.post_request_text_area.reset_selection();
    }

    pub fn update_text_inputs_handler(&mut self) {
        let default_mode = match KEY_BINDINGS.read().generic.text_input.mode {
            TextAreaMode::Vim => EditorMode::Normal,
            TextAreaMode::Emacs | TextAreaMode::Default | TextAreaMode::Custom(_) => EditorMode::Insert,
        };

        self.env_editor_table.selection_text_input.default_mode = EditorMode::Insert;
        self.new_collection_input.default_mode = default_mode;
        self.new_request_popup.text_input.default_mode = default_mode;
        self.rename_collection_input.default_mode = default_mode;
        self.rename_request_input.default_mode = default_mode;
        self.url_text_input.default_mode = default_mode;
        self.query_params_table.selection_text_input.default_mode = EditorMode::Insert;
        self.auth_basic_username_text_input.default_mode = default_mode;
        self.auth_basic_password_text_input.default_mode = default_mode;
        self.auth_bearer_token_text_input.default_mode = default_mode;
        self.auth_jwt_secret_text_input.default_mode = default_mode;
        self.auth_jwt_payload_text_area.default_mode = default_mode;
        self.auth_digest_username_text_input.default_mode = default_mode;
        self.auth_digest_password_text_input.default_mode = default_mode;
        self.auth_digest_domains_text_input.default_mode = default_mode;
        self.auth_digest_realm_text_input.default_mode = default_mode;
        self.auth_digest_nonce_text_input.default_mode = default_mode;
        self.auth_digest_opaque_text_input.default_mode = default_mode;
        self.headers_table.selection_text_input.default_mode = EditorMode::Insert;
        self.body_text_area.default_mode = default_mode;
        self.body_form_table.selection_text_input.default_mode = EditorMode::Insert;
        self.body_file_text_input.default_mode = default_mode;
        self.message_text_area.default_mode = default_mode;
        self.script_console.pre_request_text_area.default_mode = default_mode;
        self.script_console.post_request_text_area.default_mode = default_mode;

        self.reset_inputs_mode();

        self.env_editor_table.selection_text_input.is_single_line = true;
        self.new_collection_input.is_single_line = true;
        self.new_request_popup.text_input.is_single_line = true;
        self.rename_collection_input.is_single_line = true;
        self.rename_request_input.is_single_line = true;
        self.url_text_input.is_single_line = true;
        self.query_params_table.selection_text_input.is_single_line = true;
        self.auth_basic_username_text_input.is_single_line = true;
        self.auth_basic_password_text_input.is_single_line = true;
        self.auth_bearer_token_text_input.is_single_line = true;
        self.auth_jwt_secret_text_input.is_single_line = true;
        self.auth_jwt_payload_text_area.is_single_line = false;
        self.auth_digest_username_text_input.is_single_line = true;
        self.auth_digest_password_text_input.is_single_line = true;
        self.auth_digest_domains_text_input.is_single_line = true;
        self.auth_digest_realm_text_input.is_single_line = true;
        self.auth_digest_nonce_text_input.is_single_line = true;
        self.auth_digest_opaque_text_input.is_single_line = true;
        self.headers_table.selection_text_input.is_single_line = true;
        self.body_text_area.is_single_line = false;
        self.body_form_table.selection_text_input.is_single_line = true;
        self.body_file_text_input.is_single_line = true;
        self.message_text_area.is_single_line = false;
        self.script_console.pre_request_text_area.is_single_line = false;
        self.script_console.post_request_text_area.is_single_line = false;

        self.env_editor_table.selection_text_input.insert_mode_only = true;
        self.query_params_table.selection_text_input.insert_mode_only = true;
        self.headers_table.selection_text_input.insert_mode_only = true;
        self.body_form_table.selection_text_input.insert_mode_only = true;

        self.env_editor_table.selection_text_input.update_handler();
        self.new_collection_input.update_handler();
        self.new_request_popup.text_input.update_handler();
        self.rename_collection_input.update_handler();
        self.rename_request_input.update_handler();
        self.url_text_input.update_handler();
        self.query_params_table.selection_text_input.update_handler();
        self.auth_basic_username_text_input.update_handler();
        self.auth_basic_password_text_input.update_handler();
        self.auth_bearer_token_text_input.update_handler();
        self.auth_jwt_secret_text_input.update_handler();
        self.auth_jwt_payload_text_area.update_handler();
        self.auth_digest_username_text_input.update_handler();
        self.auth_digest_password_text_input.update_handler();
        self.auth_digest_domains_text_input.update_handler();
        self.auth_digest_realm_text_input.update_handler();
        self.auth_digest_nonce_text_input.update_handler();
        self.auth_digest_opaque_text_input.update_handler();
        self.headers_table.selection_text_input.update_handler();
        self.body_text_area.update_handler();
        self.body_form_table.selection_text_input.update_handler();
        self.body_file_text_input.update_handler();
        self.message_text_area.update_handler();
        self.script_console.pre_request_text_area.update_handler();
        self.script_console.post_request_text_area.update_handler();
    }
}