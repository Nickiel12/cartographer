/// Creates a [`crate::MenuItem`], filling in the defaults if values are not provided
///
///
///
/// ## Use
/// ```
/// menu_item!("A Menu Item", true, 2, ["alt search"])
///
/// // Is equal to
///
/// MenuItem {
///     visible_name: "A Menu Item".to_string(),
///     visible_at_rest: true,
///     at_rest_position: Some(2),
///     alternative_matches: Some(vec!["alt search"]),
///     }
/// ```
#[macro_export]
macro_rules! menu_item {
    ($name:expr) => {
        {
            use cartographer::MenuItem;
            MenuItem {
                visible_name: $name.to_string(),
                visible_at_rest: true,
                at_rest_position: None,
                alternative_matches: None,
            }
        }
    };
    ($name:expr, $visible_at_rest:expr) => {
        {
            use cartographer::MenuItem;
            MenuItem {
                visible_name: $name.to_string(),
                visible_at_rest: $visible_at_rest,
                at_rest_position: None,
                alternative_matches: None,
            }
        }
    };
    ($name:expr, $visible_at_rest:expr, $default_position:expr) => {
        {
            use cartographer::MenuItem;
            MenuItem {
                visible_name: $name.to_string(),
                visible_at_rest: $visible_at_rest,
                at_rest_position: Some($default_position),
                alternative_matches: None,
            }
        }
    };
    ($name:expr, $visible_at_rest:expr, $default_position:expr, [$($alt_matches:expr),+]) => {
        {
            use cartographer::MenuItem;
            MenuItem {
                visible_name: $name.to_string(),
                visible_at_rest: $visible_at_rest,
                at_rest_position: Some($default_position),
                alternative_matches: Some({
                    let mut matches = Vec::<String>::new();
                    $(
                        matches.push($alt_matches.to_string());
                    )+
                    matches
                }),
            }
        }
    };
}

/// Creates a [`crate::Menu`] in one line
///
/// For the best experience, pair it with the [`crate::menu_item`] macro for simple menu declaration.
///
/// You can also configure the menu by passing a [`crate::MenuOptions`]. If this is not provided,
/// the defaults are used instead
///
/// ## Example
/// ```
/// let menu = menu![
///       "Pick a number: ",
///       [
///           menu_item!("Item Number 1", true, 1),
///           menu_item!("Item Number 2", true, 1),
///       ]
/// ]
///
/// let usr_choice = menu.serve().unwrap();
/// println!("{}", usr_choice);
/// ```
#[macro_export]
macro_rules! menu {
    ( $prompt:expr, [$( $menu_item:expr ),*]) => {
        {
            use cartographer::{MenuItem, Menu, MenuOptions};
            let mut items = Vec::<MenuItem>::new();
            $(
                items.push($menu_item);
            )*
            Menu {
                prompt: $prompt.to_string(),
                configuration: MenuOptions::default(),
                items
            }
        }

    };
    ( $prompt:expr, $configuration:expr, [$( $menu_item:expr ),*]) => {
        {
            use cartographer::{MenuItem, Menu};
            let items = Vec::<MenuItem>::new();
            $(
                items.push($menu_item);
            )*
            Menu {
                prompt: $prompt.to_string(),
                configuration: $configuration,
                items
            }
        }

    };
}
