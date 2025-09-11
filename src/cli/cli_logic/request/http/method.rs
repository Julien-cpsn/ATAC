use crate::app::app::App;

impl App<'_> {
    pub fn cli_print_request_method(&mut self, collection_index: usize, request_index: usize) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.read();
            let selected_http_request = selected_request.get_http_request()?;
            println!("{}", selected_http_request.method.to_string())
        }
        
        Ok(())
    }
}