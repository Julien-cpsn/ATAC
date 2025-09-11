use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use lazy_static::lazy_static;
use nestify::nest;
use parking_lot::RwLock;
use ratatui::style::Color;
use tracing::{trace, warn};
use serde::{Deserialize, Serialize};

use crate::app::app::App;
use crate::app::files::utils::expand_tilde;
use crate::panic_error;

nest! {
    #[derive(Serialize, Deserialize)]
    pub struct Theme {
        #[serde(alias = "UI")]
        pub ui:
            #[derive(Serialize, Deserialize)]
            pub struct ThemeUI {
                pub font_color: Color,
                pub app_background: Option<Color>,

                pub main_foreground_color: Color,
                pub secondary_foreground_color: Color,

                pub main_background_color: Color,
                pub secondary_background_color: Color
            },

        #[serde(alias = "Others")]
        pub others:
            #[derive(Serialize, Deserialize)]
            pub struct ThemeOthers {
                pub selection_highlight_color: Color,
                pub environment_variable_highlight_color: Color,
            },

        #[serde(alias = "HTTP")]
        pub http: #[derive(Serialize, Deserialize)]
            pub struct ThemeHttp {
            #[serde(alias = "Methods")]
            pub methods:
                #[derive(Serialize, Deserialize)]
                pub struct ThemeMethods {
                    #[serde(alias = "GET")]
                    pub get: Color,
                    #[serde(alias = "POST")]
                    pub post: Color,
                    #[serde(alias = "PUT")]
                    pub put: Color,
                    #[serde(alias = "PATCH")]
                    pub patch: Color,
                    #[serde(alias = "DELETE")]
                    pub delete: Color,
                    #[serde(alias = "HEAD")]
                    pub head: Color,
                    #[serde(alias = "OPTIONS")]
                    pub options: Color,
                    #[serde(alias = "TRACE")]
                    pub trace: Color,
                    #[serde(alias = "CONNECT")]
                    pub connect: Color
                },
        },

        #[serde(alias = "Websocket")]
        pub websocket:
            #[derive(Serialize, Deserialize)]
            pub struct ThemeWebsocket {
                #[serde(alias = "Connection Statuses")]
                pub connection_status:
                    #[derive(Serialize, Deserialize)]
                    pub struct ThemeConnectionStatuses {
                        #[serde(alias = "Connected")]
                        pub connected: Color,
                        #[serde(alias = "Disconnected")]
                        pub disconnected: Color,
                    },

                #[serde(alias = "Messages")]
                pub messages:
                    #[derive(Serialize, Deserialize)]
                    pub struct ThemeMessages {
                        #[serde(alias = "server_foreground_color")]
                        pub server_foreground_color: Color,
                        #[serde(alias = "server_background_color")]
                        pub server_background_color: Color,
                        #[serde(alias = "you_background_color")]
                        pub you_background_color: Color,
                        #[serde(alias = "details_color")]
                        pub details_color: Color,
                    }
            }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            ui: ThemeUI {
                font_color: Color::White,
                app_background: None,

                main_foreground_color: Color::White,
                secondary_foreground_color: Color::DarkGray,

                main_background_color: Color::DarkGray,
                secondary_background_color: Color::Rgb(50, 50, 50),
            },
            others: ThemeOthers {
                selection_highlight_color: Color::Yellow,
                environment_variable_highlight_color: Color::Cyan,
            },
            http: ThemeHttp {
                methods: ThemeMethods {
                    get: Color::Green,
                    post: Color::Yellow,
                    put: Color::LightBlue,
                    patch: Color::LightCyan,
                    delete: Color::LightRed,
                    options: Color::Magenta,
                    head: Color::Green,
                    trace: Color::Yellow,
                    connect: Color::LightBlue
                },
            },
            websocket: ThemeWebsocket {
                connection_status: ThemeConnectionStatuses {
                    connected: Color::Green,
                    disconnected: Color::Red
                },

                messages: ThemeMessages {
                    server_foreground_color: Color::Blue,
                    server_background_color: Color::DarkGray,
                    you_background_color: Color::Blue,
                    details_color: Color::DarkGray
                }
            }
        }
    }
}

lazy_static! {
    pub static ref THEME: RwLock<Theme> = RwLock::new(Theme::default());
}

impl App<'_> {
    pub fn parse_theme_file(&mut self) {
        let path = match env::var("ATAC_THEME") {
            // If the ATAC_THEME environment variable exists
            Ok(env_theme) => expand_tilde(PathBuf::from(env_theme)),
            Err(_) => {
                warn!("No theme file found, using default");
                return;
            }
        };

        trace!("Parsing theme file \"{}\"", path.display());

        let mut theme_file = match OpenOptions::new().read(true).open(path) {
            Ok(theme_file) => theme_file,
            Err(e) => panic_error(format!("Could not open theme file\n\t{e}"))
        };

        let mut file_content = String::new();
        theme_file.read_to_string(&mut file_content).expect("\tCould not read key bindings file");

        let theme: Theme = match toml::from_str(&file_content) {
            Ok(theme) => theme,
            Err(e) => panic_error(format!("Could not parse theme file\n\t{e}"))
        };

        *THEME.write() = theme;

        trace!("Theme file parsed!");
    }
}
