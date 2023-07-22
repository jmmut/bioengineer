use std::str::FromStr;
use macroquad::texture::Texture2D;
use crate::external::drawer_egui_macroquad::DrawerEguiMacroquad;
use crate::external::drawer_macroquad::DrawerMacroquad;
use crate::screen::drawer_trait::DrawerTrait;

#[derive(Debug)]
pub enum UiBackend {
    Macroquad,
    Egui,
}

impl FromStr for UiBackend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "mq" || s == "macroquad" {
            return Ok(UiBackend::Macroquad);
        } else if s == "egui" {
            return Ok(UiBackend::Egui);
        } else {
            return Err(format!("error: unknown UiBackend {s}"));
        }
    }
}


pub fn drawer_factory(drawer_type: UiBackend, textures: Vec<Texture2D>) -> Box<dyn DrawerTrait> {
    match drawer_type {
        UiBackend::Macroquad => Box::new(DrawerMacroquad::new(textures)),
        UiBackend::Egui => Box::new(DrawerEguiMacroquad::new(textures)),
    }
}
