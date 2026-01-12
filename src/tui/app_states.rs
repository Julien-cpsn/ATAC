use crokey::{key, KeyCombination};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use ratatui::prelude::Span;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use strum::Display;

use crate::app::app::App;
use crate::app::files::key_bindings::{CustomTextArea, TextAreaMode, KEY_BINDINGS};
use crate::app::files::theme::THEME;
use crate::models::protocol::protocol::Protocol;
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

    /* Env */

    #[strum(to_string = "Displaying environment editor")]
    DisplayingEnvEditor,

    #[strum(to_string = "Editing env variable")]
    EditingEnvVariable,

    /* Cookies */

    #[strum(to_string = "Displaying cookies")]
    DisplayingCookies,

    #[strum(to_string = "Editing cookies")]
    #[allow(dead_code)]
    EditingCookies,

    /* Logs */

    #[strum(to_string = "Displaying logs")]
    DisplayingLogs,

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
    EditingRequestAuthBasicUsername,

    #[strum(to_string = "Editing request auth password")]
    EditingRequestAuthBasicPassword,

    #[strum(to_string = "Editing request auth bearer token")]
    EditingRequestAuthBearerToken,

    #[strum(to_string = "Editing request JWT secret")]
    EditingRequestAuthJwtSecret,

    #[strum(to_string = "Editing request JWT payload")]
    EditingRequestAuthJwtPayload,

    #[strum(to_string = "Editing request digest username")]
    EditingRequestAuthDigestUsername,

    #[strum(to_string = "Editing request digest password")]
    EditingRequestAuthDigestPassword,

    #[strum(to_string = "Editing request digest domains")]
    EditingRequestAuthDigestDomains,

    #[strum(to_string = "Editing request digest realm")]
    EditingRequestAuthDigestRealm,

    #[strum(to_string = "Editing request digest nonce")]
    EditingRequestAuthDigestNonce,

    #[strum(to_string = "Editing request digest opaque")]
    EditingRequestAuthDigestOpaque,
    
    #[strum(to_string = "Editing request header")]
    EditingRequestHeader,

    #[strum(to_string = "Editing request body (Form)")]
    EditingRequestBodyTable,

    #[strum(to_string = "Editing request body (File)")]
    EditingRequestBodyFile,

    #[strum(to_string = "Editing request body (Text)")]
    EditingRequestBodyString,

    #[strum(to_string = "Editing request message")]
    EditingRequestMessage,

    #[strum(to_string = "Editing pre-request script")]
    EditingPreRequestScript,

    #[strum(to_string = "Editing post-request script")]
    EditingPostRequestScript,

    #[strum(to_string = "Editing request settings")]
    EditingRequestSettings,

    #[strum(to_string = "Choosing request export format")]
    ChoosingRequestExportFormat,

    #[strum(to_string = "Displaying request export")]
    DisplayingRequestExport
}

pub fn next_app_state(app_state: &AppState) -> AppState {
    match app_state {
        Normal => DisplayingEnvEditor,
        DisplayingEnvEditor => EditingEnvVariable,
        EditingEnvVariable => DisplayingCookies,
        DisplayingCookies => EditingCookies,
        EditingCookies => DisplayingLogs,
        DisplayingLogs => ChoosingElementToCreate,
        ChoosingElementToCreate => CreatingNewCollection,
        CreatingNewCollection => CreatingNewRequest,
        CreatingNewRequest => DeletingCollection,
        DeletingCollection => DeletingRequest,
        DeletingRequest => RenamingCollection,
        RenamingCollection => RenamingRequest,
        RenamingRequest => SelectedRequest,
        SelectedRequest => EditingRequestUrl,
        EditingRequestUrl => EditingRequestParam,
        EditingRequestParam => EditingRequestAuthBasicUsername,
        EditingRequestAuthBasicUsername => EditingRequestAuthBasicPassword,
        EditingRequestAuthBasicPassword => EditingRequestAuthBearerToken,
        EditingRequestAuthBearerToken => EditingRequestAuthJwtSecret,
        EditingRequestAuthJwtSecret => EditingRequestAuthJwtPayload,
        EditingRequestAuthJwtPayload => EditingRequestAuthDigestUsername,
        EditingRequestAuthDigestUsername => EditingRequestAuthDigestPassword,
        EditingRequestAuthDigestPassword => EditingRequestAuthDigestDomains,
        EditingRequestAuthDigestDomains => EditingRequestAuthDigestRealm,
        EditingRequestAuthDigestRealm => EditingRequestAuthDigestNonce,
        EditingRequestAuthDigestNonce => EditingRequestAuthDigestOpaque,
        EditingRequestAuthDigestOpaque => EditingRequestHeader,
        EditingRequestHeader => EditingRequestBodyTable,
        EditingRequestBodyTable => EditingRequestBodyFile,
        EditingRequestBodyFile => EditingRequestBodyString,
        EditingRequestBodyString => EditingRequestMessage,
        EditingRequestMessage => EditingPreRequestScript,
        EditingPreRequestScript => EditingPostRequestScript,
        EditingPostRequestScript => EditingRequestSettings,
        EditingRequestSettings => ChoosingRequestExportFormat,
        ChoosingRequestExportFormat => DisplayingRequestExport,
        DisplayingRequestExport => Normal
    }
}

pub fn previous_app_state(app_state: &AppState) -> AppState {
    match app_state {
        Normal => EditingRequestSettings,
        DisplayingEnvEditor => Normal,
        EditingEnvVariable => DisplayingEnvEditor,
        DisplayingCookies => EditingEnvVariable,
        EditingCookies => DisplayingCookies,
        DisplayingLogs => EditingCookies,
        ChoosingElementToCreate => DisplayingLogs,
        CreatingNewCollection => ChoosingElementToCreate,
        CreatingNewRequest => CreatingNewCollection,
        DeletingCollection => CreatingNewRequest,
        DeletingRequest => DeletingCollection,
        RenamingCollection => DeletingRequest,
        RenamingRequest => RenamingCollection,
        SelectedRequest => RenamingRequest,
        EditingRequestUrl => SelectedRequest,
        EditingRequestParam => EditingRequestUrl,
        EditingRequestAuthBasicUsername => EditingRequestParam,
        EditingRequestAuthBasicPassword => EditingRequestAuthBasicUsername,
        EditingRequestAuthBearerToken => EditingRequestAuthBasicPassword,
        EditingRequestAuthJwtSecret => EditingRequestAuthBearerToken,
        EditingRequestAuthJwtPayload => EditingRequestAuthJwtSecret,
        EditingRequestAuthDigestUsername => EditingRequestAuthJwtPayload,
        EditingRequestAuthDigestPassword => EditingRequestAuthDigestUsername,
        EditingRequestAuthDigestDomains => EditingRequestAuthDigestPassword,
        EditingRequestAuthDigestRealm => EditingRequestAuthDigestDomains,
        EditingRequestAuthDigestNonce => EditingRequestAuthDigestRealm,
        EditingRequestAuthDigestOpaque => EditingRequestAuthDigestNonce,
        EditingRequestHeader => EditingRequestAuthDigestOpaque,
        EditingRequestBodyTable => EditingRequestHeader,
        EditingRequestBodyFile => EditingRequestBodyTable,
        EditingRequestBodyString => EditingRequestBodyFile,
        EditingRequestMessage => EditingRequestBodyString,
        EditingPreRequestScript => EditingRequestMessage,
        EditingPostRequestScript => EditingPreRequestScript,
        EditingRequestSettings => EditingPostRequestScript,
        ChoosingRequestExportFormat => EditingRequestSettings,
        DisplayingRequestExport => ChoosingRequestExportFormat
    }
}

impl AppState {
    pub fn get_available_events(&self, request_view: RequestView, request_param_tab: RequestParamsTabs, protocol: Option<Protocol>, is_there_any_env: bool) -> Vec<AppEvent> {
        let key_bindings = KEY_BINDINGS.read();

        match self {
            Normal => {
                let mut base_events = vec![
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
                    DuplicateElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.duplicate_element], "Duplicate element", None)),

                    MoveElementUp(EventKeyBinding::new(vec![key_bindings.main_menu.move_request_up], "Move request up", None)),
                    MoveElementDown(EventKeyBinding::new(vec![key_bindings.main_menu.move_request_down], "Move request down", None)),
                ];

                if is_there_any_env {
                    let env_events = vec![
                        NextEnvironment(EventKeyBinding::new(vec![key_bindings.main_menu.next_environment], "Next environment", None)),
                        DisplayEnvEditor(EventKeyBinding::new(vec![key_bindings.main_menu.display_env_editor], "Environment editor", None)),
                    ];
                    
                    base_events.extend(env_events);
                }

                let other_events = vec![
                    DisplayCookies(EventKeyBinding::new(vec![key_bindings.main_menu.display_cookies], "Display cookies", None)),
                    DisplayLogs(EventKeyBinding::new(vec![key_bindings.main_menu.display_logs], "Display logs", None)),
                ];
                
                base_events.extend(other_events);
                
                base_events
            },
            DisplayingEnvEditor => vec![
                GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Quit", Some("Quit"))),
                EditEnvVariable(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], "Edit env variable", None)),

                EnvVariablesMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], "Move up", Some("Up"))),
                EnvVariablesMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], "Move down", Some("Down"))),
                EnvVariablesMoveLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move left", Some("Left"))),
                EnvVariablesMoveRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move right", Some("Right"))),

                CreateEnvVariable(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.create_element], "Create env variable", Some("Create variable"))),
                DeleteEnvVariable(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.delete_element], "Delete env variable", Some("Delete variable"))),
            ],
            EditingEnvVariable => [
                vec![
                    ModifyEnvVariable(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelModifyEnvVariable(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventModifyEnvVariable(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, true)
            ].concat(),
            DisplayingCookies => vec![
                GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Quit", Some("Quit"))),

                CookiesMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], "Move up", Some("Up"))),
                CookiesMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], "Move down", Some("Down"))),
                CookiesMoveLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move left", Some("Left"))),
                CookiesMoveRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move right", Some("Right"))),

                DeleteCookie(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.delete_element], "Delete cookie", Some("Delete"))),
            ],
            EditingCookies => vec![
                Documentation(EventKeyBinding::new(vec![*EMPTY_KEY], "Not implemented yet", None))
            ],
            DisplayingLogs => vec![
                GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Quit", Some("Quit"))),
                ScrollLogsUp(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_up], "Scroll logs up", Some("Up"))),
                ScrollLogsDown(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_down], "Scroll logs down", Some("Down"))),
                ScrollLogsLeft(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_left], "Scroll logs left", Some("Left"))),
                ScrollLogsRight(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_right], "Scroll logs right", Some("Right"))),
            ],
            ChoosingElementToCreate => vec![
                GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Quit", Some("Quit"))),

                ChooseElementToCreateMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move selection left", Some("Left"))),
                ChooseElementToCreateMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move selection right", Some("Right"))),

                SelectElementToCreate(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], "Select element to create", Some("Select"))),
            ],
            CreatingNewCollection => [
                vec![
                    CreateNewCollection(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelCreateNewCollection(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventCreateNewCollection(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            CreatingNewRequest => [
                vec![
                    CreateNewRequest(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelCreateNewRequest(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),

                    CreatingRequestSelectInputUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.alt_move_cursor_up], "Input selection up", Some("Up"))),
                    CreatingRequestSelectInputDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.alt_move_cursor_down], "Input selection down", Some("Down"))),
                    CreatingRequestInputLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Previous", Some("Left"))),
                    CreatingRequestInputRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Next", Some("Right"))),

                    KeyEventCreateNewRequest(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            DeletingCollection => vec![
                GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Cancel", Some("Cancel"))),

                DeletingCollectionMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move selection left", Some("Left"))),
                DeletingCollectionMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move selection right", Some("Right"))),

                DeleteCollection(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], "Select choice", Some("Select"))),
            ],
            DeletingRequest => vec![
                GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Cancel", Some("Cancel"))),

                DeletingRequestMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move selection left", Some("Left"))),
                DeletingRequestMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move selection right", Some("Right"))),

                DeleteRequest(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], "Select choice", Some("Select"))),

            ],
            RenamingCollection => [
                vec![
                    RenameCollection(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelRenameCollection(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventRenameCollection(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            RenamingRequest => [
                vec![
                    RenameRequest(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelRenameRequest(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventRenameRequest(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            SelectedRequest => {
                // Depending on the current request view, some keys may need to be deactivated
                let (params_events_allowed, result_events_allowed) = match request_view {
                    RequestView::Normal => (true, true),
                    RequestView::OnlyResult => (false, true),
                    RequestView::OnlyParams => (true, false)
                };

                let mut base_events: Vec<AppEvent> = vec![
                    ExitApp(EventKeyBinding::new(vec![key!(ctrl-c)], "Exit app", None)),

                    GoBackToLastState(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Quit to main menu", Some("Quit"))),
                    Documentation(EventKeyBinding::new(vec![key_bindings.generic.display_help], "Display help", Some("Help"))),

                    EditUrl(EventKeyBinding::new(vec![key_bindings.request_selected.change_url], "Edit URL", Some("URL"))),
                    EditMethod(EventKeyBinding::new(vec![key_bindings.request_selected.change_method], "Change method", Some("Method"))),

                    EditSettings(EventKeyBinding::new(vec![key_bindings.request_selected.request_settings], "Request settings", None)),

                    NextView(EventKeyBinding::new(vec![key_bindings.request_selected.next_view], "Next view", None)),

                    SendRequest(EventKeyBinding::new(vec![key_bindings.request_selected.send_request, key_bindings.request_selected.alt_send_request], "Send/cancel request", Some("Send/Cancel"))),
                ];
                
                if is_there_any_env {
                    let env_events = vec![
                        NextEnvironment(EventKeyBinding::new(vec![key_bindings.main_menu.next_environment], "Next environment", None)),
                        DisplayEnvEditor(EventKeyBinding::new(vec![key_bindings.main_menu.display_env_editor], "Environment editor", None)),
                    ];
                    
                    base_events.extend(env_events);
                }
                
                let other_events = vec![
                    DisplayCookies(EventKeyBinding::new(vec![key_bindings.main_menu.display_cookies], "Display cookies", None)),
                    DisplayLogs(EventKeyBinding::new(vec![key_bindings.main_menu.display_logs], "Display logs", None)),
                    ExportRequest(EventKeyBinding::new(vec![key_bindings.request_selected.export_request], "Export request", None)),
                ];
                
                base_events.extend(other_events);

                let mut base_param_tabs_events: Vec<AppEvent> = vec![];
                let mut base_result_tabs_events: Vec<AppEvent> = vec![];

                // Param tabs
                if params_events_allowed {
                    base_param_tabs_events = vec![
                        NextParamTab(EventKeyBinding::new(vec![key_bindings.request_selected.param_next_tab], "Next param tab", Some("Next tab"))),

                        ModifyRequestAuthMethod(EventKeyBinding::new(vec![key_bindings.request_selected.param_tabs.change_auth_method], "Modify auth method", None)),
                    ];

                    if let Some(protocol) = protocol {
                        let protocol_specific = match protocol {
                            Protocol::HttpRequest(_) => vec![
                                ModifyRequestBodyContentType(EventKeyBinding::new(vec![key_bindings.request_selected.param_tabs.change_body_content_type], "Modify body content-type", None)),
                            ],
                            Protocol::WsRequest(_) => vec![
                                ModifyRequestMessageType(EventKeyBinding::new(vec![key_bindings.request_selected.param_tabs.change_message_type], "Modify message type", None)),
                            ]
                        };

                        base_param_tabs_events.extend(protocol_specific);

                    }

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
                            DuplicateRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.duplicate_element], "Duplicate query param", None)),
                        ],
                        RequestParamsTabs::Auth => vec![
                            EditRequestAuth(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], "Edit auth element", None)),

                            RequestAuthMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], "Move up", None)),
                            RequestAuthMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], "Move down", None)),
                            RequestAuthMoveLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move left", None)),
                            RequestAuthMoveRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move right", None)),
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
                            DuplicateRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.duplicate_element], "Duplicate header", None)),
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
                            DuplicateRequestBodyTableElement(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.duplicate_element], "Duplicate form element", None)),
                        ],
                        RequestParamsTabs::Message => vec![
                            EditRequestMessage(EventKeyBinding::new(vec![key_bindings.generic.list_and_table_actions.edit_element], "Edit message", None)),
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
                    
                        CopyResponsePart(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.yank_response_part], "Yank response part", Some("Yank"))),
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
            EditingRequestUrl => [
                vec![
                    ModifyRequestUrl(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestUrl(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestUrl(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestParam => [
                vec![
                    ModifyRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestQueryParam(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestQueryParam(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, true)
            ].concat(),
            EditingRequestAuthBasicUsername => [
                vec![
                    ModifyRequestAuthBasicUsername(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestAuthBasicUsername(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestAuthBasicUsername(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthBasicPassword => [
                vec![
                    ModifyRequestAuthBasicPassword(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestAuthBasicPassword(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestAuthBasicPassword(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthBearerToken => [
                vec![
                    ModifyRequestAuthBearerToken(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestAuthBearerToken(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestAuthBearerToken(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthJwtSecret => [
                vec![
                    ModifyRequestAuthJwtSecret(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestAuthJwtSecret(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestAuthJwtSecret(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthJwtPayload => [
                vec![
                    ModifyRequestAuthJwtPayload(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_area], "Confirm", Some("Confirm"))),
                    CancelEditRequestAuthJwtPayload(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestAuthJwtPayload(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, false, false)
            ].concat(),
            EditingRequestAuthDigestUsername => [
                vec![
                    ModifyRequestAuthDigestUsername(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestAuthDigestUsername(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestAuthDigestUsername(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthDigestPassword => [
                vec![
                    ModifyRequestAuthDigestPassword(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestAuthDigestPassword(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestAuthDigestPassword(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthDigestDomains => [
                vec![
                    ModifyRequestAuthDigestDomains(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestAuthDigestDomains(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestAuthDigestDomains(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthDigestRealm => [
                vec![
                    ModifyRequestAuthDigestRealm(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestAuthDigestRealm(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestAuthDigestRealm(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthDigestNonce => [
                vec![
                    ModifyRequestAuthDigestNonce(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestAuthDigestNonce(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestAuthDigestNonce(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestAuthDigestOpaque => [
                vec![
                    ModifyRequestAuthDigestOpaque(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestAuthDigestOpaque(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestAuthDigestOpaque(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestHeader => [
                vec![
                    ModifyRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestHeader(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestHeader(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, true)
            ].concat(),
            EditingRequestBodyTable => [
                vec![
                    ModifyRequestBodyTable(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestBodyTable(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestBodyTable(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, true)
            ].concat(),
            EditingRequestBodyFile => [
                vec![
                    ModifyRequestBodyFile(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_single_line], "Confirm", Some("Confirm"))),
                    CancelEditRequestBodyFile(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestBodyFile(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, true, false)
            ].concat(),
            EditingRequestBodyString => [
                vec![
                    ModifyRequestBodyString(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_area], "Confirm", Some("Confirm"))),
                    CancelEditRequestBodyString(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestBodyString(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, false, false)
            ].concat(),
            EditingRequestMessage => [
                vec![
                    ModifyRequestMessage(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_area], "Confirm", Some("Confirm"))),
                    CancelEditRequestMessage(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestMessage(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, false, false)
            ].concat(),
            EditingPreRequestScript => [
                vec![
                    ModifyRequestPreRequestScript(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_area], "Confirm", Some("Confirm"))),
                    CancelEditRequestPreRequestScript(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestPreRequestScript(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, false, false)
            ].concat(),
            EditingPostRequestScript => [
                vec![
                    ModifyRequestPostRequestScript(EventKeyBinding::new(vec![key_bindings.generic.text_input.save_and_quit_area], "Confirm", Some("Confirm"))),
                    CancelEditRequestPostRequestScript(EventKeyBinding::new(vec![key_bindings.generic.text_input.quit_without_saving], "Cancel", Some("Cancel"))),
                    KeyEventEditRequestPostRequestScript(EventKeyBinding::new(vec![], "Any input", None)),
                ],
                generate_text_input_documentation(key_bindings.generic.text_input.mode, false, false)
            ].concat(),
            EditingRequestSettings => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Cancel", Some("Cancel"))),

                RequestSettingsMoveUp(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_up], "Move up", Some("Up"))),
                RequestSettingsMoveDown(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_down], "Move down", Some("Down"))),
                RequestSettingsToggleSettingLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Toggle setting", Some("Toggle left"))),
                RequestSettingsToggleSettingRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Toggle setting", Some("Toggle right"))),

                ModifyRequestSettings(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], "Confirm", Some("Confirm"))),
            ],
            ChoosingRequestExportFormat => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Quit", Some("Quit"))),

                RequestExportFormatMoveCursorLeft(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_left], "Move selection left", Some("Left"))),
                RequestExportFormatMoveCursorRight(EventKeyBinding::new(vec![key_bindings.generic.navigation.move_cursor_right], "Move selection right", Some("Right"))),

                SelectRequestExportFormat(EventKeyBinding::new(vec![key_bindings.generic.navigation.select], "Select export format", Some("Select"))),
            ],
            DisplayingRequestExport => vec![
                GoBackToRequestMenu(EventKeyBinding::new(vec![key_bindings.generic.navigation.go_back], "Quit", Some("Quit"))),

                ScrollRequestExportUp(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_up], "Scroll request export up", None)),
                ScrollRequestExportDown(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_down], "Scroll request export down", None)),
                ScrollRequestExportLeft(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_left], "Scroll request export left", None)),
                ScrollRequestExportRight(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.scroll_right], "Scroll request export right", None)),

                CopyRequestExport(EventKeyBinding::new(vec![key_bindings.request_selected.result_tabs.yank_response_part], "Yank request export", Some("Yank"))),
            ]
        }
    }
}

fn generate_text_input_documentation(text_input_mode: TextAreaMode, single_line: bool, insert_mode_only: bool) -> Vec<AppEvent> {
    let mut initial = Vec::new();

    match text_input_mode {
        TextAreaMode::Vim => {
            if !single_line {
                initial.push(Documentation(EventKeyBinding::new(vec![key!(ctrl-e)], "System editor", None)));
            }

            if !insert_mode_only {
                initial.extend(vec![
                    Documentation(EventKeyBinding::new(vec![key!(esc)], "Normal mode", Some("Esc"))),
                    Documentation(EventKeyBinding::new(vec![key!(i)], "Enter insert mode", None)),
                    Documentation(EventKeyBinding::new(vec![key!(v)], "Enter visual mode", None)),
                    Documentation(EventKeyBinding::new(vec![key!('/')], "Start search", Some("Search"))),
                ]);
            }

        initial.extend(vec![
            Documentation(EventKeyBinding::new(vec![key!(y)], "Copy selection", None)),
            Documentation(EventKeyBinding::new(vec![key!(y), key!(y)], "Copy line", None)),
            Documentation(EventKeyBinding::new(vec![key!(p)], "Paste", None)),
            Documentation(EventKeyBinding::new(vec![key!(u)], "Undo", Some("Undo"))),
            Documentation(EventKeyBinding::new(vec![key!(ctrl-r)], "Redo", Some("Redo"))),
            Documentation(EventKeyBinding::new(vec![key!(w)], "Move to next word", None)),
            Documentation(EventKeyBinding::new(vec![key!(e)], "Move to end of word", None)),
            Documentation(EventKeyBinding::new(vec![key!(b)], "Move to previous word", None)),
            Documentation(EventKeyBinding::new(vec![key!(0)], "Move to start of line", None)),
            Documentation(EventKeyBinding::new(vec![key!('$')], "Move to end of line", None)),
            Documentation(EventKeyBinding::new(vec![key!(g), key!(g)], "Move to first line", None)),
            Documentation(EventKeyBinding::new(vec![key!(G)], "Move to last line", None)),
            Documentation(EventKeyBinding::new(vec![key!(a)], "Append after cursor", None)),
            Documentation(EventKeyBinding::new(vec![key!(o)], "Insert line below", None)),
            Documentation(EventKeyBinding::new(vec![key!(O)], "Insert line above", None)),
            Documentation(EventKeyBinding::new(vec![key!(enter)], "Insert line break", None)),
            Documentation(EventKeyBinding::new(vec![key!(x)], "Delete char", None)),
            Documentation(EventKeyBinding::new(vec![key!(d), key!(d)], "Delete line", None)),
            Documentation(EventKeyBinding::new(vec![key!(D)], "Delete to end of line", None)),
            Documentation(EventKeyBinding::new(vec![*EMPTY_KEY], "Many other vim commands...", None)),
          ]);
        },
        TextAreaMode::Emacs => {
            if !single_line {
                initial.push(Documentation(EventKeyBinding::new(vec![key!(alt-e)], "System editor", None)));
            }

            initial.extend(vec![
                Documentation(EventKeyBinding::new(vec![key!(ctrl-u)], "Undo", Some("Undo"))),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-r)], "Redo", Some("Redo"))),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-y)], "Paste", None)),
                Documentation(EventKeyBinding::new(vec![key!(backspace)], "Remove char from search", None)),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-k)], "Delete to end of line", None)),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-o)], "Insert line break above", None)),
                Documentation(EventKeyBinding::new(vec![key!(enter)], "Insert line break", None)),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-j)], "Insert line break", None)),
                Documentation(EventKeyBinding::new(vec![key!(backspace)], "Delete previous char", None)),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-h)], "Delete previous char", None)),
                Documentation(EventKeyBinding::new(vec![key!(backspace)], "Delete next char", None)),
                Documentation(EventKeyBinding::new(vec![key!(ctrl-d)], "Delete next char", None)),
                Documentation(EventKeyBinding::new(vec![key!(alt-d)], "Delete next word", None)),
                Documentation(EventKeyBinding::new(vec![key!(alt-backspace)], "Delete previous word", None)),
                Documentation(EventKeyBinding::new(vec![*EMPTY_KEY], "Many other emacs shortcuts...", None)),
            ]);

            if !single_line {
                initial.extend(vec![
                    Documentation(EventKeyBinding::new(vec![key!(ctrl-s)], "Start search", Some("Search"))),
                    Documentation(EventKeyBinding::new(vec![key!(ctrl-s)], "Find next match", None)),
                    Documentation(EventKeyBinding::new(vec![key!(ctrl-r)], "Find previous match", None)),
                    Documentation(EventKeyBinding::new(vec![key!(enter)], "Select current search result", None)),
                    Documentation(EventKeyBinding::new(vec![key!(ctrl-g)], "Stop search", None)),
                ]);
            }
        }
        _ => {
            let custom_text_area_bindings = match text_input_mode {
                TextAreaMode::Default => CustomTextArea::default(),
                TextAreaMode::Custom(custom_text_area_bindings) => custom_text_area_bindings,
                _ => unreachable!()
            };

            initial.extend(vec![
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.delete_backward], "Delete char backward", None)),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.delete_forward], "Delete char forward", None)),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.move_cursor_left], "Move cursor left", None)),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.move_cursor_right], "Move cursor right", None)),
            ]);

            if !single_line {
                initial.extend(vec![
                    Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.move_cursor_up], "Move cursor up", None)),
                    Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.move_cursor_down], "Move cursor down", None)),
                ]);
            }

            initial.extend(vec![
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.move_cursor_line_start], "Move cursor line start", Some("Home"))),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.move_cursor_line_end], "Move cursor line end", Some("End"))),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.skip_word_left], "Skip word left", None)),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.skip_word_right], "Skip word right", None)),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.undo], "Undo", Some("Undo"))),
                Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.redo], "Redo", None)),
            ]);

            if !insert_mode_only {
                initial.push(Documentation(EventKeyBinding::new(vec![custom_text_area_bindings.search], "Search", Some("Search"))));
            }
        }
    }

    initial
}

pub fn event_available_keys_to_spans(events: &Vec<AppEvent>, fg_color: Color, bg_color: Color, short_only: bool) -> Vec<Vec<Span<'_>>> {
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
        let is_there_any_env = match self.get_selected_env_as_local() {
            None => false,
            Some(_) => true
        };

        let protocol = match &self.collections_tree.selected {
            Some(selected_request_index) => {
                let local_selected_request = self.collections[selected_request_index.0].requests[selected_request_index.1].clone();
                let selected_request = local_selected_request.read();
                Some(selected_request.protocol.clone())
            },
            None => None
        };
        
        *AVAILABLE_EVENTS.write() = self.state.get_available_events(self.request_view, self.request_param_tab, protocol, is_there_any_env);
    }

    pub fn get_state_line(&self) -> Line<'_> {
        match self.state {
            Normal |
            ChoosingElementToCreate |
            CreatingNewCollection | CreatingNewRequest |
            DisplayingCookies | EditingCookies |
            DisplayingLogs => Line::from(self.state.to_string()).fg(THEME.read().ui.font_color).bg(THEME.read().ui.main_background_color),

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

            DisplayingEnvEditor | EditingEnvVariable => {
                let local_env = self.get_selected_env_as_local().unwrap();
                let env = local_env.read();

                Line::from(vec![
                    Span::raw("Environment editor > ").fg(THEME.read().ui.secondary_foreground_color),
                    Span::raw(env.name.clone()).fg(THEME.read().ui.font_color).bg(THEME.read().ui.main_background_color)
                ])
            },

            SelectedRequest |
            EditingRequestUrl |
            EditingRequestParam |
            EditingRequestAuthBasicUsername | EditingRequestAuthBasicPassword |
            EditingRequestAuthBearerToken |
            EditingRequestAuthJwtSecret | EditingRequestAuthJwtPayload |
            EditingRequestAuthDigestUsername | EditingRequestAuthDigestPassword | EditingRequestAuthDigestDomains | EditingRequestAuthDigestRealm | EditingRequestAuthDigestNonce | EditingRequestAuthDigestOpaque |
            EditingRequestHeader |
            EditingRequestBodyTable | EditingRequestBodyFile | EditingRequestBodyString |
            EditingRequestMessage |
            EditingPreRequestScript | EditingPostRequestScript |
            EditingRequestSettings |
            ChoosingRequestExportFormat | DisplayingRequestExport
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
    
    pub fn in_input(&self) -> bool {
        match self.state {
            EditingEnvVariable |
            EditingCookies |
            CreatingNewCollection |
            CreatingNewRequest |
            RenamingCollection |
            RenamingRequest |
            EditingRequestUrl |
            EditingRequestParam |
            EditingRequestAuthBasicUsername | EditingRequestAuthBasicPassword | EditingRequestAuthBearerToken | EditingRequestAuthJwtSecret | EditingRequestAuthJwtPayload |
            EditingRequestHeader |
            EditingRequestBodyTable | EditingRequestBodyFile | EditingRequestBodyString |
            EditingPreRequestScript | EditingPostRequestScript |
            EditingRequestSettings => true,
            _ => false
        }
    }
}
