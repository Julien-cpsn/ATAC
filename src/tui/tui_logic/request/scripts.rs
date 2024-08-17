use tui_textarea::TextArea;

use crate::app::app::App;

impl App<'_> {
    pub fn tui_refresh_pre_request_script_textarea(&mut self, text: &str) {
        let lines: Vec<String> = text
            .lines()
            .map(|line| line.to_string())
            .collect();

        self.script_console.pre_request_text_area = TextArea::new(lines);
    }

    pub fn tui_refresh_post_request_script_textarea(&mut self, text: &str) {
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