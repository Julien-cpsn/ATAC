use crate::app::app::App;

impl App<'_> {
    pub async fn cancel_request(&mut self) {
        let local_selected_request = self.get_selected_request_as_local();

        let selected_request = local_selected_request.write();

        if let Some(task) = &selected_request.abort_tx
        {
            if let Ok(task) = task.lock() {
                let _ = task.send(true);
            }
        }

        self.write_to_log_file("canceling request".to_string(), self.state.to_string());
    }
}
