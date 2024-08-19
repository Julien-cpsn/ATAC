use std::sync::Arc;

use ratatui::style::Stylize;
use ratatui::text::Line;
use tokio::task;
use tracing::info;
use crate::app::app::App;
use crate::app::business_logic::request::send::send_request;
use crate::tui::utils::syntax_highlighting::highlight;

impl App<'_> {
    pub async fn tui_send_request(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        {
            let selected_request = local_selected_request.read();
            
            if selected_request.is_pending {
                selected_request.cancellation_token.cancel();
                info!("Request canceled");
                return;
            }
        }
        
        let mut selected_request = local_selected_request.write();

        /* PRE-REQUEST SCRIPT */
        let mut local_console_output = self.script_console.console_output.write();
        let mut local_highlighted_console_output = self.syntax_highlighting.highlighted_console_output.write();

        // Resets the data
        *local_console_output = None;
        *local_highlighted_console_output = vec![];

        let (prepared_request, console_output) = match self.prepare_request(&*selected_request).await {
            Ok(result) => result,
            Err(prepare_request_error) => {
                selected_request.response.status_code = Some(prepare_request_error.to_string());
                return;
            }
        };

        let mut highlighted_console_output = highlight(&console_output, "json").unwrap();

        highlighted_console_output.insert(0, Line::default());
        highlighted_console_output.insert(1, Line::raw("----- Pre-request script start -----").dark_gray().centered());
        highlighted_console_output.push(Line::raw("----- Pre-request script end -----").dark_gray().centered());

        *local_highlighted_console_output = highlighted_console_output;

        *local_console_output = Some(console_output);

        // Drops the write mutex
        drop(local_console_output);
        drop(local_highlighted_console_output);

        let local_selected_request = self.get_selected_request_as_local();
        let local_env = self.get_selected_env_as_local();
        let local_console_output = Arc::clone(&self.script_console.console_output);
        let local_highlighted_body = Arc::clone(&self.syntax_highlighting.highlighted_body);
        let local_highlighted_console_output = Arc::clone(&self.syntax_highlighting.highlighted_console_output);

        /* SEND REQUEST */

        task::spawn(async move {
            let (response, result_console_output, highlighted_body) = match send_request(prepared_request, local_selected_request.clone(), &local_env).await {
                Ok(response) => response,
                Err(response_error) => {
                    let mut selected_request = local_selected_request.write();
                    selected_request.response.status_code = Some(response_error.to_string());
                    return;
                }
            };

            let mut selected_request = local_selected_request.write();

            let mut console_output = local_console_output.write();
            let mut local_highlighted_console_output = local_highlighted_console_output.write();


            let mut highlighted_console_output = highlight(&result_console_output, "json").unwrap();

            highlighted_console_output.insert(0, Line::default());
            highlighted_console_output.insert(1, Line::raw("----- Post-request script start -----").dark_gray().centered());
            highlighted_console_output.push(Line::raw("----- Post-request script end -----").dark_gray().centered());

            *local_highlighted_body.write() = highlighted_body;
            local_highlighted_console_output.extend(highlighted_console_output);

            *console_output = match console_output.as_ref() {
                None => Some(result_console_output),
                Some(console_output) => Some(format!("{console_output}\n{result_console_output}"))
            };

            selected_request.response = response;
        });
    }
}