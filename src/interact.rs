use rust_fuzzy_search::fuzzy_compare;
use core::num;
use std::io::Write;
use std::time::Duration;

use crate::Menu;
use crate::MenuItem;
use crate::MenuOptions;
use console::Key;
use console::Term;

struct MenuItemKeepTrack {
    menu_item: MenuItem,
    is_visible: bool,
    is_selected: bool,
}

/// Keeps track of the state of the menu
struct MenuState {
    // Stored user input
    pub prompt: String,
    pub inputed: String,
    pub cursor_row: usize,

    // Live updated info on data rows
    pub rows: Vec<MenuItemKeepTrack>,

    // stored data that is only read
    pub term: Term,

    // data about the displayed menu
    pub lines_written: usize,
}

impl MenuState {
    pub fn search_from_inputed(self: &mut Self, opts: &MenuOptions) {
        let mut num_results = 0;
        for i in 0..self.rows.len() {
            let mut score = 0.0;

            if self.rows[i].is_selected && opts.show_select_in_search {
                self.rows[i].is_visible = true;
                num_results += 1;
            } else {
                score += fuzzy_compare(&self.rows[i].menu_item.visible_name, &self.inputed);
                if self.rows[i].menu_item.alternative_matches.is_some() {
                    for i in self.rows[i].menu_item.alternative_matches.clone().unwrap() {
                        score += fuzzy_compare(&i, &self.inputed);
                        score /= 2.0;
                    }
                }
                if score > opts.min_search_threshold {
                    num_results += 1;
                    self.rows[i].is_visible = true;
                } else {
                    self.rows[i].is_visible = false;
                }
            }
        }

        if num_results == 0 {
            for i in 0..self.rows.len() {
                if self.rows[i].menu_item.visible_at_rest {
                    self.rows[i].is_visible = true;
                }
                self.cursor_row = 0;
            }
        } else {
            self.cursor_row = (self.cursor_row / (self.lines_written - 3)) * num_results;
        }
    }

    /// Edit the current row's indicator to be visible
    fn mark_selected(self: &mut Self) {
        let mut counter = 0;
        for i in 0..self.rows.len() {
            if self.rows[i].is_visible {
                if counter == self.cursor_row {
                    self.rows[i].is_selected = !self.rows[i].is_selected;
                }
                counter += 1;
            }
        }
    }

    /// Get the string for visible row at item index `item_index`
    fn get_row(
        self: &Self,
        item: &MenuItemKeepTrack,
        cur_redraw_row: usize,
        opts: &MenuOptions,
    ) -> String {
        let cursor = if self.cursor_row == cur_redraw_row {
            opts.cursor.to_string()
        } else {
            " ".to_string()
        };

        let sel_indicator = match item.is_selected {
            true => opts.selected_indicator.to_string() + " ",
            false => "  ".to_string(),
        };

        return cursor + sel_indicator.as_str() + item.menu_item.visible_name.as_str() + "\n";
    }

    /// Redraw the menu based on the info in MenuState
    pub fn redraw(self: &mut Self, opts: &MenuOptions) -> Result<(), std::io::Error> {
        // count the number of lines the next draw will write
        let mut next_screen_num_lines = 0;
        let mut next_screen = {
            let mut output = String::new();

            // for every item we are keeping track of,
            // if it is "visible", get_row for it and add it to the next draw
            for i in 0..self.rows.len() {
                let item = self.rows.get(i).unwrap();

                // If the menu + prompt-and-user-input is greater than the
                // maxumum configured lines, exit early
                if next_screen_num_lines + 1 > opts.max_lines_visible {
                    break;
                }

                if item.is_visible {
                    let x = self.get_row(item, next_screen_num_lines, &opts);
                    output += x.as_str();

                    next_screen_num_lines += 1;
                }
            }
            output
        };

        // Add the prompt and the user's input to the redraw
        next_screen += self.prompt.as_str();
        next_screen += self.inputed.as_str();
        next_screen_num_lines += 1;

        // Clear last menu draw, but ignore this section if it is the first draw
        if self.lines_written != 0 {
            self.term.clear_line()?;
            self.term.clear_last_lines(self.lines_written - 1)?;
            self.lines_written = 0;
        }
        // Draw the next menu
        self.term.write(&next_screen.as_bytes())?;
        self.term.flush()?;
        self.lines_written = next_screen_num_lines;

        Ok(())
    }
}

impl Menu {
    pub fn serve(self: &Self) -> Result<Option<Vec::<String>>, std::io::Error> {
        let term = Term::stdout();

        let mut state = MenuState {
            prompt: self.prompt.clone(),
            lines_written: 0,
            cursor_row: 1,
            inputed: String::new(),
            rows: Vec::<MenuItemKeepTrack>::new(),
            term,
        };

        // Load the MenuItems into the MenuState
        for i in 0..self.items.len() {
            let item = self.items[i].clone();

            let mut is_visible = false;
            if self.items[i].visible_at_rest {
                is_visible = true;
            }

            state.rows.push(MenuItemKeepTrack {
                menu_item: item.clone(),
                is_visible,
                is_selected: false,
            });
        }

        loop {
            state.redraw(&self.configuration)?;
            let usr_key = state.term.read_key()?;

            match usr_key {
                Key::Char(c) => {
                    if Key::Char(c) == self.configuration.select_key {
                        state.mark_selected();
                    } else {
                        state.inputed.push(c);
                        state.search_from_inputed(&self.configuration);
                    }
                }
                Key::Backspace => {
                    state.inputed.pop();
                    state.search_from_inputed(&self.configuration);
                }
                Key::ArrowUp | Key::ArrowLeft => {
                    if state.cursor_row != 0 {
                        state.cursor_row -= 1;
                    }
                }
                Key::Tab => {
                    if state.cursor_row <= state.lines_written - 3 {
                        state.cursor_row += 1;
                    } else {
                        state.cursor_row = 0;
                    }
                }
                Key::ArrowDown | Key::ArrowRight => {
                    if state.cursor_row <= state.lines_written - 3 {
                        state.cursor_row += 1;
                    }
                }
                Key::Enter => {
                    break;
                }
                _ => {
                    println!("Heya fella, that key hasn't been implemented yet");
                    std::thread::sleep(Duration::from_millis(2000));
                    state.lines_written += 1;
                }
            }
        }

        let mut output: Vec<String> = Vec::new();
        for i in state.rows {
            if i.is_selected {
                output.push(i.menu_item.visible_name);
            }
        }

        if output.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(output))
        }
    }
}
