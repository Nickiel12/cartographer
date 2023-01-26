use cartographer::{menu, menu_item, MenuOptions};

fn main() {
    let options = MenuOptions::new().cursor('➤').selected_indicator('✓');

    let menu = menu!(
        "So you should try it!: ",
        options,
        [
            menu_item!("Using Cartographer", true, 1),
            menu_item!("Making TUI menus", true, 2),
            menu_item!("Is easy", true, 3),
            menu_item!("Read on for more!", false, 0, ["Ok"])
        ]
    );

    let usr_selection = menu.serve().unwrap().unwrap();
    println!("\nYou Selected:\n{:?}", usr_selection);
}
