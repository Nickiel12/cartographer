use cartographer::{Menu, MenuItem};

fn main() {
    let mut menu_items: Vec<MenuItem> = Vec::new();

    menu_items.push(MenuItem::new("You can".to_string()));
    menu_items.push(MenuItem::new("Manually".to_string()));
    menu_items.push(MenuItem::new("Make A".to_string()));
    menu_items.push(MenuItem::new("Menu with".to_string()));
    menu_items.push(MenuItem::new("Pure Structs".to_string()));

    let menu = Menu::new("It is wordy though: ".to_string(), menu_items, None);

    let selection = menu.serve().unwrap().unwrap();
    println!("\n{:?}", selection);
}
