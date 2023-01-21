use cartographer::{menu, menu_item};

extern crate cartographer;

fn main() {
    let menu = menu!(
        "Pick a number: ",
        [
            menu_item!("Item Number 1", true, 1),
            menu_item!("Item Number 2", false, 2, ["death"]),
            menu_item!("Item Number 3", true, 3),
            menu_item!("Item Number 5", true, 5, ["80", "5"])
        ]
    );

    menu.serve().unwrap();

    cartographer::scream_to_the_void();
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
