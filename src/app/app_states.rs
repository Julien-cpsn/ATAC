use ratatui::prelude::Span;
use ratatui::style::Stylize;
use ratatui::text::Line;
use strum::Display;

use crate::app::app::App;
use crate::app::app_states::AppState::*;
use crate::app::files::key_bindings::{CREATING_NEW_REQUEST, DISPLAYING_COOKIES, MAIN_MENU_KEYS, NAVIGATION_KEYS, REQUEST_SELECTED_KEYS, TEXT_AREA_INPUT_KEYS, TEXT_INPUT_KEYS, VALIDATION_KEYS};

#[derive(Copy, Clone, PartialEq, Default, Display)]
pub enum AppState {
    #[default]
    #[strum(to_string = "Main menu")]
    Normal,

    /* Cookies */
    
    #[strum(to_string = "Displaying cookies")]
    DisplayingCookies,

    #[strum(to_string = "Editing cookies")]
    #[allow(dead_code)]
    EditingCookies,
    
    /* Collections */

    #[strum(to_string = "Choosing an element to create")]
    ChoosingElementToCreate,

    #[strum(to_string = "Creating new collection")]
    CreatingNewCollection,

    #[strum(to_string = "Creating new request")]
    CreatingNewRequest,

    #[strum(to_string = "Deleting collection")]
    DeletingCollection,

    #[strum(to_string = "Deleting request")]
    DeletingRequest,

    #[strum(to_string = "Renaming collection")]
    RenamingCollection,

    #[strum(to_string = "Renaming request")]
    RenamingRequest,

    /* Request */

    #[strum(to_string = "Request menu")]
    SelectedRequest,
    
    #[strum(to_string = "Editing request URL")]
    EditingRequestUrl,

    #[strum(to_string = "Editing request param")]
    EditingRequestParam,

    #[strum(to_string = "Editing request auth username")]
    EditingRequestAuthUsername,

    #[strum(to_string = "Editing request auth password")]
    EditingRequestAuthPassword,

    #[strum(to_string = "Editing request auth bearer token")]
    EditingRequestAuthBearerToken,

    #[strum(to_string = "Editing request header")]
    EditingRequestHeader,
    
    #[strum(to_string = "Editing request body")]
    EditingRequestBodyTable,
    
    #[strum(to_string = "Editing request body")]
    EditingRequestBodyFile,
    
    #[strum(to_string = "Editing request body")]
    EditingRequestBodyString,

    #[strum(to_string = "Editing request settings")]
    EditingRequestSettings
}

impl App<'_> {
    pub fn get_state_line(&self) -> Line {
        match self.state {
            Normal => Line::from(self.state.to_string().white().on_dark_gray()),
            ChoosingElementToCreate => Line::from(self.state.to_string().white().on_dark_gray()),
            CreatingNewCollection | CreatingNewRequest => Line::from(self.state.to_string().white().on_dark_gray()),
            DisplayingCookies | EditingCookies => Line::from(self.state.to_string().white().on_dark_gray()),
            DeletingCollection | RenamingCollection => {
                let collection_index = self.collections_tree.state.selected()[0];
                let collection_name = &self.collections[collection_index].name;

                Line::from(vec![
                    Span::raw("Collection > ").dark_gray(),
                    Span::raw(format!("{} > ", collection_name)).dark_gray(),
                    Span::raw(self.state.to_string()).white().on_dark_gray()
                ])
            },
            DeletingRequest | RenamingRequest => {
                let selected_request_index = &self.collections_tree.state.selected();
                let selected_request = &self.collections[selected_request_index[0]].requests[selected_request_index[1]].read().unwrap();

                Line::from(vec![
                    Span::raw("Request > ").dark_gray(),
                    Span::raw(format!("{} > ", selected_request.name)).dark_gray(),
                    Span::raw(self.state.to_string()).white().on_dark_gray()
                ])
            },
            SelectedRequest |
            EditingRequestUrl |
            EditingRequestParam |
            EditingRequestAuthUsername | EditingRequestAuthPassword | EditingRequestAuthBearerToken  |
            EditingRequestHeader |
            EditingRequestBodyTable | EditingRequestBodyFile | EditingRequestBodyString |
            EditingRequestSettings 
            => {
                let local_selected_request = self.get_selected_request_as_local();
                let selected_request = local_selected_request.read().unwrap();

                if self.state == SelectedRequest {
                    Line::from(vec![
                        Span::raw("Request > ").dark_gray(),
                        Span::raw(selected_request.name.clone()).white().on_dark_gray()
                    ])
                }
                else {
                    Line::from(vec![
                        Span::raw("Request > ").dark_gray(),
                        Span::raw(format!("{} > ", selected_request.name)).dark_gray(),
                        Span::raw(self.state.to_string()).white().on_dark_gray()
                    ])
                }
            }
        }
    }

    pub fn get_available_keys(&self) -> Line {
        match self.state {
            Normal => MAIN_MENU_KEYS.read().unwrap().clone(),

            /* Cookies */

            ChoosingElementToCreate => VALIDATION_KEYS.read().unwrap().clone(),

            DisplayingCookies => DISPLAYING_COOKIES.read().unwrap().clone(),

            EditingCookies => TEXT_INPUT_KEYS.read().unwrap().clone(),

            /* Collections */
            
            CreatingNewCollection => TEXT_INPUT_KEYS.read().unwrap().clone(),

            CreatingNewRequest => CREATING_NEW_REQUEST.read().unwrap().clone(),

            DeletingCollection => VALIDATION_KEYS.read().unwrap().clone(),

            DeletingRequest => VALIDATION_KEYS.read().unwrap().clone(),

            RenamingCollection => TEXT_INPUT_KEYS.read().unwrap().clone(),

            RenamingRequest => TEXT_INPUT_KEYS.read().unwrap().clone(),

            /* Request */

            SelectedRequest => REQUEST_SELECTED_KEYS.read().unwrap().clone(),
            
            EditingRequestUrl => TEXT_INPUT_KEYS.read().unwrap().clone(),

            EditingRequestParam => TEXT_INPUT_KEYS.read().unwrap().clone(),

            EditingRequestAuthUsername | EditingRequestAuthPassword | EditingRequestAuthBearerToken => TEXT_INPUT_KEYS.read().unwrap().clone(),

            EditingRequestHeader => TEXT_INPUT_KEYS.read().unwrap().clone(),

            EditingRequestBodyTable => TEXT_INPUT_KEYS.read().unwrap().clone(),
            
            EditingRequestBodyFile => TEXT_INPUT_KEYS.read().unwrap().clone(),

            EditingRequestBodyString => TEXT_AREA_INPUT_KEYS.read().unwrap().clone(),

            EditingRequestSettings => NAVIGATION_KEYS.read().unwrap().clone()
        }
    }
}