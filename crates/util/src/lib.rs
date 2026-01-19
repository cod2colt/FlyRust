#![doc = include_str!("../README.md")]

// mod
pub mod assets;
pub mod customfonts;
pub mod sqlite;
pub mod utilities;

// use
pub use customfonts::setup_custom_fonts;
pub use sqlite::MyScore;
pub use utilities::get_resource_path_str;
pub use utilities::what_panic;
