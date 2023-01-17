extern crate cartographer;

fn main() {
    println!("Hello, world!");
    cartographer::scream_to_the_void();

    cartographer::Menu::serve();
}

// I wish to be able to say:
// #[cartographer::Menu]
// struct MainMenu {
//
// #[menu_item]
// Add
//
// #[menu_item,
//     alternative_searches=["UnAdd", "Remove", "stop adding"]
// Reset
//
// #[menu_item,
//     hidden_from_startmenu]
// Branches
//
// }
//
// user_item: MainMenuEnum = MainMenu::serve()?;
//
// match user_item
