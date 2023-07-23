use crate::external::drawer_egui_macroquad::DrawerEguiMacroquad;
use crate::external::drawer_macroquad::DrawerMacroquad;
use crate::external::input_macroquad::InputMacroquad as InputSource;
use crate::screen::drawer_trait::DrawerTrait;
use macroquad::texture::Texture2D;
use std::str::FromStr;
use crate::common::cli::{CliArgs, GIT_VERSION};
use crate::external::assets_macroquad::load_tileset;
use crate::screen::Screen;
use crate::world::World;

#[derive(Debug, Copy, Clone)]
pub enum UiBackend {
    Macroquad,
    Egui,
}

impl FromStr for UiBackend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return if s == "mq" || s == "macroquad" {
            Ok(UiBackend::Macroquad)
        } else if s == "egui" {
            Ok(UiBackend::Egui)
        } else {
            Err(format!("error: unknown UiBackend {s}"))
        }
    }
}

pub async fn factory(args: &CliArgs) -> (Screen, World) {
    println!("Running Bioengineer version {}", GIT_VERSION);
    let tileset = load_tileset("assets/image/tileset.png");
    let drawer = drawer_factory(args.ui, tileset.await);
    let input_source = Box::new(InputSource::new());
    let world = World::new_with_options(args.profile);
    (Screen::new(drawer, input_source), world)
}

pub fn drawer_factory(drawer_type: UiBackend, textures: Vec<Texture2D>) -> Box<dyn DrawerTrait> {
    match drawer_type {
        UiBackend::Macroquad => Box::new(DrawerMacroquad::new(textures)),
        UiBackend::Egui => Box::new(DrawerEguiMacroquad::new(textures)),
    }
}

