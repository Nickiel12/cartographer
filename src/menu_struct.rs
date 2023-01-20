use console::Key;

#[derive(Clone, Debug)]
/// The data struct for menu items
/// Can be build using the `menu_item!` macro
pub struct MenuItem {
    pub visible_name: String,
    pub visible_at_rest: bool,
    pub alternative_matches: Option<Vec<String>>,
    pub at_rest_position: Option<usize>,
}

/// The Menu struct that contains the information and
/// functions for displaying the menus
pub struct Menu {
    pub prompt: String,
    pub items: Vec<MenuItem>,
    pub configuration: MenuOptions,
}

pub struct MenuOptions {
    pub cursor: char,
    pub select_key: Key,
}

impl Default for MenuOptions {
    fn default() -> Self {
        MenuOptions {
            cursor: '>',
            select_key: Key::Char(' '),
        }
    }
}
