use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

use crokey::KeyCombination;
use crokey::OneToThree::One;
use crossterm::event::{KeyCode, KeyModifiers};
use lazy_static::lazy_static;
use nestify::nest;
use serde::Deserialize;

use crate::app::app::App;
use crate::panic_error;

#[derive(Default, Copy, Clone, Deserialize)]
pub struct KeyBindingsConfig {
    pub keybindings: KeyBindings,
}

nest! {
    #[derive(Copy, Clone, Deserialize)]
    pub struct KeyBindings {
        pub main_menu: #[derive(Copy, Clone, Deserialize)] pub struct MainMenu {
            pub quit: KeyCombination,

            pub collections_expand: KeyCombination,
            pub unselect_request: KeyCombination,

            pub collections_move_request_up: KeyCombination,
            pub collections_move_request_down: KeyCombination,

            pub next_environment: KeyCombination,

            pub display_cookies: KeyCombination,

            pub display_help: KeyCombination,
        },

        pub generic: #[derive(Copy, Clone, Deserialize)] pub struct Generic {
            pub text_inputs: #[derive(Copy, Clone, Deserialize)] pub struct TexInputs {
                /// Collection name, request name, URL, Header, Query param, Basic Auth, Bearer Token
                pub small_text_inputs: #[derive(Copy, Clone, Deserialize)] pub struct SmallTextInputs {
                    pub cancel: KeyCombination,
                    pub validate: KeyCombination,

                    pub delete_forward: KeyCombination,
                    pub delete_backward: KeyCombination,

                    pub move_cursor_left: KeyCombination,
                    pub move_cursor_right: KeyCombination,
                }
            },

            /// Navigation in tables, popups, up and down in the collections list
            pub navigation: #[derive(Copy, Clone, Deserialize)] pub struct Navigation {
                pub move_cursor_up: KeyCombination,
                pub move_cursor_down: KeyCombination,
                pub move_cursor_left: KeyCombination,
                pub move_cursor_right: KeyCombination,

                pub go_back: KeyCombination,
                pub select: KeyCombination,
            },

            pub list_and_table_actions: #[derive(Copy, Clone, Deserialize)] pub struct ListAndTableActions {
                pub create_element: KeyCombination,
                pub delete_element: KeyCombination,
                /// Only used in the collections list (main menu)
                pub rename_element: KeyCombination,
                /// Only used in tables (Query params, headers, cookies)
                pub toggle_element: KeyCombination,
            }
        },

        pub cookies: #[derive(Copy, Clone, Deserialize)] pub struct Cookies {
            pub displaying_cookies: #[derive(Copy, Clone, Deserialize)] pub struct DisplayingCookies {
                pub delete_cookie: KeyCombination,
            },
            /*
            pub editing_cookies: #[derive(Deserialize)] pub struct EditingCookies {
                pub back_to_displaying_cookies: KeyCombination,
            }*/
        },

        pub request_selected: #[derive(Copy, Clone, Deserialize)] pub struct RequestSelected {
        },
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            main_menu: MainMenu {
                quit: KeyCombination { codes: One(KeyCode::Char('q')), modifiers: KeyModifiers::NONE },

                collections_expand: KeyCombination { codes: One(KeyCode::Right), modifiers: KeyModifiers::NONE },
                unselect_request: KeyCombination { codes: One(KeyCode::Left), modifiers: KeyModifiers::NONE },

                collections_move_request_up: KeyCombination { codes: One(KeyCode::Up), modifiers: KeyModifiers::CONTROL },
                collections_move_request_down: KeyCombination { codes: One(KeyCode::Down), modifiers: KeyModifiers::CONTROL },

                next_environment: KeyCombination { codes: One(KeyCode::Char('e')), modifiers: KeyModifiers::NONE },

                display_cookies: KeyCombination { codes: One(KeyCode::Char('c')), modifiers: KeyModifiers::NONE },

                display_help: KeyCombination { codes: One(KeyCode::Char('h')), modifiers: KeyModifiers::NONE },
            },

            generic: Generic {
                text_inputs: TexInputs {
                    small_text_inputs: SmallTextInputs {
                        cancel: KeyCombination { codes: One(KeyCode::Esc), modifiers: KeyModifiers::NONE },
                        validate: KeyCombination { codes: One(KeyCode::Enter), modifiers: KeyModifiers::NONE },

                        delete_forward: KeyCombination { codes: One(KeyCode::Delete), modifiers: KeyModifiers::NONE },
                        delete_backward: KeyCombination { codes: One(KeyCode::Backspace), modifiers: KeyModifiers::NONE },

                        move_cursor_left: KeyCombination { codes: One(KeyCode::Left), modifiers: KeyModifiers::NONE },
                        move_cursor_right: KeyCombination { codes: One(KeyCode::Right), modifiers: KeyModifiers::NONE },
                    },
                },

                navigation: Navigation {
                    move_cursor_up: KeyCombination { codes: One(KeyCode::Up), modifiers: KeyModifiers::NONE },
                    move_cursor_down: KeyCombination { codes: One(KeyCode::Down), modifiers: KeyModifiers::NONE },
                    move_cursor_left: KeyCombination { codes: One(KeyCode::Left), modifiers: KeyModifiers::NONE },
                    move_cursor_right: KeyCombination { codes: One(KeyCode::Right), modifiers: KeyModifiers::NONE },

                    go_back: KeyCombination { codes: One(KeyCode::Esc), modifiers: KeyModifiers::NONE },
                    select: KeyCombination { codes: One(KeyCode::Enter), modifiers: KeyModifiers::NONE },
                },
                list_and_table_actions: ListAndTableActions {
                    create_element: KeyCombination { codes: One(KeyCode::Char('n')), modifiers: KeyModifiers::NONE },
                    delete_element: KeyCombination { codes: One(KeyCode::Char('d')), modifiers: KeyModifiers::NONE },
                    rename_element: KeyCombination { codes: One(KeyCode::Char('r')), modifiers: KeyModifiers::NONE },
                    toggle_element: KeyCombination { codes: One(KeyCode::Char('t')), modifiers: KeyModifiers::NONE },
                },
            },

            cookies: Cookies {
                displaying_cookies: DisplayingCookies {
                    delete_cookie: KeyCombination { codes: One(KeyCode::Char('d')), modifiers: KeyModifiers::NONE },
                },
                /*
                editing_cookies: EditingCookies {

                }*/
            },

            request_selected: RequestSelected {
            }
        }
    }
}

impl App<'_> {
    pub fn parse_key_bindings_file(&mut self) {
        let path = match env::var("ATAC_KEY_BINDINGS") {
            // If the ATAC_KEY_BINDINGS environment variable exists
            Ok(env_key_bindings) => PathBuf::from(env_key_bindings),
            Err(_) => {
                println!("No key bindings file found\n");
                return;
            }
        };

        println!("Parsing key bindings file: {}", path.display());

        let mut key_bindings_file = OpenOptions::new()
            .read(true)
            .open(path)
            .expect("\tCould not open key bindings file");

        let mut file_content = String::new();
        key_bindings_file.read_to_string(&mut file_content).expect("\tCould not read key bindings file");

        let config: KeyBindingsConfig = match toml::from_str(&file_content) {
            Ok(config) => config,
            Err(e) => panic_error(format!("Could not parse key bindings file\n\t{e}"))
        };

        *KEY_BINDINGS = config.keybindings;

        println!("Key bindings file parsed!\n");
    }
}

lazy_static! {
    pub static ref KEY_BINDINGS: KeyBindings = KeyBindings::default();
}