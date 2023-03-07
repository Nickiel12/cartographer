use cartographer_rs::{menu, menu_item, MenuOptions};

fn main() {
    let opts = MenuOptions::new().minimum_search_threshold(0.1);

    let menu = menu!(
        "You can hide items to keep from clogging up a main menu\nIt's empty here: ",
        opts,
        [
            menu_item!("Where are all the items?", true, 0),
            menu_item!("We were hiding!", false, 0, ["hello?"]),
            menu_item!("There are a lot of us", false, 0, ["hello?"]),
            menu_item!(
                "Having all these items on the front page would have been quite crowded",
                false,
                0,
                ["hello?"]
            ),
            menu_item!("\\o/", false, 0, ["hello?"]),
            menu_item!(
                "You can assign additional search arguments for these items",
                false,
                0,
                ["hello?"]
            )
        ]
    );

    let usr_selected = menu.serve().unwrap().unwrap();

    println!("\nYou Selected:\n{:?}", usr_selected);
}
