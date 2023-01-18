pub struct MenuItem {
    pub visible_name: String,
    pub visible_at_rest: bool,
    pub alternative_matches: Option<Vec<String>>,
    pub at_rest_position: Option<usize>,
}

pub struct Menu {
    pub items: Vec<MenuItem>,
}
