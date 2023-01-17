type menu_enum<'a + enum> = Vec<'a>;

pub enum MenuOptions {
    So,
    Lonely,
}

pub struct MenuItem {
    visible_name: String,
    visible_at_rest: bool,
    alternative_matches: Option<Vec<String>>,
    at_rest_position: Option<usize>,
}

pub struct Menu {
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn serve() {
        println!("You've been served o-");
    }
}
