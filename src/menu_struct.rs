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
    pub items: Vec<MenuItem>,
}
