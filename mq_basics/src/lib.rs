
//! These reimports are all "safe" imports we can use in the logic crate
//! without including the singleton context when we compile the dynamic library liblogic.so.
//! The goal is that if the logic crate imports this crate, logic doesn't need to import macroquad.
//! A known exception is Texture2D which has some methods that use the macroquad context, but we
//! need to use this struct in the logic crate. We'll have to be careful not to call those methods
//! in the logic crate.

pub use macroquad::miniquad::Texture;
pub use macroquad::prelude::{Color, Image, Texture2D, Vec2, IVec2, Vec3, Rect, IVec3, KeyCode, FilterMode, load_image, MouseButton};
pub mod color {
    pub use macroquad::color::*;
}

pub type Seconds = f64;

pub fn now() -> Seconds {
    macroquad::miniquad::date::now()
}
