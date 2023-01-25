//! A menu builder for Terminal User Interfaces with searching and arrow
//! key navigation
//!
//! Use the [`crate::menu_item!`] and [`crate::menu!`] macros for the best effect
//!
//! ## Example
//! ```
//! use cartographer::{menu, menu_item};
//! let menu = menu!(
//!       "Pick a number: ",
//!       [
//!           menu_item!("Item Number 1", true, 1),
//!           menu_item!("Item Number 2", false, 2, ["death"]),
//!           menu_item!("Item Number 3", true, 3),
//!           menu_item!("Item Number 5", true, 5, ["80", "5"])
//!       ]
//!   );
//!
//!
//! // Returns the string of the item given
//! let usr_selection = menu.serve()?;
//! ```
//!

/// Contains the Menu and Menu Item structs for configuration
mod menu;
pub use menu::{Menu, MenuItem, MenuOptions};

/// Contains the menu! and menu_item! macros
mod menu_macros;
