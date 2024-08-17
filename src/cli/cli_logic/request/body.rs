use crate::app::app::App;
use crate::app::business_logic::key_value::print_key_value_vector;
use crate::models::body::ContentType;

impl App<'_> {
    pub fn cli_print_request_body(&mut self, collection_index: usize, request_index: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.read();

            println!("{}", selected_request.body);

            match &selected_request.body {
                ContentType::NoBody => {},
                ContentType::Multipart(form) | ContentType::Form(form) => {
                    for key_value in form {
                        println!("\t{}: {}", key_value.data.0, key_value.data.1);
                    }
                },
                ContentType::File(content) | ContentType::Raw(content) | ContentType::Json(content) | ContentType::Xml(content) | ContentType::Html(content) | ContentType::Javascript(content) => {
                    println!("{content}")
                }
            }
        }

        Ok(())
    }

    pub fn cli_print_all_form_data(&mut self, collection_index: usize, request_index: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.read();
            let form = selected_request.body.get_form()?;
            print_key_value_vector(form, None);
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }

    pub fn cli_print_form_data(&mut self, collection_index: usize, request_index: usize, row: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.read();

            let value = &selected_request.headers[row].data.1;

            println!("{value}")
        }

        self.save_collection_to_file(collection_index);
        Ok(())
    }
}