#![warn(missing_docs)]
//! This crate provides some features to convert color code, RGB or color name(if exists) to struct `Color` which holds the color information.
//!
//! What makes this crate (kind of) unique is that it supports the color name conversion. For example:
//! ```
//! use colorconv::Color;
//! use std::str::FromStr;
//!
//! match Color::from_str("yale blue") {
//!     Ok(color) => assert_eq!(color.hex, "0f4d92".to_string()),
//!     Err(e) => eprintln!("{:?}", e),
//! }
//! ```
//! This conversion is based on <https://github.com/jonathantneal/color-names>.
//!
//! Also, you can convert a color code or RGB:
//! ```
//! use colorconv::Color;
//! use std::str::FromStr;
//!
//! if let Ok(rusty_red) = Color::from_str("da2c43") {
//!     assert_eq!(Some("rusty red".to_string()), rusty_red.name);
//! }
//!
//! let true_blue = Color::from([0, 115, 207]);
//! assert_eq!("0073cf".to_string(), true_blue.hex);
mod color;
mod color_names;
mod error;

pub use color::Color;
pub use color_names::*;
