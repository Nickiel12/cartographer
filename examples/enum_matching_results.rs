use cartographer_rs::{menu, menu_item, MenuOptions};
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

    let menu = menu!(
        "You can convert strings to and from enum variants\nto make them easier to match: ",
        options,
        [
            menu_item!(MenuElements::First.to_str()),
            menu_item!(MenuElements::Second.to_str()),
            menu_item!(MenuElements::Third.to_str()),
            menu_item!(MenuElements::Fourth.to_str())
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
                println!("The second one was chosen");
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
