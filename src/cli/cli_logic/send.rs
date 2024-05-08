use std::sync::Arc;
use parking_lot::RwLock;
use crate::app::app::App;
use crate::app::business_logic::request::send::send_request;
use crate::cli::send::CollectionAndRequestArg::{CollectionAndRequest, CollectionOnly};
use crate::cli::send::SendCommand;
use crate::models::request::Request;
use crate::models::response::ResponseContent;

impl App<'_> {
    pub async fn send_request_command(&mut self, send_command: SendCommand) -> anyhow::Result<()> {
        let send_command_copy = send_command.clone();
        
        match send_command.collection_and_request {
            CollectionOnly(collection_name) => {
                let collection = self.find_collection(collection_name)?;

                let mut requests: Vec<Arc<RwLock<Request>>> = vec![];
                
                for request in &collection.requests {
                    let local_request = request.clone();
                    requests.push(local_request);
                }
                
                for request in requests {
                    self.local_send_request(&send_command_copy, request).await?
                }
            },
            CollectionAndRequest(collection_name, request_name) => {
                let local_request = self.find_collection_and_request(collection_name, request_name)?;

                self.local_send_request(&send_command_copy, local_request).await?;
            }
        }

        Ok(())
    }

    async fn local_send_request(&self, send_command: &SendCommand, local_request: Arc<RwLock<Request>>) -> anyhow::Result<()> {
        let request = local_request.read();

        if send_command.request_name {
            println!("{}", request.name);
        }
        
        let (prepared_request, mut console_output) = self.prepare_request(&request).await?;

        drop(request);

        let (response, result_console_output, _) = send_request(prepared_request, local_request, &None).await?;

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