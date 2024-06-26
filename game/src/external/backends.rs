use crate::common::cli::{CliArgs, UiBackend, GIT_VERSION};
use crate::external::assets_macroquad;
use crate::external::drawer_egui_macroquad::DrawerEguiMacroquad;
use crate::external::drawer_macroquad::DrawerMacroquad;
use juquad::input::input_macroquad::InputMacroquad;
use juquad::texture_loader::TextureLoader;
use logic::scene::introduction_scene::{IntroductionScene, JuquadFunctions};
use logic::scene::main_scene::MainScene;
use logic::screen::drawer_trait::DrawerTrait;
use logic::screen::main_scene_input_source::MainSceneInputSource;
use logic::screen::Screen;
use logic::world::map::MapType;
use logic::world::World;
use logic::SceneState;
use macroquad::texture::Texture2D;

pub const TILESET_PATH: &'static str = "assets/image/tileset.png";

pub async fn create_main_scene(args: &CliArgs, textures: Vec<Texture2D>) -> Box<SceneState> {
    println!("Running Bioengineer version {}", GIT_VERSION);
    let drawer = drawer_factory(args.ui, textures);
    let input_source = MainSceneInputSource::new(Box::new(InputMacroquad));
    let world = World::new_with_options(args.profile, args.fluids, MapType::Simplex);
    Box::new(SceneState::Main(MainScene {
        screen: Screen::new(
            drawer,
            input_source,
            world.map.get_ship_position().unwrap_or_default(),
        ),
        world,
    }))
}

pub async fn create_introduction_scene(args: &CliArgs) -> Box<SceneState> {
    let drawer = drawer_factory(args.ui, Vec::new());
    let input = Box::new(InputMacroquad);
    let juquad_functions = JuquadFunctions {
        measure_text: macroquad::prelude::measure_text,
        draw_text: macroquad::prelude::draw_text,
        render_button: juquad::widgets::button::render_button,
    };
    Box::new(SceneState::Introduction(IntroductionScene::new(
        drawer,
        input,
        TextureLoader::new_from_image(&[TILESET_PATH]),
        assets_macroquad::split_tileset,
        juquad_functions,
    )))
}

pub fn drawer_factory(drawer_type: UiBackend, textures: Vec<Texture2D>) -> Box<dyn DrawerTrait> {
    match drawer_type {
        UiBackend::Macroquad => Box::new(DrawerMacroquad::new(textures)),
        UiBackend::Egui => Box::new(DrawerEguiMacroquad::new(textures)),
    }
}
