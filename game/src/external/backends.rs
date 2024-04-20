use crate::common::cli::{CliArgs, GIT_VERSION};
use crate::external::assets_macroquad;
use crate::external::drawer_egui_macroquad::DrawerEguiMacroquad;
use crate::external::drawer_macroquad::DrawerMacroquad;
use crate::external::main_input_macroquad::InputMacroquad as InputSource;
use juquad::input::input_macroquad::InputMacroquad;
use juquad::texture_loader::TextureLoader;
use logic::scene::introduction_scene::{IntroductionSceneState, JuquadFunctions};
use logic::scene::main_scene::MainScene;
use logic::screen::drawer_trait::DrawerTrait;
use logic::screen::Screen;
use logic::world::World;
use logic::SceneState;
use macroquad::texture::Texture2D;
use std::str::FromStr;

pub const TILESET_PATH: &'static str = "assets/image/tileset.png";

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
    let juquad_functions = JuquadFunctions {
        measure_text: macroquad::prelude::measure_text,
        draw_text: macroquad::prelude::draw_text,
        render_button: juquad::widgets::button::render_button,
    };
    Box::new(Some(SceneState::Introduction(IntroductionSceneState::new(
        drawer,
        input,
        TextureLoader::new_from_image(&[TILESET_PATH]),
        assets_macroquad::split_tileset,
        juquad_functions,
    ))))
}

pub fn drawer_factory(drawer_type: UiBackend, textures: Vec<Texture2D>) -> Box<dyn DrawerTrait> {
    match drawer_type {
        UiBackend::Macroquad => Box::new(DrawerMacroquad::new(textures)),
        UiBackend::Egui => Box::new(DrawerEguiMacroquad::new(textures)),
    }
}
