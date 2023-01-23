use console::Key;

#[cfg_attr(
    feature = "serde_serialize",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A data structure representing a line-item in a menu
///
/// The recommended way of constructing these is to use  the [`menu_item!`] macro
/// though the output will be the same
///
/// ## Example
/// ```
/// let menu_item = MenuItem {
///     visible_name: "A Menu Item".to_string(),
///     visible_at_rest: true,
///     at_rest_position: Some(1),
///     alternative_matches: None,
///     }
///
///
/// ```
pub struct MenuItem {
    /// The String that will display for this item in the menu
    pub visible_name: String,

    /// Toggles if this item will be shown when no search terms are available
    pub visible_at_rest: bool,

    /// Optional feature that will let you specify in what order the MenuItems will be
    /// displayed
    pub at_rest_position: Option<usize>,

    /// A list of strings that will also be used, in addition to the [`visible_name`],
    /// when processing the search results
    pub alternative_matches: Option<Vec<String>>,
}

/// The Menu struct that contains the information and
/// functions for displaying the menus
#[derive(Clone, Debug, PartialEq)]
pub struct Menu {
    /// The text to be displayed on the same line as user input will be shown.
    /// To make it extra clear, try adding a semicolon and a space. (e.g. `prompt: "Pick and item: "`)
    pub prompt: String,

    /// The Vector of [`MenuItem`]s
    pub items: Vec<MenuItem>,

    /// The [`MenuOptions`] to use when displaying the menu
    pub configuration: MenuOptions,
}

/// Controls and characters that can be configured
/// to change the way the menu acts and displays
#[derive(Clone, Debug, PartialEq)]
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
