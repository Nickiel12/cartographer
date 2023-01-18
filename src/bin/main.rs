use cartographer::Menu;

extern crate cartographer;

fn main() {
    let mut items: Vec<cartographer::MenuItem> = Vec::new();

    items.push(cartographer::MenuItem {
        visible_name: "Item Number 1".to_string(),
        visible_at_rest: true,
        alternative_matches: Some(vec!["1".into(), "Number 1".into()]),
        at_rest_position: Some(0),
    });

    items.push(cartographer::MenuItem {
        visible_name: "Item Number 2".to_string(),
        visible_at_rest: true,
        alternative_matches: Some(vec!["2".into(), "Number 2".into()]),
        at_rest_position: Some(1),
    });

    items.push(cartographer::MenuItem {
        visible_name: "Item Number 3".to_string(),
        visible_at_rest: true,
        alternative_matches: Some(vec!["3".into(), "Number 3".into()]),
        at_rest_position: Some(2),
    });

    let menu: Menu = Menu { items };

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
