use crate::Menu;
use crate::MenuItem;
use crate::MenuOptions;
use console::Key;
use console::Term;
use rust_fuzzy_search::fuzzy_compare;
use std::io::Write;

struct MenuItemKeepTrack {
    menu_item: MenuItem,
    is_visible: bool,
    is_selected: bool,
}

/// Keeps track of the state of the menu
struct MenuState {
    // Stored user input
    prompt: String,
    inputed: String,
    cursor_row: usize,

    // Live updated info on data rows
    rows: Vec<MenuItemKeepTrack>,

    // stored data that is only read
    term: Term,

    // data about the displayed menu
    lines_written: usize,
}

impl MenuState {
    /// goes through the [`MenuState`], comparing each [`MenuItem`](crate::MenuItem) comparing the
    /// visible_name and alternative_matches to the user's input
    fn search_from_inputed(self: &mut Self, opts: &MenuOptions) {
        // keep a count of how many rows for later use
        let mut num_results = 0;

        // For each row, fuzzy compare, average out the fuzzy score, and if it is greater than the
        // [`MenuOptions'](crate::MenuOptions) configured min_search_threshold. If it is greater,
        // set its visibility to true. (The visibility of the row is what decides if something is
        // shown
        for i in 0..self.rows.len() {
            // The score of this row
            let mut score = 0.0;

            // If the row is already selected, and it is configured to display selected items in
            // search results, then mark all the selected rows as visible
            if self.rows[i].is_selected && opts.show_select_in_search {
                self.rows[i].is_visible = true;
                num_results += 1;
            } else {
                score += fuzzy_compare(
                    &self.rows[i].menu_item.visible_name.to_lowercase(),
                    &self.inputed.to_lowercase(),
                );
                if self.rows[i].menu_item.alternative_matches.is_some() {
                    for i in self.rows[i].menu_item.alternative_matches.clone().unwrap() {
                        score += fuzzy_compare(&i.to_lowercase(), &self.inputed.to_lowercase());
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

        // If there are no search results, default to showing the original menu
        if num_results == 0 {
            for i in 0..self.rows.len() {
                if self.rows[i].menu_item.visible_at_rest {
                    self.rows[i].is_visible = true;
                }
                // Assume that the there were no items shown at some point, and the cursor has been
                // "smooshed to the ceiling"
                self.cursor_row = 0;
            }
        } else {
            // Have the cursor stay in the same percentage zone of the menu (25% down before the
            // search, keep it 25% from the top, after the search)
            if ((self.lines_written as isize) - 3) > 0 {
                if num_results == 1 {
                    self.cursor_row = 0;
                } else {
                    self.cursor_row = (self.cursor_row / (self.lines_written - 3)) * num_results;
                }
            } else {
                self.cursor_row = 0;
            }
        }
    }

    /// Edit the current row's indicator to be visible on user input
    fn mark_selected(self: &mut Self) {
        // Poor man's "filter by visible" for loop
        // counter keeps track of current "visible" row, and if that is the line that the user's
        // cursor is on, sets it to selected, because that is the only row that the user could be
        // trying to select
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

    /// Get the visible string for visible row at item index `item_index`
    fn get_row(
        self: &Self,
        item: &MenuItemKeepTrack,
        cur_redraw_row: usize,
        opts: &MenuOptions,
    ) -> String {
        // If the row we are making a string for, and if the user's cursor is set to that row, set
        // the cursor character, else it is a space
        let cursor = if self.cursor_row == cur_redraw_row {
            opts.cursor.to_string()
        } else {
            " ".to_string()
        };

        // If the line is selected, add the selected character to the string.
        let sel_indicator = match item.is_selected {
            true => opts.selected_indicator.to_string() + " ",
            false => "  ".to_string(),
        };

        return cursor + sel_indicator.as_str() + item.menu_item.visible_name.as_str();
    }

    /// Redraw the menu based on the info in MenuState
    fn redraw(self: &mut Self, opts: &MenuOptions) -> Result<(), std::io::Error> {
        // Keep the number of lines the next draw will write
        let mut next_screen_num_lines = 0;

        // Make a multiline string that represents the next screen
        let mut next_screen = {
            let mut output = String::new();

            // for every item we are keeping track of,
            // if it is "visible", get_row the visible string for it and add it to the next draw
            for i in 0..self.rows.len() {
                let item = self.rows.get(i).unwrap();

                // If adding another line would make it taller than the configured max screen,
                // break early
                if next_screen_num_lines + 1 > opts.max_lines_visible {
                    break;
                }

                if item.is_visible {
                    let x = self.get_row(item, next_screen_num_lines, &opts) + "\n";
                    output += x.as_str();

                    next_screen_num_lines += 1;
                }
            }
            output
        };

        // Add the prompt and the user's input to the redraw String
        next_screen += self.prompt.as_str();
        next_screen += self.inputed.as_str();
        next_screen_num_lines += 1;

        // Clear last menu draw, but ignore this section if it is the first draw
        if self.lines_written != 0 {
            // This line fixes some strange bugs that included prompt lines not being deleted
            // it does cause some flickering in generated video files however
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
    /// Serve a menu. This function is locking and requires a terminal.
    /// It returns a Vec of Strings from the items the user selected
    pub fn serve(self: &Self) -> Result<Option<Vec<String>>, std::io::Error> {
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
                    if (state.cursor_row as i32) <= (state.lines_written as i32) - 3 {
                        state.cursor_row += 1;
                    } else {
                        state.cursor_row = 0;
                    }
                }
                Key::ArrowDown | Key::ArrowRight => {
                    if (state.cursor_row as i32) <= (state.lines_written as i32) - 3 {
                        state.cursor_row += 1;
                    }
                }
                Key::Enter => {
                    break;
                }
                _ => {
                    // Ignore any other keypresses
                    //println!("Heya fella, that key hasn't been implemented yet");
                    //std::thread::sleep(Duration::from_millis(2000));
                    //state.lines_written += 1;
                    continue;
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
