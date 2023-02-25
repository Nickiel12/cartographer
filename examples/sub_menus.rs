use cartographer::{menu, menu_item, MenuOptions};
use enum_variants_strings::EnumVariantsStrings;

#[derive(Debug, PartialEq, EnumVariantsStrings)]
enum MenuElements {
    First,
    Second,
    Third,
    Fourth,
}

fn main() {
    let options = MenuOptions::new().cursor("➤").selected_indicator("✓");
    let options2 = MenuOptions::new().cursor("➥").selected_indicator("U");

    let menu = menu!(
        "Using serde for good matching: ",
        options,
        [
            menu_item!(MenuElements::First.to_str()),
            menu_item!(MenuElements::Second.to_str()),
            menu_item!(MenuElements::Third.to_str()),
            menu_item!(MenuElements::Fourth.to_str())
        ]
    );

    let menu2 = menu!(
        "Using serde for good matching: ",
        options2,
        [
            menu_item!("You can also set up a sub_menu"),
            menu_item!("By using matching and other menus")
        ]
    );

    let usr_selection = menu.serve().unwrap().unwrap();
    let mut usr_enum: Vec<MenuElements> = Vec::new();

    for i in usr_selection {
        usr_enum.push(MenuElements::from_str(i.as_str()).unwrap());
    }

    println!("\n");

    for i in usr_enum {
        match i {
            MenuElements::First => {
                println!("You chose the first one!");
            }
            MenuElements::Second => {
                println!("\n");
                let _ = menu2.serve().unwrap().unwrap();
                println!("\nCool, right?");
            }
            MenuElements::Third => {
                println!("3333333333333");
            }
            MenuElements::Fourth => {
                println!("Being last isn't that bad");
            }
        }
    }
}
