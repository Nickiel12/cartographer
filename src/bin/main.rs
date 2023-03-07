use cartographer_rs::{menu, menu_item};

extern crate cartographer_rs;

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
}
