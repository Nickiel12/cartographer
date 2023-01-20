use std::io::Write;

use crate::Menu;
use crate::MenuItem;
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
    /// Get the visible row at item index `item_index`
    fn get_row(self: &Self, item: &MenuItemKeepTrack, cur_redraw_row: usize) -> String {
        let cursor = if self.cursor_row == cur_redraw_row {
            ">".to_string()
        } else {
            " ".to_string()
        };

        let sel_indicator = match item.is_selected {
            true => "X ".to_string(),
            false => "  ".to_string(),
        };

        return cursor + sel_indicator.as_str() + item.menu_item.visible_name.as_str() + "\n";
    }

    /// Redraw the menu based on the info in MenuState
    pub fn redraw(self: &mut Self) -> Result<(), std::io::Error> {
        // count the number of lines the next draw will write
        let mut next_screen_num_lines = 0;
        let mut next_screen = {
            let mut output = String::new();

            // for every item we are keeping track of,
            // if it is "visible", get_row for it and add it to the next draw
            for i in 0..self.rows.len() {
                let item = self.rows.get(i).unwrap();
                if item.is_visible {
                    let x = self.get_row(item, next_screen_num_lines);
                    output += x.as_str();

                    next_screen_num_lines += 1;
                }
            }
            output
        };
        // Clear last menu draw, but ignore the first draw
        if self.lines_written != 0 {
            self.term.clear_last_lines(self.lines_written - 1)?;
            self.lines_written = 0;
        }

        // Add the prompt and the user's input
        // to the redraw
        next_screen += self.prompt.as_str();
        next_screen += self.inputed.as_str();
        next_screen_num_lines += 1;

        self.term.write(&next_screen.as_bytes())?;
        self.lines_written = next_screen_num_lines;

        Ok(())
    }
}

impl Menu {
    pub fn serve(self: &Self) -> Result<usize, std::io::Error> {
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
            state.redraw()?;
            let usr_key = state.term.read_key()?;

            match usr_key {
                Key::Char(c) => state.inputed.push(c),
                Key::ArrowUp => {
                    if state.cursor_row != 0 {
                        state.cursor_row -= 1;
                    }
                }
                Key::ArrowDown => {
                    if state.cursor_row <= state.lines_written - 3 {
                        state.cursor_row += 1;
                    }
                }
                Key::Enter => {
                    println!("Ok, I leave now");
                    break;
                }
                _ => break,
            }
            state.term.clear_line()?;
        }

        //term.write_line("Hiya")?;

        println!("\n {}", state.inputed);

        Ok(1)
    }
}
