use crate::common::cli::{CliArgs, GIT_VERSION};
use crate::external::drawer_egui_macroquad::DrawerEguiMacroquad;
use crate::external::drawer_macroquad::DrawerMacroquad;
use crate::external::input_macroquad::InputMacroquad;
use crate::external::main_input_macroquad::InputMacroquad as InputSource;
use crate::scene::introduction_scene::IntroductionSceneState;
use crate::scene::main_scene::MainScene;
use crate::screen::drawer_trait::DrawerTrait;
use crate::screen::Screen;
use crate::world::World;
use crate::SceneState;
use macroquad::texture::Texture2D;
use std::str::FromStr;
use juquad::texture_loader::TextureLoader;

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
        };
    }
}

pub async fn factory(args: &CliArgs, textures: Vec<Texture2D>) -> Box<Option<SceneState>> {
    println!("Running Bioengineer version {}", GIT_VERSION);
    let drawer = drawer_factory(args.ui, textures);
    let input_source = Box::new(InputSource::new());
    let world = World::new_with_options(args.profile, args.fluids);
    Box::new(Some(SceneState::Main(MainScene {
        screen: Screen::new(drawer, input_source),
        world,
    })))
}
pub async fn introduction_factory(args: &CliArgs) -> Box<Option<SceneState>> {
    let drawer = drawer_factory(args.ui, Vec::new());
    let input = Box::new(InputMacroquad);
    Box::new(Some(SceneState::Introduction(IntroductionSceneState::new(
        drawer,
        input,
        TextureLoader::new_from_image(&["assets/image/tileset.png"]),
    ))))
}

pub fn drawer_factory(drawer_type: UiBackend, textures: Vec<Texture2D>) -> Box<dyn DrawerTrait> {
    match drawer_type {
        UiBackend::Macroquad => Box::new(DrawerMacroquad::new(textures)),
        UiBackend::Egui => Box::new(DrawerEguiMacroquad::new(textures)),
    }
}
