#[derive(Default)]
pub struct TextInput {
    pub text: String,
    pub cursor_position: usize,
}

const ELLIPSIS: &str = "..";
const ELLIPSIS_LENGTH: usize = ELLIPSIS.len();

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
        self.cursor_position = self.text.len();
    }

    pub fn enter_char(&mut self, new_char: char) {
        if !new_char.is_ascii() {
            return;
        }

        self.text.insert(self.cursor_position, new_char);
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
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.text.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.text.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.text = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn delete_char_forward(&mut self) {
        let is_not_cursor_rightmost = self.cursor_position != self.text.len();

        if is_not_cursor_rightmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.text.chars().take(current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.text.chars().skip(current_index + 1);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.text = before_char_to_delete.chain(after_char_to_delete).collect();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.text.len())
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
        // if the text is longer than the desired length
        if self.text.len() < length {
            return (self.text.clone(), self.cursor_position);
        }

        let first_part = &self.text[..length-ELLIPSIS_LENGTH];

        if (self.cursor_position / (length - ELLIPSIS_LENGTH)) == 0 {
            let text = format!("{first_part}{ELLIPSIS}");
            return (text, self.cursor_position)
        }

        let simple_adjusted_length = length  - ELLIPSIS_LENGTH;
        let double_adjusted_length = length  - 2 * ELLIPSIS_LENGTH;
        let nb_lengths_text = ((self.text.len() - simple_adjusted_length) / double_adjusted_length) + 1;
        let nb_lengths_cursor = ((self.cursor_position - simple_adjusted_length) / double_adjusted_length) + 1;
        let start_index = simple_adjusted_length + (nb_lengths_cursor - 1) * double_adjusted_length;

        if nb_lengths_cursor == nb_lengths_text {
            let text = format!("{ELLIPSIS}{}", &self.text[start_index..]);
            return (text, self.cursor_position + ELLIPSIS_LENGTH - start_index)
        }
        else {
            let text = format!("{ELLIPSIS}{}{ELLIPSIS}", &self.text[start_index..start_index + double_adjusted_length]);
            return (text, self.cursor_position + ELLIPSIS_LENGTH - start_index)
        }
    }
}