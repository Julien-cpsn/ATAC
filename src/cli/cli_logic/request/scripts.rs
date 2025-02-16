use crate::app::app::App;
use crate::models::scripts::ScriptType;

impl App<'_> {
    pub fn cli_print_request_script(&mut self, collection_index: usize, request_index: usize, script_type: &ScriptType) -> anyhow::Result<()> {
        let local_selected_request = self.get_request_as_local_from_indexes(&(collection_index, request_index));

        {
            let selected_request = local_selected_request.read();

            let script = match script_type {
                ScriptType::Pre => &selected_request.scripts.pre_request_script,
                ScriptType::Post => &selected_request.scripts.post_request_script,
            };

            let data = match script {
                None => &String::from("None"),
                Some(script) => script
            };
            
            println!("{data}")
        }

        Ok(())
    }
}