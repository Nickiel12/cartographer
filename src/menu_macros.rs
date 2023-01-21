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
