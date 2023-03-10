/// Creates a [`MenuItem`](crate::MenuItem), filling in the defaults if values are not provided
///
/// ## Example
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
            use cartographer_rs::MenuItem;
            MenuItem::new($name.to_string())
        }
    };
    ($name:expr, $visible_at_rest:expr) => {
        {
            use cartographer_rs::MenuItem;
            MenuItem::new($name.to_string()).visible_at_rest($visible_at_rest)
        }
    };
    ($name:expr, $visible_at_rest:expr, $default_position:expr) => {
        {
            use cartographer_rs::MenuItem;
            MenuItem::new($name.to_string())
                .visible_at_rest($visible_at_rest)
                .at_rest_position($default_position)
        }
    };
    ($name:expr, $visible_at_rest:expr, $default_position:expr, [$($alt_matches:expr),+]) => {
        {
            use cartographer_rs::MenuItem;
            MenuItem::new($name.to_string())
                .visible_at_rest($visible_at_rest)
                .at_rest_position($default_position)
                .add_alternative_match({
                    let mut matches = Vec::<String>::new();
                    $(
                        matches.push($alt_matches.to_string());
                    )+
                    matches
                })
        }
    };
}

/// Creates a [`Menu`](crate::Menu) in one line
///
/// For the best experience, pair it with the [`menu_item!`](crate::menu_item!) macro for simple menu declaration.
///
/// You can also configure the menu by passing a [`MenuOptions`](crate::MenuOptions). If this is not provided,
/// the defaults are used instead
///
/// ## Example
/// ```
/// let configuration = MenuOptions::new();
/// let menu = menu!(
///       "Pick a number: ",
///       configuration,
///       [
///           menu_item!("Item Number 1", true, 1),
///           menu_item!("Item Number 2", true, 1),
///       ]
/// )
///
/// let usr_choice = menu.serve()?;
/// println!("{}", usr_choice);
/// ```
#[macro_export]
macro_rules! menu {
    ( $prompt:expr, [$( $menu_item:expr ),*]) => {
        {
            use cartographer_rs::{MenuItem, Menu, MenuOptions};
            let mut items = Vec::<MenuItem>::new();
            $(
                items.push($menu_item);
            )*
            Menu::new(
                $prompt.to_string(),
                items,
                Some(MenuOptions::default())
            )
        }

    };
    ( $prompt:expr, $configuration:expr, [$( $menu_item:expr ),*]) => {
        {
            use cartographer_rs::{MenuItem, Menu};
            let mut items = Vec::<MenuItem>::new();
            $(
                items.push($menu_item);
            )*
            Menu::new(
                $prompt.to_string(),
                items,
                Some($configuration)
            )
        }

    };
}
