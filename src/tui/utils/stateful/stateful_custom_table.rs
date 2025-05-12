use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::models::request::KeyValue;
use crate::tui::app_states::AppState;
use crate::tui::utils::stateful::text_input::TextInput;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::prelude::{Line, Modifier, Style, Stylize};
use ratatui::widgets::{Block, Borders, List, ListState, Paragraph};
use ratatui::Frame;


#[derive(Default)]
pub struct StatefulCustomTable {
    pub left_state: ListState,
    pub right_state: ListState,
    /// (x, y)
    pub selection: Option<(usize, usize)>,
    pub rows: Vec<KeyValue>,
    pub selection_text_input: TextInput,
}

impl StatefulCustomTable {
    pub fn update_selection(&mut self, selection: Option<(usize, usize)>) {
        match selection {
            None => {
                self.selection = None;
                self.left_state.select(None);
                self.right_state.select(None);
            }
            Some(selection) => {
                self.selection = Some(selection);
                self.left_state.select(Some(selection.0));
                self.right_state.select(Some(selection.1));
            }
        }
    }
    
    fn decrement_x(&self, i: usize) -> usize {
        if i == 0 {
            self.rows.len() - 1
        } else {
            i - 1
        }
    }

    fn increment_x(&self, i: usize) -> usize {
        if i >= self.rows.len() - 1 {
            0
        } else {
            i + 1
        }
    }

    pub fn change_y(&mut self) {
        if self.rows.is_empty() || self.selection.is_none() {
            return;
        }

        match self.selection.unwrap() {
            (x, 0) => self.selection = Some((x, 1)),
            (x, 1) => self.selection = Some((x, 0)),
            (x, _) => self.selection = Some((x, 0))
        }

        let x = self.selection.unwrap().0;

        self.right_state.select(Some(x));
        self.left_state.select(Some(x));
    }

    pub fn up(&mut self) {
        if self.rows.is_empty() || self.selection.is_none() {
            return;
        }

        let x = match self.selection.unwrap() {
            (_, 0) => match self.left_state.selected() {
                None => 0,
                Some(i) => self.decrement_x(i)
            },
            (_, 1) => match self.right_state.selected() {
                None => 0,
                Some(i) => self.decrement_x(i)
            },
            (_, _) => 0
        };

        self.left_state.select(Some(x));
        self.right_state.select(Some(x));

        match self.selection.unwrap() {
            (_, y) => self.selection = Some((x, y))
        }
    }

    pub fn down(&mut self) {
        if self.rows.is_empty() || self.selection.is_none() {
            return;
        }

        let x = match self.selection.unwrap() {
            (_, 0) => match self.left_state.selected() {
                None => 0,
                Some(i) => self.increment_x(i)
            },
            (_, 1) => match self.right_state.selected() {
                None => 0,
                Some(i) => self.increment_x(i)
            },
            (_, _) => 0
        };

        self.left_state.select(Some(x));
        self.right_state.select(Some(x));

        match self.selection.unwrap() {
            (_, y) => self.selection = Some((x, y))
        }
    }

    pub fn is_selected(&self) -> bool {
        return self.selection.is_some();
    }
}

impl App<'_> {
    pub fn render_custom_table(
        &self,
        frame: &mut Frame,
        area: Rect,
        table: &StatefulCustomTable,
        no_selection_lines: Vec<Line>,
        editing_state: AppState,
        key_name: &'static str,
        value_name: &'static str
    ) {
        match table.selection {
            None => {
                let headers_paragraph = Paragraph::new(no_selection_lines).centered();

                frame.render_widget(headers_paragraph, area);
            },
            Some(selection) => {
                let layout = Layout::new(
                    Vertical,
                    [
                        Constraint::Length(2),
                        Constraint::Fill(1)
                    ]
                )
                    .split(area);

                let inner_layout = Layout::new(
                    Horizontal,
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50)
                    ]
                )
                    .split(layout[0]);

                let title = Paragraph::new(key_name)
                    .centered()
                    .block(
                        Block::new()
                            .borders(Borders::BOTTOM | Borders::RIGHT)
                            .fg(THEME.read().ui.secondary_foreground_color)
                    )
                    .fg(THEME.read().ui.secondary_foreground_color);

                let form_value = Paragraph::new(value_name)
                    .centered()
                    .block(
                        Block::new()
                            .borders(Borders::BOTTOM)
                            .fg(THEME.read().ui.secondary_foreground_color)
                    )
                    .fg(THEME.read().ui.secondary_foreground_color);

                frame.render_widget(title, inner_layout[0]);
                frame.render_widget(form_value, inner_layout[1]);

                let horizontal_margin = 2;

                let table_layout = Layout::new(
                    Horizontal,
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50)
                    ]
                )
                    .horizontal_margin(horizontal_margin)
                    .split(layout[1]);

                let mut left_list_style = Style::default();
                let mut right_list_style = Style::default();

                match selection.1 {
                    0 => left_list_style = left_list_style
                        .add_modifier(Modifier::BOLD)
                        .fg(THEME.read().others.selection_highlight_color),
                    1 => right_list_style = right_list_style
                        .add_modifier(Modifier::BOLD)
                        .fg(THEME.read().others.selection_highlight_color),
                    _ => {}
                }

                let (keys, values) = self.key_value_vec_to_items_list(&table.rows);

                let left_list = List::new(keys)
                    .highlight_style(left_list_style)
                    .fg(THEME.read().ui.font_color);

                let right_list = List::new(values)
                    .highlight_style(right_list_style)
                    .fg(THEME.read().ui.font_color);

                frame.render_stateful_widget(left_list, table_layout[0], &mut table.left_state.clone());
                frame.render_stateful_widget(right_list, table_layout[1], &mut table.right_state.clone());

                // Form input & cursor

                if self.state == editing_state {
                    let cell_with = layout[1].width / 2;

                    let width_adjustment = match selection.1 {
                        0 => 0,
                        1 => {
                            let even_odd_adjustment = match layout[1].width % 2 {
                                1 => 1,
                                0 => 2,
                                _ => 0
                            };
                            cell_with - even_odd_adjustment
                        },
                        _ => 0
                    };

                    let height_adjustment = (selection.0 - table.left_state.offset()) as u16 % layout[1].height;

                    let selection_position_x = layout[1].x + width_adjustment + horizontal_margin;
                    let selection_position_y = layout[1].y + height_adjustment;

                    let data_text = table.selection_text_input.text.clone();

                    let text_input = Paragraph::new(format!("{:fill$}", data_text, fill = (cell_with - horizontal_margin) as usize));
                    let text_rect = Rect::new(selection_position_x, selection_position_y, cell_with, 1);

                    frame.render_widget(text_input, text_rect);

                    frame.set_cursor_position(Position::new(
                        selection_position_x + table.selection_text_input.cursor_position as u16,
                        selection_position_y
                    ));
                }
            }
        }
    }
}