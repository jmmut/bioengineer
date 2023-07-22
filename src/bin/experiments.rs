use macroquad::color::colors::LIGHTGRAY;
use macroquad::texture::Texture2D;
use macroquad::window::{Conf, next_frame};
use bioengineer::external::assets_macroquad::load_tileset;
use bioengineer::external::drawer_macroquad::DrawerMacroquad;
use bioengineer::external::drawer_egui_macroquad::DrawerEguiMacroquad;
use bioengineer::external::input_macroquad::InputMacroquad as InputSource;
use bioengineer::screen::drawer_trait::DrawerTrait;
use bioengineer::screen::gui::set_skin;
use bioengineer::screen::input::InputSourceTrait;
use bioengineer::world::map::cell::ExtraTextures;

const DEFAULT_WINDOW_WIDTH: i32 = 1365;
const DEFAULT_WINDOW_HEIGHT: i32 = 768;
const DEFAULT_WINDOW_TITLE: &str = "Bioengineer Experiments";

#[macroquad::main(window_conf)]
async fn main() {
    let (mut drawer, mut input_source) = factory().await;

    while frame(&mut drawer, &mut input_source) {
        next_frame().await
    }

}

fn window_conf() -> Conf {
    Conf {
        // high_dpi: true,
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}

async fn factory() -> (Box<dyn DrawerTrait>, Box<InputSource>) {
    let tileset = load_tileset("assets/image/tileset.png");
    let drawer_name = std::env::args().last();
    let mut drawer = drawer_factory(drawer_name, tileset.await);
    set_skin(drawer.as_mut());
    let input_source = Box::new(InputSource::new());
    (drawer, input_source)
}

fn drawer_factory(drawer_type_name: Option<String>, textures: Vec<Texture2D>) -> Box<dyn DrawerTrait> {
    return if drawer_type_name == Some("mq".to_string()) {
        Box::new(DrawerMacroquad::new(textures))
    } else {
        Box::new(DrawerEguiMacroquad::new(textures))
    }
}

fn frame(drawer: &mut Box<dyn DrawerTrait>, input_source: &mut Box<InputSource>) -> bool {
    let input = input_source.get_input();
    drawer.clear_background(LIGHTGRAY);
    drawer.draw_transparent_texture(&ExtraTextures::ZoomedRobot, 0.0, 0.0, 5.0, 1.0);

    drawer.ui_run(&mut |drawer: &mut dyn DrawerTrait| {
        drawer.ui_button("click me");
        drawer.ui_named_group("named group", 300.0, 400.0, 100.0, 200.0, &mut |drawer: &mut dyn DrawerTrait| {
            drawer.ui_button("click button inside group");
            drawer.ui_text("this is some text after the button");
            drawer.ui_texture(ExtraTextures::ZoomedRobot.into());
        });
    });
    drawer.ui_draw();
    !input.quit
}
