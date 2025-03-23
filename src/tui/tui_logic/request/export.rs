use ratatui::prelude::Line;
use rayon::prelude::*;
use strum::VariantArray;
use crate::app::app::App;
use crate::models::export::ExportFormat;
use crate::tui::utils::syntax_highlighting::highlight;

impl App<'_> {
    pub fn tui_export_request(&mut self) {

        let local_selected_request = self.get_selected_request_as_local();

        {
            let selected_request = local_selected_request.read();

            let export_format = &ExportFormat::VARIANTS[self.export_request.selection];
            let export_result = self.export_request_to_string_with_format(export_format, &selected_request);

            self.display_request_export.content = export_result.clone();
            self.display_request_export.title = export_format.to_string();
            self.display_request_export.horizontal_scrollbar.set_scroll(App::get_max_str_len(export_result.lines()));

            let extension = export_format.to_extension();
            let lines = match extension {
                None => export_result.par_lines().map(|line| Line::from(line.to_string())).collect(),
                Some(extension) => highlight(&export_result, extension).unwrap()
            };

            self.display_request_export.vertical_scrollbar.set_scroll(lines.len() - 1);
            self.display_request_export.lines = lines;
        }

        self.display_request_export_state();
    }
}