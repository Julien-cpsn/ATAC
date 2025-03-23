#[derive(Default)]
pub struct TextInput {
    pub text: String,
    pub cursor_position: usize
}

const ELLIPSIS: &str = "..";

impl TextInput {
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn move_cursor_line_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_cursor_line_end(&mut self) {
        self.cursor_position = self.text.chars().count();
    }

    pub fn enter_char(&mut self, new_char: char) {
        // Remove ASCII-only check to support UTF-8
        let byte_index = self.get_byte_index(self.cursor_position);
        self.text.insert(byte_index, new_char);
        self.move_cursor_right();
    }

    pub fn enter_str(&mut self, string: &str) {
        for char in string.chars() {
            self.enter_char(char)
        }
    }

    pub fn delete_char_backward(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Use character-based removal instead of byte manipulation
            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character
            let before_char_to_delete = self.text.chars().take(from_left_to_current_index);
            // Getting all characters after selected character
            let after_char_to_delete = self.text.chars().skip(current_index);

            // Put all characters together except the selected one
            self.text = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn delete_char_forward(&mut self) {
        let is_not_cursor_rightmost = self.cursor_position < self.text.chars().count();

        if is_not_cursor_rightmost {
            // Use character-based removal instead of byte manipulation
            let current_index = self.cursor_position;

            // Getting all characters before the selected character
            let before_char_to_delete = self.text.chars().take(current_index);
            // Getting all characters after selected character
            let after_char_to_delete = self.text.chars().skip(current_index + 1);

            // Put all characters together except the selected one
            self.text = before_char_to_delete.chain(after_char_to_delete).collect();
        }
    }

    // Helper method to convert character index to byte index
    fn get_byte_index(&self, char_index: usize) -> usize {
        self.text.char_indices()
            .nth(char_index)
            .map_or(self.text.len(), |(byte_index, _)| byte_index)
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.text.chars().count())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    pub fn reset_input(&mut self) {
        self.text.clear();
        self.reset_cursor();
    }

    /// Returns the text with ellipsis if the cursor is further the input box length
    pub fn get_padded_text_and_cursor(&self, length: usize) -> (String, usize) {
        let text_char_count = self.text.chars().count();

        // if the text is shorter than the desired length
        if text_char_count <= length {
            return (self.text.clone(), self.cursor_position);
        }

        let ellipsis_char_count = ELLIPSIS.chars().count();

        // Calculate the visible portion length
        let first_part_length = length - ellipsis_char_count;

        // If cursor is within the first visible portion
        if self.cursor_position <= first_part_length {
            // Take the first part of the text (in characters)
            let first_part: String = self.text.chars().take(first_part_length).collect();
            let text = format!("{}{}", first_part, ELLIPSIS);
            return (text, self.cursor_position);
        }

        // For cursor positions beyond the first visible portion
        let double_adjusted_length = length - 2 * ellipsis_char_count;
        let char_vec: Vec<char> = self.text.chars().collect();

        // Calculate the number of "pages" of text
        let nb_lengths_text = ((text_char_count - first_part_length) / double_adjusted_length) + 1;
        let nb_lengths_cursor = ((self.cursor_position - first_part_length) / double_adjusted_length) + 1;

        // Calculate the starting character index for the visible portion
        let start_index = first_part_length + (nb_lengths_cursor - 1) * double_adjusted_length;

        if nb_lengths_cursor == nb_lengths_text {
            // If cursor is in the last "page"
            let text = format!("{}{}", ELLIPSIS, char_vec[start_index..].iter().collect::<String>());
            return (text, self.cursor_position + ellipsis_char_count - start_index);
        } else {
            // If cursor is in a middle "page"
            let end_index = std::cmp::min(start_index + double_adjusted_length, text_char_count);
            let visible_text: String = char_vec[start_index..end_index].iter().collect();
            let text = format!("{}{}{}", ELLIPSIS, visible_text, ELLIPSIS);
            return (text, self.cursor_position + ellipsis_char_count - start_index);
        }
    }
}