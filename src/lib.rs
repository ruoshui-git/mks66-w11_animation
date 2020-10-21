#![allow(dead_code)]

pub mod canvas;
pub mod colors;
pub mod drawer;
pub mod img;
pub mod light;
pub mod matrix;
pub mod mdl;
pub mod parametrics;
pub mod processes;
pub mod utils;
pub mod vector;

// re-exports
pub use canvas::Canvas;
pub use colors::{HSL, RGB};
pub use drawer::Drawer;
pub use img::PPMImg;
pub use matrix::Matrix;
