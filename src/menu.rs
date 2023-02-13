mod interact;

#[cfg_attr(
    feature = "serde_serialize",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A data structure representing a line-item in a menu
///
/// The recommended way of constructing these is to use  the [`menu_item!`](crate::menu_item!) macro
/// though the output will be the same
///
/// ## Example
/// ```
/// let menu_item = MenuItem::new("A Menu Item".to_string())
///     .visible_at_rest(true)
///     .at_rest_position(1);
///
/// // is the same as
///
/// menu_item!("A Menu Item", true, 1);
/// ```
pub struct MenuItem {
    /// The String that will display for this item in the menu
    visible_name: String,

    /// Toggles if this item will be shown when no search terms are available
    visible_at_rest: bool,

    /// Optional feature that will let you specify in what order the MenuItems will be
    /// displayed
    at_rest_position: Option<usize>,

    /// A list of strings that will also be used, in addition to the `visible_name`,
    /// when processing the search results
    alternative_matches: Option<Vec<String>>,
}

impl MenuItem {
    /// Create a new MenuItem with the visible name specified
    pub fn new(visible_name: String) -> Self {
        MenuItem {
            visible_name,
            visible_at_rest: true,
            at_rest_position: None,
            alternative_matches: None,
        }
    }

    /// Set whether a [`MenuItem`] is visible when no search is showing
    pub fn visible_at_rest(self, visible: bool) -> Self {
        MenuItem {
            visible_at_rest: visible,
            ..self
        }
    }

    /// Set a [`MenuItem`]'s resting position in the no search menu
    /// Note: Won't have any effect if visible_at_rest is false
    pub fn at_rest_position(self, position: usize) -> Self {
        MenuItem {
            at_rest_position: Some(position),
            ..self
        }
    }

    /// Set alternative matches for a [`MenuItem`]. These are strings that this item will
    /// match to when searching - in addition to the visible_name
    pub fn add_alternative_match(self, new_matches: Vec<String>) -> Self {
        let mut cur_matches: Vec<String>;
        if self.alternative_matches.is_none() {
            cur_matches = Vec::new();
        } else {
            cur_matches = self.alternative_matches.unwrap();
        }
        for i in new_matches {
            cur_matches.push(i);
        }
        MenuItem {
            alternative_matches: Some(cur_matches),
            ..self
        }
    }
}

/// The Menu struct that contains the information and
/// functions for displaying the menus
#[derive(Clone, Debug, PartialEq)]
pub struct Menu {
    /// The text to be displayed on the same line as user input will be shown.
    /// To make it extra clear, try adding a semicolon and a space. (e.g. `prompt: "Pick and item: "`)
    prompt: String,

    /// The Vector of [`MenuItem`]s
    items: Vec<MenuItem>,

    /// The [`MenuOptions`] to use when displaying the menu
    configuration: MenuOptions,
}

impl Menu {
    /// Create a new Menu from a prompt, list of [`MenuItem`](crate::MenuItem)s, and an optional
    /// [`MenuOptions`](crate::MenuOptions) instance. If configuration is `None`, then the default
    /// is used
    pub fn new(
        prompt: String,
        menu_items: Vec<MenuItem>,
        configuration: Option<MenuOptions>,
    ) -> Menu {
        Menu {
            prompt,
            items: menu_items,
            configuration: {
                if let Some(configuration) = configuration {
                    configuration
                } else {
                    MenuOptions {
                        ..MenuOptions::default()
                    }
                }
            },
        }
    }
}

/// Controls and characters that can be configured
/// to change the way the menu acts and displays
///
///
/// ## Example
/// ```
/// let options = MenuOptions::new()
///     .cursor("→")
///     .select_key(console::Key::Tab)
///     .max_lines_visible(6);
///
/// menu!("Only 6 lines are visible!",
///     menu_items_list,
///     options
///     )
///
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct MenuOptions {
    /// The user's cursor while they navigate
    cursor: String,
    cursor_width: usize,

    /// The char used to indicate an item is selected
    selected_indicator: String,
    selected_indicator_width: usize,

    /// The button the user uses to select an item
    select_key: console::Key,

    /// The maximum number of vertical lines the menu can have
    max_lines_visible: usize,

    ///  The minimum search score for an item to be displayed in the menu
    ///  The lower the number, the more results will be displayed
    min_search_threshold: f32,

    /// Configures if selected items stay visible in search results
    show_select_in_search: bool,

    /// Set if the menu returns the first selected item
    only_one: bool,

    /// Set if the menu cleans up the terminal after exiting
    clear_menu_on_exit: bool,
}

impl MenuOptions {
    /// Create a new [`MenuOptions`] with all the defaults
    pub fn new() -> Self {
        MenuOptions {
            ..MenuOptions::default()
        }
    }

    /// Set the user's row-indicator/cursor to a custom character.
    /// The default is: '>'
    pub fn cursor(self, cursor: &str) -> Self {
        MenuOptions { cursor: cursor.to_string(), ..self }
    }
    /// Override the space given for the cursor navigation column
    pub fn cursor_width(self, cursor_width: usize) -> Self {
        MenuOptions { cursor_width, ..self }
    }

    /// Set the "Item Selected" indicator to a custom character.
    /// Defaults is: 'X'
    pub fn selected_indicator(self, indicator: &str) -> Self {
        MenuOptions {
            selected_indicator: indicator.to_string(),
            ..self
        }
    }
    /// Override the space given for the cursor navigation column
    pub fn selected_indicator_width(self, indicator_width: usize) -> Self {
        MenuOptions { selected_indicator_width: indicator_width, ..self }
    }
    /// Set the key that is used to select an item.
    /// The default is: [`console::Key::Char(' ')`]
    pub fn select_key(self, key: console::Key) -> Self {
        MenuOptions {
            select_key: key,
            ..self
        }
    }
    /// Set the maximum number of items that will be displayed at any one time.
    /// The default is: 10
    pub fn max_lines_visible(self, max_lines: usize) -> Self {
        MenuOptions {
            max_lines_visible: max_lines,
            ..self
        }
    }
    /// Set the degree of "fuzziness" that it will match too. Higher numbers will return more
    /// results, but less accurate ones. Has to be 1.0 >= x >= 0 or will panic
    /// The default is: 0.005
    pub fn minimum_search_threshold(self, threshold: f32) -> Self {
        assert!( (0.0..1.0).contains(&threshold) );
        MenuOptions {
            min_search_threshold: threshold,
            ..self
        }
    }
    /// Set if 'selected' rows are still shown during searches they aren't results for
    /// The default is: true
    pub fn show_selected_in_search(self, show_in_search: bool) -> Self {
        MenuOptions {
            show_select_in_search: show_in_search,
            ..self
        }
    }
    /// Set if the menu should exit and return only the first user selection.
    /// The default is: false
    pub fn only_one_selection(self, only_one: bool) -> Self {
        MenuOptions { only_one, ..self }
    }
    /// Set if the menu should delete any left-over lines from the terminal.
    /// The default is: true
    pub fn clear_on_close(self, do_clear: bool) -> Self {
        MenuOptions {
            clear_menu_on_exit: do_clear,
            ..self
        }
    }
}

impl Default for MenuOptions {
    fn default() -> Self {
        MenuOptions {
            cursor: ">".to_string(),
            cursor_width: 1,
            selected_indicator: "X".to_string(),
            selected_indicator_width: 1,
            select_key: console::Key::Char(' '),
            max_lines_visible: 10,
            min_search_threshold: 0.005,
            show_select_in_search: true,
            only_one: false,
            clear_menu_on_exit: true,
        }
    }
}
