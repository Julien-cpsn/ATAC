use boa_engine::{Context, JsString, NativeFunction, Source};
use indexmap::IndexMap;
use tui_textarea::TextArea;

use crate::app::app::App;
use crate::request::request::Request;
use crate::request::response::RequestResponse;

use super::script_support::generate_signed_jwt;

impl App<'_> {
    pub fn refresh_pre_request_script_textarea(&mut self, text: &str) {
        let lines: Vec<String> = text
            .lines()
            .map(|line| line.to_string())
            .collect();

        self.script_console.pre_request_text_area = TextArea::new(lines);
    }

    pub fn refresh_post_request_script_textarea(&mut self, text: &str) {
        let lines: Vec<String> = text
            .lines()
            .map(|line| line.to_string())
            .collect();

        self.script_console.post_request_text_area = TextArea::new(lines);
    }

    pub fn modify_pre_request_script(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            let pre_request_script = self.script_console.pre_request_text_area.lines().join("\n");

            if pre_request_script.is_empty() {
                selected_request.scripts.pre_request_script = None;
            }
            else {
                selected_request.scripts.pre_request_script = Some(pre_request_script);
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }

    pub fn modify_post_request_script(&mut self) {
        let selected_request_index = &self.collections_tree.selected.unwrap();
        let local_selected_request = self.get_request_as_local_from_indexes(selected_request_index);

        {
            let mut selected_request = local_selected_request.write();

            let post_request_script = self.script_console.post_request_text_area.lines().join("\n");

            if post_request_script.is_empty() {
                selected_request.scripts.post_request_script = None;
            }
            else {
                selected_request.scripts.post_request_script = Some(post_request_script);
            }
        }

        self.save_collection_to_file(selected_request_index.0);
        self.select_request_state();
    }
}

const JS_CONSOLE: &str = r#"
let console_log_output = "";

globalThis.console = {
  log: (msg) => {
    console_log_output += msg + '\n';
    return msg;
  }
}
"#;

const JS_UTILS: &str = r#"
function pretty_print(data) {
    console.log(JSON.stringify(data, null, 2));
}
"#;

pub(super) fn execute_pre_request_script(user_script: &String, request: &Request, env: Option<IndexMap<String, String>>) -> (Option<Request>, Option<IndexMap<String, String>>, String) {
    // Instantiate the execution context
    let mut context = Context::default();
    context.register_global_callable(JsString::from("generate_signed_jwt"), 0, NativeFunction::from_fn_ptr(generate_signed_jwt)).unwrap();

    let request_json = serde_json::to_string(request).unwrap();
    let env_json = match &env {
        Some(env) => serde_json::to_string(env).unwrap(),
        None => String::from("undefined")
    };

    let script = format!(r#"
        let request = {request_json};
        let env = {env_json};

        {JS_CONSOLE}
        {JS_UTILS}

        /* Start of the user script */

        {user_script}

        /* End of the user script */

        JSON.stringify([request, env, console_log_output])
    "#);

    let result = match context.eval(Source::from_bytes(&script)) {
        Ok(result) => result,
        Err(error) => {
            return (None, env, error.to_string())
        }
    };

    let stringed_result = result.as_string().unwrap().to_std_string_escaped();

    let (result_request, result_env_values, console_output) = match serde_json::from_str::<(Request, Option<IndexMap<String, String>>, String)>(&stringed_result) {
        Ok((result_request, result_env_values, console_output)) => (Some(result_request), result_env_values, console_output),
        Err(error) => (None, env, error.to_string())
    };

    return (result_request, result_env_values, console_output);
}

pub(super) fn execute_post_request_script(user_script: &String, response: &RequestResponse, env: Option<IndexMap<String, String>>) -> (Option<RequestResponse>, Option<IndexMap<String, String>>, String) {
    // Instantiate the execution context
    let mut context = Context::default();
    context.register_global_callable(JsString::from("generate_signed_jwt"), 0, NativeFunction::from_fn_ptr(generate_signed_jwt)).unwrap();

    let response_json = serde_json::to_string(response).unwrap();
    let env_json = match &env {
        Some(env) => serde_json::to_string(env).unwrap(),
        None => String::from("undefined")
    };

    let script = format!(r#"
        let response = {response_json};
        let env = {env_json};

        {JS_CONSOLE}
        {JS_UTILS}

        /* Start of the user script */

        {user_script}

        /* End of the user script */

        JSON.stringify([response, env, console_log_output])
    "#);

    let result = match context.eval(Source::from_bytes(&script)) {
        Ok(result) => result,
        Err(error) => {
            return (None, env, error.to_string())
        }
    };

    let stringed_result = result.as_string().unwrap().to_std_string_escaped();

    let (response_result, result_env_values, console_output) = match serde_json::from_str::<(RequestResponse, Option<IndexMap<String, String>>, String)>(&stringed_result) {
        Ok((mut response_result, result_env_values, console_output)) => {
            // Avoid loosing those fields since they are not serialized
            response_result.duration = response.duration.clone();
            response_result.status_code = response.status_code.clone();

            (Some(response_result), result_env_values, console_output)
        },
        Err(error) => (None, env, error.to_string())
    };

    return (response_result, result_env_values, console_output);
}
