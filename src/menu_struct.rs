use console::Key;

#[derive(Clone, Debug)]
pub struct MenuItem {
    pub visible_name: String,
    pub visible_at_rest: bool,
    pub alternative_matches: Option<Vec<String>>,
    pub at_rest_position: Option<usize>,
}

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
