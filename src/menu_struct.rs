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

/// Controls and characters that can be configured
/// to change the way the menu acts and displays
pub struct MenuOptions {
    /// The user's cursor while they navigate
    pub cursor: char,

    /// The char used to indicate an item is selected
    pub selected_indicator: char,

    /// The button the user uses to select an item
    pub select_key: Key,

    /// The maximum number of vertical lines the menu can have
    pub max_lines_visible: usize,

    ///  The minimum search score for an item to be displayed in the menu
    ///  The lower the number, the more results will be displayed
    pub min_search_threshold: f32,

    /// Configures if selected items stay visible in search results
    pub show_select_in_search: bool,
}

impl Default for MenuOptions {
    fn default() -> Self {
        MenuOptions {
            cursor: '>',
            selected_indicator: 'X',
            select_key: Key::Char(' '),
            max_lines_visible: 30,
            min_search_threshold: 0.005,
            show_select_in_search: true,
        }
    }
}
