use crate::app::app::App;
use crate::models::export::ExportFormat;

impl App<'_> {
    pub fn cli_export_request(&mut self, collection_index: usize, request_index: usize, export_format: &ExportFormat) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.read();

            let export_result = self.export_request_to_string_with_format(export_format, &selected_request);

            println!("{export_result}");
        }
        
        Ok(())
    }
}