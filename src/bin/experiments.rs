use macroquad::color::colors::LIGHTGRAY;
use macroquad::window::{Conf, next_frame};
use bioengineer::external::assets_macroquad::load_tileset;
use bioengineer::external::drawer_egui_macroquad::DrawerEguiMacroquad as DrawerImpl;
use bioengineer::external::input_macroquad::InputMacroquad as InputSource;
use bioengineer::screen::drawer_trait::DrawerTrait;
use bioengineer::screen::input::InputSourceTrait;
use bioengineer::screen::Screen;
use bioengineer::world::map::cell::ExtraTextures;
use bioengineer::world::World;

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

async fn factory() -> (Box<DrawerImpl>, Box<InputSource>) {
    let tileset = load_tileset("assets/image/tileset.png");
    let drawer = Box::new(DrawerImpl::new(tileset.await));
    let input_source = Box::new(InputSource::new());
    (drawer, input_source)
}

fn frame(drawer: &mut Box<DrawerImpl>, input_source: &mut Box<InputSource>) -> bool {
    let input = input_source.get_input();
    drawer.clear_background(LIGHTGRAY);
    drawer.draw_transparent_texture(&ExtraTextures::Robot, 0.0, 0.0, 5.0, 1.0);

    // drawer.ui_run(&mut |drawer: &dyn DrawerTrait| {
    //     drawer.ui_button("click me");
    // });
    // drawer.ui_draw();
    !input.quit
}
