use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

use tracing::{trace, warn};
use crokey::{key, KeyCombination};
use lazy_static::lazy_static;
use nestify::nest;
use parking_lot::RwLock;
use ratatui::text::Span;
use serde::Deserialize;

use crate::app::app::App;
use crate::app::files::utils::expand_tilde;
use crate::panic_error;

#[derive(Default, Copy, Clone, Deserialize)]
pub struct KeyBindingsConfig {
    pub keybindings: KeyBindings,
}

nest! {
    #[derive(Copy, Clone, Deserialize)]
    pub struct KeyBindings {
        pub main_menu: #[derive(Copy, Clone, Deserialize)] pub struct MainMenu {
            /// ctrl-c is implemented by default
            pub exit: KeyCombination,

            pub expand_collection: KeyCombination,
            pub unselect_request: KeyCombination,

            pub move_request_up: KeyCombination,
            pub move_request_down: KeyCombination,

            pub next_environment: KeyCombination,

            pub display_cookies: KeyCombination,},

        pub generic: #[derive(Copy, Clone, Deserialize)] pub struct Generic {
            pub display_help: KeyCombination,

            pub text_inputs: #[derive(Copy, Clone, Deserialize)] pub struct TexInputs {
                /// Collection name, request name, URL, Header, Query param, Basic Auth, Bearer Token
                pub text_input: #[derive(Copy, Clone, Deserialize)] pub struct TextInput {
                    pub cancel: KeyCombination,
                    pub confirm: KeyCombination,

                    pub delete_backward: KeyCombination,
                    pub delete_forward: KeyCombination,

                    pub move_cursor_left: KeyCombination,
                    pub move_cursor_right: KeyCombination,
                },

                /// Request body
                pub text_area_mode: #[derive(Copy, Clone, PartialEq, Deserialize)] pub enum TextAreaMode {
                    VimEmulation,
                    Custom(CustomTextArea)
                }
            },

            /// Navigation in tables, popups, up and down in the collections list
            pub navigation: #[derive(Copy, Clone, Deserialize)] pub struct Navigation {
                pub move_cursor_up: KeyCombination,
                pub move_cursor_down: KeyCombination,
                pub move_cursor_left: KeyCombination,
                pub move_cursor_right: KeyCombination,

                pub alt_move_cursor_up: KeyCombination,
                pub alt_move_cursor_down: KeyCombination,
                #[allow(dead_code)]
                pub alt_move_cursor_left: KeyCombination,
                #[allow(dead_code)]
                pub alt_move_cursor_right: KeyCombination,
                
                pub go_back: KeyCombination,
                pub select: KeyCombination,
            },

            pub list_and_table_actions: #[derive(Copy, Clone, Deserialize)] pub struct ListAndTableActions {
                pub create_element: KeyCombination,
                pub delete_element: KeyCombination,
                pub edit_element: KeyCombination,
                /// Only used in the collections list (main menu)
                pub rename_element: KeyCombination,
                /// Only used in tables (Query params, headers, cookies)
                pub toggle_element: KeyCombination,
            }
        },

        pub request_selected: #[derive(Copy, Clone, Deserialize)] pub struct RequestSelected {
            pub param_next_tab: KeyCombination,
            pub change_url: KeyCombination,
            pub change_method: KeyCombination,
            pub request_settings: KeyCombination,

            pub next_view: KeyCombination,

            pub send_request: KeyCombination,
            pub alt_send_request: KeyCombination,
            
            pub param_tabs: #[derive(Copy, Clone, Deserialize)] pub struct ParamTabs {
                pub change_auth_method: KeyCombination,
                pub change_body_content_type: KeyCombination,
            },

            pub result_tabs: #[derive(Copy, Clone, Deserialize)] pub struct ResultTabs {
                pub scroll_up: KeyCombination,
                pub scroll_down: KeyCombination,
                pub scroll_left: KeyCombination,
                pub scroll_right: KeyCombination,

                pub yank_response_part: KeyCombination,

                /// Will use param_next_tab depending on the selected view
                pub result_next_tab: KeyCombination,
            }
        },
    }
}

lazy_static! {
    pub static ref KEY_BINDINGS: RwLock<KeyBindings> = RwLock::new(KeyBindings::default());
}

#[derive(Copy, Clone, PartialEq, Deserialize)]
pub struct CustomTextArea {
    pub quit_without_saving: KeyCombination,

    pub copy: KeyCombination,
    pub paste: KeyCombination,

    pub undo: KeyCombination,
    pub redo: KeyCombination,

    pub save_and_quit: KeyCombination,

    pub new_line: KeyCombination,
    pub indent: KeyCombination,

    pub delete_backward: KeyCombination,
    pub delete_forward: KeyCombination,

    pub skip_word_right: KeyCombination,
    pub skip_word_left: KeyCombination,

    pub move_cursor_up: KeyCombination,
    pub move_cursor_down: KeyCombination,
    pub move_cursor_left: KeyCombination,
    pub move_cursor_right: KeyCombination,
    pub move_cursor_line_start: KeyCombination,
    pub move_cursor_line_end: KeyCombination,
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            main_menu: MainMenu {
                exit: key!(q),

                expand_collection: key!(right),
                unselect_request: key!(left),

                move_request_up: key!(ctrl-up),
                move_request_down: key!(ctrl-down),

                next_environment: key!(e),

                display_cookies: key!(c),
            },

            generic: Generic {
                display_help: key!(Ctrl-h),

                text_inputs: TexInputs {
                    text_input: TextInput {
                        cancel: key!(esc),
                        confirm: key!(enter),

                        delete_backward: key!(delete),
                        delete_forward: key!(backspace),

                        move_cursor_left: key!(left),
                        move_cursor_right: key!(right),
                    },
                    text_area_mode: TextAreaMode::Custom(CustomTextArea::default()),
                },

                navigation: Navigation {
                    move_cursor_up: key!(up),
                    move_cursor_down: key!(down),
                    move_cursor_left: key!(left),
                    move_cursor_right: key!(right),

                    alt_move_cursor_up: key!(Up),
                    alt_move_cursor_down: key!(Down),
                    alt_move_cursor_left: key!(Left),
                    alt_move_cursor_right: key!(Right),
                    
                    go_back: key!(esc),
                    select: key!(enter),
                },
                list_and_table_actions: ListAndTableActions {
                    create_element: key!(n),
                    delete_element: key!(d),
                    edit_element: key!(enter),
                    rename_element: key!(r),
                    toggle_element: key!(t),
                },
            },

            request_selected: RequestSelected {
                param_next_tab: key!(tab),

                change_url: key!(u),
                change_method: key!(m),

                request_settings: key!(s),

                next_view: key!(v),

                // Used to be ctrl + enter, but it doesn't register right on many platforms
                // https://github.com/crossterm-rs/crossterm/issues/685
                send_request: key!(space),
                alt_send_request: key!(ctrl-enter),

                param_tabs: ParamTabs {
                    change_auth_method: key!(ctrl-a),
                    change_body_content_type: key!(ctrl-b),
                },
                result_tabs: ResultTabs {
                    scroll_up: key!(ctrl-up),
                    scroll_down: key!(ctrl-down),
                    scroll_left: key!(ctrl-left),
                    scroll_right: key!(ctrl-right),

                    yank_response_part: key!(y),

                    result_next_tab: key!(shift-backtab),
                },
            }
        }
    }
}

impl Default for CustomTextArea {
    fn default() -> Self {
        CustomTextArea {
            quit_without_saving: key!(esc),

            copy: key!(ctrl-c),
            paste: key!(ctrl-v),

            undo: key!(ctrl-z),
            redo: key!(ctrl-y),

            save_and_quit: key!(ctrl-s),

            new_line: key!(enter),
            indent: key!(tab),

            delete_backward: key!(delete),
            delete_forward: key!(backspace),

            skip_word_right: key!(ctrl-right),
            skip_word_left: key!(ctrl-left),

            move_cursor_up: key!(up),
            move_cursor_down: key!(down),
            move_cursor_left: key!(left),
            move_cursor_right: key!(right),
            move_cursor_line_start: key!(home),
            move_cursor_line_end: key!(end),
        }
    }
}

impl App<'_> {
    pub fn parse_key_bindings_file(&mut self) {
        let path = match env::var("ATAC_KEY_BINDINGS") {
            // If the ATAC_KEY_BINDINGS environment variable exists
            Ok(env_key_bindings) => expand_tilde(PathBuf::from(env_key_bindings)),
            Err(_) => {
                warn!("No key bindings file found, using default");
                return;
            }
        };

        trace!("Parsing key bindings file \"{}\"", path.display());

        let mut key_bindings_file = match OpenOptions::new().read(true).open(path) {
            Ok(key_bindings_file) => key_bindings_file,
            Err(e) => panic_error(format!("Could not open key bindings file\n\t{e}"))
        };

        let mut file_content = String::new();
        key_bindings_file.read_to_string(&mut file_content).expect("\tCould not read key bindings file");

        let config: KeyBindingsConfig = match toml::from_str(&file_content) {
            Ok(config) => config,
            Err(e) => panic_error(format!("Could not parse key bindings file\n\t{e}"))
        };

        *KEY_BINDINGS.write() = config.keybindings;

        trace!("Key bindings file parsed!");
    }
}

pub fn unique_key_and_help(help: Span<'static>, key: Span<'static>) -> Vec<Span<'static>> {
    if help.to_string() == key.to_string() {
        return vec![help];
    }
    else {
        return vec![help, Span::raw(" "), key]
    }
}
