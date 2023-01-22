//! A menu builder for Terminal User Interfaces with searching and arrow
//! key navigation
//! 
//! ## Example
//! ```
//! use cartographer::{menu, menu_item};
//! let menu = menu! {
//!       "Pick a number: ",
//!       [
//!           menu_item!("Item Number 1", true, 1),
//!           menu_item!("Item Number 2", false, 2, ["death"]),
//!           menu_item!("Item Number 3", true, 3),
//!           menu_item!("Item Number 5", true, 5, ["80", "5"])
//!       ]
//!   );
//! }
//! 
//! // Returns the string of the item given
//! let usr_selection = menu.serve().unwrap();
//! ```
//!

/// The module with the actual Menu Logic
pub mod interact;

/// Contains the Menu and Menu Item structs for configuration
mod menu_struct;
pub use menu_struct::{Menu, MenuItem, MenuOptions};

/// Contains the menu! and menu_item! macros
mod menu_macros;

