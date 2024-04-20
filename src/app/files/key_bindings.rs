use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use std::sync::RwLock;

use crokey::{key, KeyCombination};
use lazy_static::lazy_static;
use nestify::nest;
use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Span};
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
            /// ctrl-c is implemented by default
            pub quit: KeyCombination,

            pub collections_expand: KeyCombination,
            pub unselect_request: KeyCombination,

            pub move_request_up: KeyCombination,
            pub move_request_down: KeyCombination,

            pub next_environment: KeyCombination,

            pub display_cookies: KeyCombination,

            pub display_help: KeyCombination,
        },

        pub generic: #[derive(Copy, Clone, Deserialize)] pub struct Generic {
            pub text_inputs: #[derive(Copy, Clone, Deserialize)] pub struct TexInputs {
                /// Collection name, request name, URL, Header, Query param, Basic Auth, Bearer Token
                pub text_input: #[derive(Copy, Clone, Deserialize)] pub struct TextInput {
                    pub cancel: KeyCombination,
                    pub validate: KeyCombination,

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
            pub next_tab: KeyCombination,
            pub change_url: KeyCombination,
            pub change_method: KeyCombination,
            pub request_settings: KeyCombination,

            pub next_view: KeyCombination,

            pub send_request: KeyCombination,
            pub secondary_send_request: KeyCombination,

            pub param_tabs: #[derive(Copy, Clone, Deserialize)] pub struct ParamTabs {
                pub change_auth_method: KeyCombination,
                pub change_body_content_type: KeyCombination,
            },

            pub result_tabs: #[derive(Copy, Clone, Deserialize)] pub struct ResultTabs {
                pub scroll_up: KeyCombination,
                pub scroll_down: KeyCombination,
                pub scroll_left: KeyCombination,
                pub scroll_right: KeyCombination,

                /// Will use next_tab depending on the selected view
                pub secondary_next_tab: KeyCombination,
            }
        },
    }
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

    pub delete: KeyCombination,
    pub backspace: KeyCombination,

    pub skip_word_cursor_right: KeyCombination,
    pub skip_word_cursor_left: KeyCombination,

    pub move_cursor_up: KeyCombination,
    pub move_cursor_down: KeyCombination,
    pub move_cursor_left: KeyCombination,
    pub move_cursor_right: KeyCombination,
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            main_menu: MainMenu {
                quit: key!(q),

                collections_expand: key!(right),
                unselect_request: key!(left),

                move_request_up: key!(ctrl-up),
                move_request_down: key!(ctrl-down),

                next_environment: key!(e),

                display_cookies: key!(c),

                display_help: key!(h),
            },

            generic: Generic {
                text_inputs: TexInputs {
                    text_input: TextInput {
                        cancel: key!(esc),
                        validate: key!(enter),

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
                next_tab: key!(tab),

                change_url: key!(u),
                change_method: key!(m),

                request_settings: key!(s),

                next_view: key!(v),

                // Used to be ctrl + enter, but it doesn't register right on many platforms
                // https://github.com/crossterm-rs/crossterm/issues/685
                send_request: key!(space),
                secondary_send_request: key!(ctrl-enter),

                param_tabs: ParamTabs {
                    change_auth_method: key!(ctrl-a),
                    change_body_content_type: key!(ctrl-b),
                },
                result_tabs: ResultTabs {
                    scroll_up: key!(ctrl-up),
                    scroll_down: key!(ctrl-down),
                    scroll_left: key!(ctrl-left),
                    scroll_right: key!(ctrl-right),

                    secondary_next_tab: key!(shift-backtab),
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
            delete: key!(delete),

            backspace: key!(backspace),

            skip_word_cursor_right: key!(ctrl-right),
            skip_word_cursor_left: key!(ctrl-left),

            move_cursor_up: key!(up),
            move_cursor_down: key!(down),
            move_cursor_left: key!(left),
            move_cursor_right: key!(right),
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

        *KEY_BINDINGS.write().unwrap() = config.keybindings;

        println!("Key bindings file parsed!\n");
    }
}

fn unique_help(name: Span<'static>, value: Span<'static>) -> Vec<Span<'static>> {
    if name.to_string() == value.to_string() {
        return vec![name];
    }
    else {
        vec![name, Span::raw(" "), value]
    }
}

pub fn update_key_helpers() {
    let dark_black = Color::Rgb(50, 50, 50);

    let space = Span::raw(" ");

    let exit = "Exit".bg(dark_black);

    let main_menu = "Main menu".bg(dark_black);
    let send = "Send".bg(dark_black);
    let next_tab = "Next tab".bg(dark_black);
    let url = "Url".bg(dark_black);
    let method = "Method".bg(dark_black);
    let help = "Help".bg(dark_black);

    let quit = "Quit".bg(dark_black);
    let save = "Save".bg(dark_black);
    let indent = "Indent".bg(dark_black);

    let cancel = "Cancel".bg(dark_black);
    let validate = "Validate".bg(dark_black);

    let up = "Up".bg(dark_black);
    let down = "Down".bg(dark_black);
    let left = "Left".bg(dark_black);
    let right = "Right".bg(dark_black);

    let copy = "Copy".bg(dark_black);
    let paste = "Paste".bg(dark_black);

    let delete = "Delete".bg(dark_black);
    
    let key_bindings = KEY_BINDINGS.read().unwrap();

    // Exit Ctrl-c q Help h
    *MAIN_MENU_KEYS.write().unwrap() = Line::from(vec![
        exit,
        " Ctrl-c ".dark_gray(),
        key_bindings.main_menu.quit.to_string().dark_gray(),
        space.clone(),
        help.clone(),
        space.clone(),
        key_bindings.main_menu.display_help.to_string().dark_gray()
    ]);

    // Cancel Esc Validate Enter Left Right Copy ctrl-c Paste ctrl-v
    *TEXT_INPUT_KEYS.write().unwrap() = Line::from(vec![
        vec![
            cancel.clone(),
            space.clone(),
            key_bindings.generic.text_inputs.text_input.cancel.to_string().dark_gray(),
            space.clone(),
            validate.clone(),
            space.clone(),
            key_bindings.generic.text_inputs.text_input.validate.to_string().dark_gray(),
            space.clone()
        ],
        unique_help(left.clone(), key_bindings.generic.text_inputs.text_input.move_cursor_left.to_string().dark_gray()),
        vec![space.clone()],
        unique_help(right.clone(), key_bindings.generic.text_inputs.text_input.move_cursor_right.to_string().dark_gray()),
        vec![
            space.clone(),
            copy.clone(),
            space.clone(),
            "ctrl-c".dark_gray(),
            space.clone(),
            paste.clone(),
            space.clone(),
            "ctrl-v".dark_gray(),
       ]
    ].concat());

    *TEXT_AREA_INPUT_KEYS.write().unwrap() = match key_bindings.generic.text_inputs.text_area_mode {
        TextAreaMode::VimEmulation => Line::from(vec![
            quit.clone(),
            space.clone(),
            "q".dark_gray(),
            space.clone(),
            save.clone(),
            space.clone(),
            "Ctrl-s".dark_gray()
        ]),
        // Quit Esc Save Ctrl-s Indent Tab Up Down Left Right Copy Ctrl-c Paste Ctrl-v
        TextAreaMode::Custom(custom_text_area) => Line::from(vec![
            vec![
                quit.clone(),
                space.clone(),
                custom_text_area.quit_without_saving.to_string().dark_gray(),
                space.clone(),
                save.clone(),
                space.clone(),
                custom_text_area.save_and_quit.to_string().dark_gray(),
                space.clone(),
                indent.clone(),
                space.clone(),
                custom_text_area.indent.to_string().dark_gray(),
                space.clone(),
            ],
            unique_help(up.clone(), custom_text_area.move_cursor_up.to_string().dark_gray()),
            vec![space.clone()],
            unique_help(down.clone(), custom_text_area.move_cursor_down.to_string().dark_gray()),
            vec![space.clone()],
            unique_help(left.clone(), custom_text_area.move_cursor_left.to_string().dark_gray()),
            vec![space.clone()],
            unique_help(right.clone(), custom_text_area.move_cursor_right.to_string().dark_gray()),
            vec![
                space.clone(),
                copy,
                space.clone(),
                custom_text_area.copy.to_string().dark_gray(),
                space.clone(),
                paste,
                space.clone(),
                custom_text_area.paste.to_string().dark_gray(),
            ]
        ].concat())
    };

    // Cancel Esc Validate Enter Up Down Left Right
    *NAVIGATION_KEYS.write().unwrap() = Line::from(vec![
        vec![
            cancel.clone(),
            space.clone(),
            key_bindings.generic.navigation.go_back.to_string().dark_gray(),
            space.clone(),
            validate.clone(),
            space.clone(),
            key_bindings.generic.navigation.select.to_string().dark_gray(),
            space.clone(),
        ],
        unique_help(up.clone(), key_bindings.generic.navigation.move_cursor_up.to_string().dark_gray()),
        vec![space.clone()],
        unique_help(down.clone(), key_bindings.generic.navigation.move_cursor_down.to_string().dark_gray()),
        vec![space.clone()],
        unique_help(left.clone(), key_bindings.generic.navigation.move_cursor_left.to_string().dark_gray()),
        vec![space.clone()],
        unique_help(right.clone(), key_bindings.generic.navigation.move_cursor_right.to_string().dark_gray()),
    ].concat());

    // Cancel Esc Validate Enter Left Right
    *VALIDATION_KEYS.write().unwrap() = Line::from(vec![
        vec![
            cancel.clone(),
            space.clone(),
            key_bindings.generic.navigation.go_back.to_string().dark_gray(),
            space.clone(),
            validate,
            space.clone(),
            key_bindings.generic.navigation.select.to_string().dark_gray(),
            space.clone(),
        ],
        unique_help(left.clone(), key_bindings.generic.navigation.move_cursor_left.to_string().dark_gray()),
        vec![space.clone()],
        unique_help(right.clone(), key_bindings.generic.navigation.move_cursor_right.to_string().dark_gray()),
    ].concat());

    // Main menu Esc Send Space Next tab Tab Shift-BackTab Url u Method m Help h
    *REQUEST_SELECTED_KEYS.write().unwrap() = Line::from(vec![
        main_menu,
        space.clone(),
        key_bindings.generic.navigation.go_back.to_string().dark_gray(),
        space.clone(),
        send,
        space.clone(),
        key_bindings.request_selected.send_request.to_string().dark_gray(),
        space.clone(),
        next_tab,
        space.clone(),
        key_bindings.request_selected.next_tab.to_string().dark_gray(),
        space.clone(),
        key_bindings.request_selected.result_tabs.secondary_next_tab.to_string().dark_gray(),
        space.clone(),
        url,
        space.clone(),
        key_bindings.request_selected.change_url.to_string().dark_gray(),
        space.clone(),
        method,
        space.clone(),
        key_bindings.request_selected.change_method.to_string().dark_gray(),
        space.clone(),
        help,
        space.clone(),
        key_bindings.main_menu.display_help.to_string().dark_gray(),
        space.clone(),
    ]);

    // Cancel Esc Enter Up Down Left Right Delete d
    *DISPLAYING_COOKIES.write().unwrap() = Line::from(vec![
        vec![
            cancel.clone(),
            space.clone(),
            key_bindings.generic.navigation.go_back.to_string().dark_gray(),
            space.clone(),
        ],
        unique_help(up.clone(), key_bindings.generic.navigation.move_cursor_up.to_string().dark_gray()),
        vec![space.clone()],
        unique_help(down.clone(), key_bindings.generic.navigation.move_cursor_down.to_string().dark_gray()),
        vec![space.clone()],
        unique_help(left.clone(), key_bindings.generic.navigation.move_cursor_left.to_string().dark_gray()),
        vec![space.clone()],
        unique_help(right.clone(), key_bindings.generic.navigation.move_cursor_right.to_string().dark_gray()),
        vec![
            space.clone(),
            delete,
            space.clone(),
            key_bindings.generic.list_and_table_actions.delete_element.to_string().dark_gray()
        ],
    ].concat());

    *CREATING_NEW_REQUEST.write().unwrap() = Line::from(vec![
        TEXT_INPUT_KEYS.read().unwrap().spans.clone(),
        vec![space.clone()],
        unique_help(up.clone(), key_bindings.generic.navigation.move_cursor_up.to_string().dark_gray()),
        vec![space.clone()],
        unique_help(down.clone(), key_bindings.generic.navigation.move_cursor_down.to_string().dark_gray()),
    ].concat());
}

lazy_static! {
    pub static ref KEY_BINDINGS: RwLock<KeyBindings> = RwLock::new(KeyBindings::default());
    pub static ref MAIN_MENU_KEYS: RwLock<Line<'static>> = RwLock::new(Line::default());
    pub static ref TEXT_INPUT_KEYS: RwLock<Line<'static>> = RwLock::new(Line::default());
    pub static ref TEXT_AREA_INPUT_KEYS: RwLock<Line<'static>> = RwLock::new(Line::default());
    pub static ref NAVIGATION_KEYS: RwLock<Line<'static>> = RwLock::new(Line::default());
    pub static ref VALIDATION_KEYS: RwLock<Line<'static>> = RwLock::new(Line::default());
    pub static ref REQUEST_SELECTED_KEYS: RwLock<Line<'static>> = RwLock::new(Line::default());
    pub static ref DISPLAYING_COOKIES: RwLock<Line<'static>> = RwLock::new(Line::default());
    pub static ref CREATING_NEW_REQUEST: RwLock<Line<'static>> = RwLock::new(Line::default());
}