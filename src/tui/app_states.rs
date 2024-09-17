use crokey::{key, KeyCombination};
use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use ratatui::prelude::Span;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use strum::Display;

use crate::app::app::App;
use crate::app::files::key_bindings::{KEY_BINDINGS, TextAreaMode};
use crate::app::files::theme::THEME;
use crate::tui::app_states::AppState::*;
use crate::tui::event_key_bindings::EventKeyBinding;
use crate::tui::events::AppEvent;
use crate::tui::events::AppEvent::*;
use crate::tui::ui::param_tabs::param_tabs::RequestParamsTabs;
use crate::tui::ui::views::RequestView;

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

    #[strum(to_string = "Editing request body (Form)")]
    EditingRequestBodyTable,

    #[strum(to_string = "Editing request body (File)")]
    EditingRequestBodyFile,

    #[strum(to_string = "Editing request body (Text)")]
    EditingRequestBodyString,

    #[strum(to_string = "Editing pre-request script")]
    EditingPreRequestScript,

    #[strum(to_string = "Editing post-request script")]
    EditingPostRequestScript,

    #[strum(to_string = "Editing request settings")]
    EditingRequestSettings,
}

pub fn next_app_state(app_state: &AppState) -> AppState {
    match app_state {
        Normal => DisplayingCookies,
        DisplayingCookies => EditingCookies,
        EditingCookies => ChoosingElementToCreate,
        ChoosingElementToCreate => CreatingNewCollection,
        CreatingNewCollection => CreatingNewRequest,
        CreatingNewRequest => DeletingCollection,
        DeletingCollection => DeletingRequest,
        DeletingRequest => RenamingCollection,
        RenamingCollection => RenamingRequest,
        RenamingRequest => SelectedRequest,
        SelectedRequest => EditingRequestUrl,
        EditingRequestUrl => EditingRequestParam,
        EditingRequestParam => EditingRequestAuthUsername,
        EditingRequestAuthUsername => EditingRequestAuthPassword,
        EditingRequestAuthPassword => EditingRequestAuthBearerToken,
        EditingRequestAuthBearerToken => EditingRequestHeader,
        EditingRequestHeader => EditingRequestBodyTable,
        EditingRequestBodyTable => EditingRequestBodyFile,
        EditingRequestBodyFile => EditingRequestBodyString,
        EditingRequestBodyString => EditingPreRequestScript,
        EditingPreRequestScript => EditingPostRequestScript,
        EditingPostRequestScript => EditingRequestSettings,
        EditingRequestSettings => Normal,
    }
}

pub fn previous_app_state(app_state: &AppState) -> AppState {
    match app_state {
        Normal => EditingRequestSettings,
        DisplayingCookies => Normal,
        EditingCookies => DisplayingCookies,
        ChoosingElementToCreate => EditingCookies,
        CreatingNewCollection => ChoosingElementToCreate,
        CreatingNewRequest => CreatingNewCollection,
        DeletingCollection => CreatingNewRequest,
        DeletingRequest => DeletingCollection,
        RenamingCollection => DeletingRequest,
        RenamingRequest => RenamingCollection,
        SelectedRequest => RenamingRequest,
        EditingRequestUrl => SelectedRequest,
        EditingRequestParam => EditingRequestUrl,
        EditingRequestAuthUsername => EditingRequestParam,
        EditingRequestAuthPassword => EditingRequestAuthUsername,
        EditingRequestAuthBearerToken => EditingRequestAuthPassword,
        EditingRequestHeader => EditingRequestAuthBearerToken,
        EditingRequestBodyTable => EditingRequestHeader,
        EditingRequestBodyFile => EditingRequestBodyTable,
        EditingRequestBodyString => EditingRequestBodyFile,
        EditingPreRequestScript => EditingRequestBodyString,
        EditingPostRequestScript => EditingPreRequestScript,
        EditingRequestSettings => EditingPostRequestScript,
    }
}

impl AppState {
    pub fn get_available_events(&self, request_view: RequestView, request_param_tab: RequestParamsTabs) -> Vec<AppEvent> {
        let key_bindings = KEY_BINDINGS.read();

        match self {
            Normal => vec![
                ExitApp(EventKeyBinding::new(vec![key_bindings.main_menu.exit, key!(ctrl-c)], "Exit", Some("Exit"))),

                Documentation(EventKeyBinding::new(vec![key_bindings.generic.display_help], "Display help", Some("Help"))),

                MoveCollectionCursorUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], "Move up", Some("Up"))),
                MoveCollectionCursorDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], "Move down", Some("Down"))),

                SelectRequestOrExpandCollection(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], "Select", Some("Select"))),
                UnselectRequest(EventKeyBinding::new(vec![key_bindings.main_menu.unselect_request], "Unselect", None)),
                ExpandCollection(EventKeyBinding::new(vec![key_bindings.main_menu.expand_collection], "Expand", None)),

                CreateElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.create_element], "Create element", Some("Create"))),
                DeleteElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.delete_element], "Delete element", None)),
                RenameElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.rename_element], "Rename element", None)),

                MoveRequestUp(EventKeyBinding::new(vec![key_bindings.main_menu.move_request_up], "Move request up", None)),
                MoveRequestDown(EventKeyBinding::new(vec![key_bindings.main_menu.move_request_down], "Move request down", None)),

                NextEnvironment(EventKeyBinding::new(vec![key_bindings.main_menu.next_environment], "Next environment", None)),
                DisplayCookies(EventKeyBinding::new(vec![key_bindings.main_menu.display_cookies], "Display cookies", None)),
            ],
            DisplayingCookies => vec![
                GoBackToMainMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Quit", Some("Quit"))),

                CookiesMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], "Move up", Some("Up"))),
                CookiesMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], "Move down", Some("Down"))),
                CookiesMoveLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move left", Some("Left"))),
                CookiesMoveRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move right", Some("Right"))),

                DeleteCookie(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.delete_element], "Delete cookie", Some("Delete"))),
            ],
            EditingCookies => vec![
                Documentation(EventKeyBinding::new(vec![*EMPTY_KEY], "Not implemented yet", None))
            ],
            ChoosingElementToCreate => vec![
                GoBackToMainMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Quit", Some("Quit"))),

                ChooseElementToCreateMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move selection left", Some("Left"))),
                ChooseElementToCreateMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move selection right", Some("Right"))),

                SelectElementToCreate(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], "Select element to create", Some("Select"))),
            ],
            CreatingNewCollection => vec![
                GoBackToMainMenu(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.cancel], "Cancel", Some("Cancel"))),
                CreateNewCollection(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.confirm], "Confirm", Some("Confirm"))),

                CreatingCollectionDeleteCharBackward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_backward], "Delete char backward", Some("Delete"))),
                CreatingCollectionDeleteCharForward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_forward], "Delete char forward", Some("Backspace"))),
                CreatingCollectionMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_left], "Move cursor left", Some("Left"))),
                CreatingCollectionMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_right], "Move cursor right", Some("Right"))),
                CreatingCollectionCharInput(EventKeyBinding::new(vec![], "Char input", None)),
            ],
            CreatingNewRequest => vec![
                GoBackToMainMenu(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.cancel], "Cancel", Some("Cancel"))),
                CreateNewRequest(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.confirm], "Confirm", Some("Confirm"))),

                CreatingRequestDeleteCharBackward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_backward], "Delete char backward", Some("Delete"))),
                CreatingRequestDeleteCharForward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_forward], "Delete char forward", Some("Backspace"))),
                CreatingRequestMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_left], "Move cursor left", Some("Left"))),
                CreatingRequestMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_right], "Move cursor right", Some("Right"))),

                CreatingRequestSelectCollectionUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.alt_move_cursor_up], "Collection selection up", Some("Up"))),
                CreatingRequestSelectCollectionDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.alt_move_cursor_down], "Collection selection down", Some("Down"))),

                CreatingRequestCharInput(EventKeyBinding::new(vec![], "Char input", None)),
            ],
            DeletingCollection => vec![
                GoBackToMainMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Cancel", Some("Cancel"))),

                DeletingCollectionMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move selection left", Some("Left"))),
                DeletingCollectionMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move selection right", Some("Right"))),

                DeleteCollection(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], "Select choice", Some("Select"))),
            ],
            DeletingRequest => vec![
                GoBackToMainMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Cancel", Some("Cancel"))),

                DeletingRequestMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move selection left", Some("Left"))),
                DeletingRequestMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move selection right", Some("Right"))),

                DeleteRequest(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], "Select choice", Some("Select"))),

            ],
            RenamingCollection => vec![
                GoBackToMainMenu(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.cancel], "Cancel", Some("Cancel"))),
                RenameCollection(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.confirm], "Confirm", Some("Confirm"))),

                RenamingCollectionDeleteCharBackward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_backward], "Delete char backward", Some("Delete"))),
                RenamingCollectionDeleteCharForward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_forward], "Delete char forward", Some("Backspace"))),
                RenamingCollectionMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_left], "Move cursor left", Some("Left"))),
                RenamingCollectionMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_right], "Move cursor right", Some("Right"))),
                RenamingCollectionCharInput(EventKeyBinding::new(vec![], "Char input", None)),
            ],
            RenamingRequest => vec![
                GoBackToMainMenu(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.cancel], "Cancel", Some("Cancel"))),
                RenameRequest(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.confirm], "Confirm", Some("Confirm"))),

                RenamingRequestDeleteCharBackward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_backward], "Delete char backward", Some("Delete"))),
                RenamingRequestDeleteCharForward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_forward], "Delete char forward", Some("Backspace"))),
                RenamingRequestMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_left], "Move cursor left", Some("Left"))),
                RenamingRequestMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_right], "Move cursor right", Some("Right"))),
                RenamingRequestCharInput(EventKeyBinding::new(vec![], "Char input", None)),
            ],
            SelectedRequest => {
                // Depending on the current request view, some keys may need to be deactivated
                let (params_events_allowed, result_events_allowed) = match request_view {
                    RequestView::Normal => (true, true),
                    RequestView::OnlyResult => (false, true),
                    RequestView::OnlyParams => (true, false)
                };

                let mut base_events: Vec<AppEvent> = vec![
                    ExitApp(EventKeyBinding::new(vec![key!(ctrl-c)], "Exit app", None)),

                    GoBackToMainMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Quit to main menu", Some("Quit"))),
                    Documentation(EventKeyBinding::new(vec![key_bindings.generic.display_help], "Display help", Some("Help"))),

                    EditUrl(EventKeyBinding::new(vec![key_bindings.request_selected.change_url], "Edit URL", Some("URL"))),
                    EditMethod(EventKeyBinding::new(vec![key_bindings.request_selected.change_method], "Change method", Some("Method"))),

                    EditSettings(EventKeyBinding::new(vec![key_bindings.request_selected.request_settings], "Request settings", None)),

                    NextView(EventKeyBinding::new(vec![key_bindings.request_selected.next_view], "Next view", None)),

                    SendRequest(EventKeyBinding::new(vec![key_bindings.request_selected.send_request, key_bindings.request_selected.alt_send_request], "Send/cancel request", Some("Send/Cancel"))),

                    NextEnvironment(EventKeyBinding::new(vec![key_bindings.main_menu.next_environment], "Next environment", None)),
                    DisplayCookies(EventKeyBinding::new(vec![key_bindings.main_menu.display_cookies], "Display cookies", None)),
                ];

                let mut base_param_tabs_events: Vec<AppEvent> = vec![];
                let mut base_result_tabs_events: Vec<AppEvent> = vec![];

                // Param tabs
                if params_events_allowed {
                    base_param_tabs_events = vec![
                        NextParamTab(EventKeyBinding::new(vec![key_bindings.request_selected.param_next_tab], "Next param tab", Some("Next tab"))),

                        ModifyRequestAuthMethod(EventKeyBinding::new(vec![key_bindings.request_selected.param_tabs.change_auth_method], "Modify auth method", None)),
                        ModifyRequestBodyContentType(EventKeyBinding::new(vec![key_bindings.request_selected.param_tabs.change_body_content_type], "Modify body content-type", None)),
                    ];

                    let param_tabs_events = match request_param_tab {
                        RequestParamsTabs::QueryParams => vec![
                            EditRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], "Edit query param", None)),

                            RequestQueryParamsMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], "Move up", None)),
                            RequestQueryParamsMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], "Move down", None)),
                            RequestQueryParamsMoveLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move left", None)),
                            RequestQueryParamsMoveRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move right", None)),

                            CreateRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.create_element], "Create query param", None)),
                            DeleteRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.delete_element], "Delete query param", None)),
                            ToggleRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.toggle_element], "Toggle query param", None)),
                        ],
                        RequestParamsTabs::Auth => vec![
                            EditRequestAuth(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], "Edit auth element", None)),

                            RequestAuthMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], "Move up", None)),
                            RequestAuthMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], "Move down", None)),
                        ],
                        RequestParamsTabs::Headers => vec![
                            EditRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], "Edit header", None)),

                            RequestHeadersMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], "Move up", None)),
                            RequestHeadersMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], "Move down", None)),
                            RequestHeadersMoveLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move left", None)),
                            RequestHeadersMoveRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move right", None)),

                            CreateRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.create_element], "Create header", None)),
                            DeleteRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.delete_element], "Delete header", None)),
                            ToggleRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.toggle_element], "Toggle header", None)),
                        ],
                        RequestParamsTabs::Body => vec![
                            EditRequestBody(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], "Edit body", None)),

                            RequestBodyTableMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], "Move up", None)),
                            RequestBodyTableMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], "Move down", None)),
                            RequestBodyTableMoveLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move left", None)),
                            RequestBodyTableMoveRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move right", None)),

                            CreateRequestBodyTableElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.create_element], "Create form element", None)),
                            DeleteRequestBodyTableElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.delete_element], "Delete form element", None)),
                            ToggleRequestBodyTableElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.toggle_element], "Toggle form element", None)),
                        ],
                        RequestParamsTabs::Scripts => vec![
                            EditRequestScript(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], "Edit request script", Some("Edit"))),
                            RequestScriptMove(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], "Move up", Some("Up"))),
                            RequestScriptMove(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], "Move down", Some("Down"))),
                        ]
                    };

                    base_param_tabs_events.extend(param_tabs_events);
                }
                else {
                    base_events.push(
                        NextResultTab(EventKeyBinding::new(vec![key_bindings.request_selected.param_next_tab], "Next result tab", Some("Next tab"))),
                    );
                }

                if result_events_allowed {
                    base_result_tabs_events = vec![
                        ScrollResultUp(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_up], "Scroll result up", None)),
                        ScrollResultDown(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_down], "Scroll result down", None)),
                        ScrollResultLeft(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_left], "Scroll result left", None)),
                        ScrollResultRight(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_right], "Scroll result right", None)),
                    
                        CopyResponsePart(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.yank_response_part], "Yank response part", Some("Yank response"))),
                    ];

                    if params_events_allowed {
                        base_events.push(
                            NextResultTab(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.result_next_tab], "Next result tab", None)),
                        )
                    }
                }

                base_events.extend(base_param_tabs_events);
                base_events.extend(base_result_tabs_events);

                base_events
            },
            EditingRequestUrl => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.cancel], "Cancel", Some("Cancel"))),
                ModifyRequestUrl(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.confirm], "Confirm", Some("Confirm"))),

                EditingRequestUrlDeleteCharBackward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_backward], "Delete char backward", Some("Delete"))),
                EditingRequestUrlDeleteCharForward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_forward], "Delete char forward", Some("Backspace"))),
                EditingRequestUrlMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_left], "Move cursor left", Some("Left"))),
                EditingRequestUrlMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_right], "Move cursor right", Some("Right"))),
                EditingRequestUrlMoveCursorLineStart(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_start], "Move cursor line start", Some("Home"))),
                EditingRequestUrlMoveCursorLineEnd(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_end], "Move cursor line start", Some("Home"))),
                EditingRequestUrlCharInput(EventKeyBinding::new(vec![], "Char input", None)),
            ],
            EditingRequestParam => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.cancel], "Cancel", Some("Cancel"))),
                ModifyRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.confirm], "Confirm", Some("Confirm"))),

                EditingRequestQueryParamDeleteCharBackward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_backward], "Delete char backward", Some("Delete"))),
                EditingRequestQueryParamDeleteCharForward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_forward], "Delete char forward", Some("Backspace"))),
                EditingRequestQueryParamMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_left], "Move cursor left", Some("Left"))),
                EditingRequestQueryParamMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_right], "Move cursor right", Some("Right"))),
                EditingRequestQueryParamMoveCursorLineStart(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_start], "Move cursor line start", Some("Home"))),
                EditingRequestQueryParamMoveCursorLineEnd(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_end], "Move cursor line start", Some("Home"))),
                EditingRequestQueryParamCharInput(EventKeyBinding::new(vec![], "Char input", None)),
            ],
            EditingRequestAuthUsername => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.cancel], "Cancel", Some("Cancel"))),
                ModifyRequestAuthUsername(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.confirm], "Confirm", Some("Confirm"))),

                EditingRequestAuthUsernameDeleteCharBackward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_backward], "Delete char backward", Some("Delete"))),
                EditingRequestAuthUsernameDeleteCharForward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_forward], "Delete char forward", Some("Backspace"))),
                EditingRequestAuthUsernameMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_left], "Move cursor left", Some("Left"))),
                EditingRequestAuthUsernameMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_right], "Move cursor right", Some("Right"))),
                EditingRequestAuthUsernameMoveCursorLineStart(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_start], "Move cursor line start", Some("Home"))),
                EditingRequestAuthUsernameMoveCursorLineEnd(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_end], "Move cursor line start", Some("Home"))),
                EditingRequestAuthUsernameCharInput(EventKeyBinding::new(vec![], "Char input", None)),
            ],
            EditingRequestAuthPassword => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.cancel], "Cancel", Some("Cancel"))),
                ModifyRequestAuthPassword(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.confirm], "Confirm", Some("Confirm"))),

                EditingRequestAuthPasswordDeleteCharBackward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_backward], "Delete char backward", Some("Delete"))),
                EditingRequestAuthPasswordDeleteCharForward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_forward], "Delete char forward", Some("Backspace"))),
                EditingRequestAuthPasswordMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_left], "Move cursor left", Some("Left"))),
                EditingRequestAuthPasswordMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_right], "Move cursor right", Some("Right"))),
                EditingRequestAuthPasswordMoveCursorLineStart(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_start], "Move cursor line start", Some("Home"))),
                EditingRequestAuthPasswordMoveCursorLineEnd(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_end], "Move cursor line start", Some("Home"))),
                EditingRequestAuthPasswordCharInput(EventKeyBinding::new(vec![], "Char input", None)),
            ],
            EditingRequestAuthBearerToken => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.cancel], "Cancel", Some("Cancel"))),
                ModifyRequestAuthBearerToken(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.confirm], "Confirm", Some("Confirm"))),

                EditingRequestAuthBearerTokenDeleteCharBackward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_backward], "Delete char backward", Some("Delete"))),
                EditingRequestAuthBearerTokenDeleteCharForward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_forward], "Delete char forward", Some("Backspace"))),
                EditingRequestAuthBearerTokenMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_left], "Move cursor left", Some("Left"))),
                EditingRequestAuthBearerTokenMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_right], "Move cursor right", Some("Right"))),
                EditingRequestAuthBearerTokenMoveCursorLineStart(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_start], "Move cursor line start", Some("Home"))),
                EditingRequestAuthBearerTokenMoveCursorLineEnd(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_end], "Move cursor line start", Some("Home"))),
                EditingRequestAuthBearerTokenCharInput(EventKeyBinding::new(vec![], "Char input", None)),
            ],
            EditingRequestHeader => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.cancel], "Cancel", Some("Cancel"))),
                ModifyRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.confirm], "Confirm", Some("Confirm"))),

                EditingRequestHeaderDeleteCharBackward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_backward], "Delete char backward", Some("Delete"))),
                EditingRequestHeaderDeleteCharForward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_forward], "Delete char forward", Some("Backspace"))),
                EditingRequestHeaderMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_left], "Move cursor left", Some("Left"))),
                EditingRequestHeaderMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_right], "Move cursor right", Some("Right"))),
                EditingRequestHeaderMoveCursorLineStart(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_start], "Move cursor line start", Some("Home"))),
                EditingRequestHeaderMoveCursorLineEnd(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_end], "Move cursor line start", Some("Home"))),
                EditingRequestHeaderCharInput(EventKeyBinding::new(vec![], "Char input", None)),
            ],
            EditingRequestBodyTable => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.cancel], "Cancel", Some("Cancel"))),
                ModifyRequestBodyTable(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.confirm], "Confirm", Some("Confirm"))),

                EditingRequestBodyTableDeleteCharBackward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_backward], "Delete char backward", Some("Delete"))),
                EditingRequestBodyTableDeleteCharForward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_forward], "Delete char forward", Some("Backspace"))),
                EditingRequestBodyTableMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_left], "Move cursor left", Some("Left"))),
                EditingRequestBodyTableMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_right], "Move cursor right", Some("Right"))),
                EditingRequestBodyTableMoveCursorLineStart(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_start], "Move cursor line start", Some("Home"))),
                EditingRequestBodyTableMoveCursorLineEnd(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_end], "Move cursor line start", Some("Home"))),
                EditingRequestBodyTableCharInput(EventKeyBinding::new(vec![], "Char input", None)),
            ],
            EditingRequestBodyFile => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.cancel], "Cancel", Some("Cancel"))),
                ModifyRequestBodyFile(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.confirm], "Confirm", Some("Confirm"))),

                EditingRequestBodyFileDeleteCharBackward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_backward], "Delete char backward", Some("Delete"))),
                EditingRequestBodyFileDeleteCharForward(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.delete_forward], "Delete char forward", Some("Backspace"))),
                EditingRequestBodyFileMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_left], "Move cursor left", Some("Left"))),
                EditingRequestBodyFileMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_right], "Move cursor right", Some("Right"))),
                EditingRequestBodyFileMoveCursorLineStart(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_start], "Move cursor line start", Some("Home"))),
                EditingRequestBodyFileMoveCursorLineEnd(EventKeyBinding::new(vec![key_bindings.generic.text_inputs.text_input.move_cursor_line_end], "Move cursor line start", Some("Home"))),
                EditingRequestBodyFileCharInput(EventKeyBinding::new(vec![], "Char input", None)),
            ],
            EditingRequestBodyString => match key_bindings.generic.text_inputs.text_area_mode {
                TextAreaMode::VimEmulation => vec![
                    EditingRequestBodyStringVimInput(EventKeyBinding::new(vec![], "Vim input", None)),
                    Documentation(EventKeyBinding::new(vec![*EMPTY_KEY], "Vim key-bindings", Some("Vim-like key bindings"))),
                    Documentation(EventKeyBinding::new(vec![key!(q)], "Quit without saving", Some("Quit without saving"))),
                    Documentation(EventKeyBinding::new(vec![key!(Ctrl-s)], "Save and quit", Some("Save and quit"))),
                ],
                TextAreaMode::Custom(text_area_key_bindings) => vec![
                    GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Quit without saving", Some("Quit"))),
                    EditingRequestBodyStringSaveAndQuit(EventKeyBinding::new(vec![text_area_key_bindings.save_and_quit], "Save and quit", Some("Save"))),
                    EditingRequestBodyStringCopy(EventKeyBinding::new(vec![text_area_key_bindings.copy], "Copy", Some("Copy"))),
                    EditingRequestBodyStringPaste(EventKeyBinding::new(vec![text_area_key_bindings.paste], "Paste", Some("Paste"))),
                    EditingRequestBodyStringUndo(EventKeyBinding::new(vec![text_area_key_bindings.undo], "Undo", Some("Undo"))),
                    EditingRequestBodyStringRedo(EventKeyBinding::new(vec![text_area_key_bindings.redo], "Redo", Some("Redo"))),
                    EditingRequestBodyStringNewLine(EventKeyBinding::new(vec![text_area_key_bindings.new_line], "New line", None)),
                    EditingRequestBodyStringIndent(EventKeyBinding::new(vec![text_area_key_bindings.indent], "Indent", None)),
                    EditingRequestBodyStringDeleteCharBackward(EventKeyBinding::new(vec![text_area_key_bindings.delete_backward], "Delete char backward", None)),
                    EditingRequestBodyStringDeleteCharForward(EventKeyBinding::new(vec![text_area_key_bindings.delete_forward], "Delete char forward", None)),
                    EditingRequestBodyStringSkipWordLeft(EventKeyBinding::new(vec![text_area_key_bindings.skip_word_left], "Skip word left", None)),
                    EditingRequestBodyStringSkipWordRight(EventKeyBinding::new(vec![text_area_key_bindings.skip_word_right], "Skip word right", None)),
                    EditingRequestBodyStringMoveCursorUp(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_up], "Up", Some("Up"))),
                    EditingRequestBodyStringMoveCursorDown(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_down], "Down", Some("Down"))),
                    EditingRequestBodyStringMoveCursorLeft(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_left], "Left", Some("Left"))),
                    EditingRequestBodyStringMoveCursorRight(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_right], "Right", Some("Right"))),
                    EditingRequestBodyStringMoveCursorLineStart(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_line_start], "Line start", None)),
                    EditingRequestBodyStringMoveCursorLineEnd(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_line_end], "Line end", None)),
                    EditingRequestBodyStringCharInput(EventKeyBinding::new(vec![], "Char input", None)),
                ],
            },
            EditingPreRequestScript => match key_bindings.generic.text_inputs.text_area_mode {
                TextAreaMode::VimEmulation => vec![
                    EditingPreRequestScriptVimInput(EventKeyBinding::new(vec![], "Vim input", None)),
                    Documentation(EventKeyBinding::new(vec![*crate::tui::app_states::EMPTY_KEY], "Vim key-bindings", Some("Vim-like key bindings"))),
                    Documentation(EventKeyBinding::new(vec![key!(q)], "Quit without saving", Some("Quit without saving"))),
                    Documentation(EventKeyBinding::new(vec![key!(Ctrl-s)], "Save and quit", Some("Save and quit"))),
                ],
                TextAreaMode::Custom(text_area_key_bindings) => vec![
                    GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Quit without saving", Some("Quit"))),
                    EditingPreRequestScriptSaveAndQuit(EventKeyBinding::new(vec![text_area_key_bindings.save_and_quit], "Save and quit", Some("Save"))),
                    EditingPreRequestScriptCopy(EventKeyBinding::new(vec![text_area_key_bindings.copy], "Copy", Some("Copy"))),
                    EditingPreRequestScriptPaste(EventKeyBinding::new(vec![text_area_key_bindings.paste], "Paste", Some("Paste"))),
                    EditingPreRequestScriptUndo(EventKeyBinding::new(vec![text_area_key_bindings.undo], "Undo", Some("Undo"))),
                    EditingPreRequestScriptRedo(EventKeyBinding::new(vec![text_area_key_bindings.redo], "Redo", Some("Redo"))),
                    EditingPreRequestScriptNewLine(EventKeyBinding::new(vec![text_area_key_bindings.new_line], "New line", None)),
                    EditingPreRequestScriptIndent(EventKeyBinding::new(vec![text_area_key_bindings.indent], "Indent", None)),
                    EditingPreRequestScriptDeleteCharBackward(EventKeyBinding::new(vec![text_area_key_bindings.delete_backward], "Delete char backward", None)),
                    EditingPreRequestScriptDeleteCharForward(EventKeyBinding::new(vec![text_area_key_bindings.delete_forward], "Delete char forward", None)),
                    EditingPreRequestScriptSkipWordLeft(EventKeyBinding::new(vec![text_area_key_bindings.skip_word_left], "Skip word left", None)),
                    EditingPreRequestScriptSkipWordRight(EventKeyBinding::new(vec![text_area_key_bindings.skip_word_right], "Skip word right", None)),
                    EditingPreRequestScriptMoveCursorUp(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_up], "Up", Some("Up"))),
                    EditingPreRequestScriptMoveCursorDown(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_down], "Down", Some("Down"))),
                    EditingPreRequestScriptMoveCursorLeft(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_left], "Left", Some("Left"))),
                    EditingPreRequestScriptMoveCursorRight(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_right], "Right", Some("Right"))),
                    EditingPreRequestScriptMoveCursorLineStart(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_line_start], "Line start", None)),
                    EditingPreRequestScriptMoveCursorLineEnd(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_line_end], "Line end", None)),
                    EditingPreRequestScriptCharInput(EventKeyBinding::new(vec![], "Char input", None)),
                ]
            },
            EditingPostRequestScript => match key_bindings.generic.text_inputs.text_area_mode {
                TextAreaMode::VimEmulation => vec![
                    EditingPostRequestScriptVimInput(EventKeyBinding::new(vec![], "Vim input", None)),
                    Documentation(EventKeyBinding::new(vec![*crate::tui::app_states::EMPTY_KEY], "Vim key-bindings", Some("Vim-like key bindings"))),
                    Documentation(EventKeyBinding::new(vec![key!(q)], "Quit without saving", Some("Quit without saving"))),
                    Documentation(EventKeyBinding::new(vec![key!(Ctrl-s)], "Save and quit", Some("Save and quit"))),
                ],
                TextAreaMode::Custom(text_area_key_bindings) => vec![
                    GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Quit without saving", Some("Quit"))),
                    EditingPostRequestScriptSaveAndQuit(EventKeyBinding::new(vec![text_area_key_bindings.save_and_quit], "Save and quit", Some("Save"))),
                    EditingPostRequestScriptCopy(EventKeyBinding::new(vec![text_area_key_bindings.copy], "Copy", Some("Copy"))),
                    EditingPostRequestScriptPaste(EventKeyBinding::new(vec![text_area_key_bindings.paste], "Paste", Some("Paste"))),
                    EditingPostRequestScriptUndo(EventKeyBinding::new(vec![text_area_key_bindings.undo], "Undo", Some("Undo"))),
                    EditingPostRequestScriptRedo(EventKeyBinding::new(vec![text_area_key_bindings.redo], "Redo", Some("Redo"))),
                    EditingPostRequestScriptNewLine(EventKeyBinding::new(vec![text_area_key_bindings.new_line], "New line", None)),
                    EditingPostRequestScriptIndent(EventKeyBinding::new(vec![text_area_key_bindings.indent], "Indent", None)),
                    EditingPostRequestScriptDeleteCharBackward(EventKeyBinding::new(vec![text_area_key_bindings.delete_backward], "Delete char backward", None)),
                    EditingPostRequestScriptDeleteCharForward(EventKeyBinding::new(vec![text_area_key_bindings.delete_forward], "Delete char forward", None)),
                    EditingPostRequestScriptSkipWordLeft(EventKeyBinding::new(vec![text_area_key_bindings.skip_word_left], "Skip word left", None)),
                    EditingPostRequestScriptSkipWordRight(EventKeyBinding::new(vec![text_area_key_bindings.skip_word_right], "Skip word right", None)),
                    EditingPostRequestScriptMoveCursorUp(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_up], "Up", Some("Up"))),
                    EditingPostRequestScriptMoveCursorDown(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_down], "Down", Some("Down"))),
                    EditingPostRequestScriptMoveCursorLeft(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_left], "Left", Some("Left"))),
                    EditingPostRequestScriptMoveCursorRight(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_right], "Right", Some("Right"))),
                    EditingPostRequestScriptMoveCursorLineStart(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_line_start], "Line start", None)),
                    EditingPostRequestScriptMoveCursorLineEnd(EventKeyBinding::new(vec![text_area_key_bindings.move_cursor_line_end], "Line end", None)),
                    EditingPostRequestScriptCharInput(EventKeyBinding::new(vec![], "Char input", None)),
                ]
            }
            EditingRequestSettings => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Cancel", Some("Cancel"))),

                RequestSettingsMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], "Move up", Some("Up"))),
                RequestSettingsMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], "Move down", Some("Down"))),
                RequestSettingsToggleSetting(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left, key_bindings.generic.navigation.move_cursor_right], "Toggle setting", Some("Toggle"))),

                ModifyRequestSettings(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], "Confirm", Some("Confirm"))),
            ]
        }
    }
}

pub fn event_available_keys_to_spans(events: &Vec<AppEvent>, fg_color: Color, bg_color: Color, short_only: bool) -> Vec<Vec<Span>> {
    let mut spans: Vec<Vec<Span>> = vec![];

    for event in events.iter() {
        let is_documentation = match event {
            Documentation(_) => true,
            _ => false
        };

        let event_key_bindings = event.get_event_key_bindings();

        if let Some(key_spans) = event_key_bindings.to_spans(fg_color, bg_color, short_only, is_documentation) {
            spans.push(key_spans);
        }
    }

    spans.last_mut().unwrap().pop();

    return spans;
}

lazy_static! {
    pub static ref AVAILABLE_EVENTS: RwLock<Vec<AppEvent>> = RwLock::new(vec![]);
    pub static ref EMPTY_KEY: KeyCombination = KeyCombination::new(KeyCode::Null, KeyModifiers::NONE);
}

impl App<'_> {
    pub fn update_current_available_events(&mut self) {
        *AVAILABLE_EVENTS.write() = self.state.get_available_events(self.request_view, self.request_param_tab);
    }

    pub fn get_state_line(&self) -> Line {
        match self.state {
            Normal |
            ChoosingElementToCreate |
            CreatingNewCollection | CreatingNewRequest |
            DisplayingCookies | EditingCookies => Line::from(self.state.to_string()).fg(THEME.read().ui.font_color).bg(THEME.read().ui.main_background_color),

            DeletingCollection | RenamingCollection => {
                let collection_index = self.collections_tree.state.selected()[0];
                let collection_name = &self.collections[collection_index].name;

                Line::from(vec![
                    Span::raw("Collection > ").fg(THEME.read().ui.secondary_foreground_color),
                    Span::raw(format!("{} > ", collection_name)).fg(THEME.read().ui.secondary_foreground_color),
                    Span::raw(self.state.to_string()).fg(THEME.read().ui.font_color).bg(THEME.read().ui.main_background_color)
                ])
            },

            DeletingRequest | RenamingRequest => {
                let selected_request_index = &self.collections_tree.state.selected();
                let selected_request = &self.collections[selected_request_index[0]].requests[selected_request_index[1]].read();

                Line::from(vec![
                    Span::raw("Request > ").fg(THEME.read().ui.secondary_foreground_color),
                    Span::raw(format!("{} > ", selected_request.name)).fg(THEME.read().ui.secondary_foreground_color),
                    Span::raw(self.state.to_string()).fg(THEME.read().ui.font_color).bg(THEME.read().ui.main_background_color)
                ])
            },

            SelectedRequest |
            EditingRequestUrl |
            EditingRequestParam |
            EditingRequestAuthUsername | EditingRequestAuthPassword | EditingRequestAuthBearerToken  |
            EditingRequestHeader |
            EditingRequestBodyTable | EditingRequestBodyFile | EditingRequestBodyString |
            EditingPreRequestScript | EditingPostRequestScript |
            EditingRequestSettings
            => {
                let local_selected_request = self.get_selected_request_as_local();
                let selected_request = local_selected_request.read();

                if self.state == SelectedRequest {
                    Line::from(vec![
                        Span::raw("Request > ").fg(THEME.read().ui.secondary_foreground_color),
                        Span::raw(selected_request.name.clone()).fg(THEME.read().ui.font_color).bg(THEME.read().ui.main_background_color)
                    ])
                }
                else {
                    Line::from(vec![
                        Span::raw("Request > ").fg(THEME.read().ui.secondary_foreground_color),
                        Span::raw(format!("{} > ", selected_request.name)).fg(THEME.read().ui.secondary_foreground_color),
                        Span::raw(self.state.to_string()).fg(THEME.read().ui.font_color).bg(THEME.read().ui.main_background_color)
                    ])
                }
            }
        }
    }
}