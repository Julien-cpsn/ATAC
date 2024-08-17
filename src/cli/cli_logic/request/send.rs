use std::sync::Arc;
use parking_lot::RwLock;

use crate::app::app::App;
use crate::app::business_logic::request::send::send_request;
use crate::cli::commands::request_commands::send::SendCommand;
use crate::models::request::Request;
use crate::models::response::ResponseContent;

impl App<'_> {
    pub async fn cli_send_request(&mut self, collection_index: usize, request_index: usize, send_command: &SendCommand) -> anyhow::Result<()> {
        let local_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        self.local_send_request(&send_command, local_request).await?;

        Ok(())
    }

    pub async fn cli_send_collection(&mut self, collection_name: &str, send_command: &SendCommand) -> anyhow::Result<()> {
        let collection_index = self.find_collection(collection_name)?;
        let collection = &self.collections[collection_index];

        let mut requests: Vec<Arc<RwLock<Request>>> = vec![];

        for request in &collection.requests {
            let local_request = request.clone();
            requests.push(local_request);
        }

        for request in requests {
            self.local_send_request(&send_command, request).await?
        }

        Ok(())
    }

    pub async fn local_send_request(&mut self, send_command: &SendCommand, local_request: Arc<RwLock<Request>>) -> anyhow::Result<()> {
        let request = local_request.read();

         if let Some(env_name )= &send_command.env {
            let env_index = self.find_environment(env_name)?;
             self.selected_environment = env_index;
        };
        
        if send_command.request_name {
            println!("{}", request.name);
        }
        
        let (prepared_request, mut console_output) = self.prepare_request(&request).await?;

        drop(request);

        let local_env = self.get_selected_env_as_local();
        let (response, result_console_output, _) = send_request(prepared_request, local_request, &local_env).await?;

        console_output = format!("{console_output}{result_console_output}");

        if send_command.status_code {
            println!("{}", response.status_code.unwrap());
        }

        if send_command.duration {
            println!("{}", response.duration.unwrap());
        }

        if send_command.cookies {
            println!("{}", response.cookies.unwrap());
        }

        if send_command.headers {
            println!("{:?}", response.headers);
        }

        if send_command.console {
            println!("{}", console_output);
        }

        if !send_command.hide_content {
            match response.content.unwrap() {
                ResponseContent::Body(body) => println!("{}", body),
                ResponseContent::Image(image) => println!("{:?}", image.data)
            };
        }

        Ok(())
    }
}