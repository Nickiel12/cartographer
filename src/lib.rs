//! A menu builder for Terminal User Interfaces with searching and arrow
//! key navigation

//! ## Example
//! ```
//! assert!(true == true) # because I have no working api yet
//! ```
//!

/// The module with the actual Menu Logic
pub mod interact;

/// Contains the Menu and Menu Item structs for configuration
mod menu_struct;
pub use menu_struct::{Menu, MenuItem, MenuOptions};

/// Contains the menu! and menu_item! macros
mod menu_macros;

pub fn scream_to_the_void() {
    println!("AHAAAAAAAAAAAAAAA");
}
