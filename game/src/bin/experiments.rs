use bioengineer::external::assets_macroquad::load_tileset;
use bioengineer::external::backends::{drawer_factory, UiBackend};
use bioengineer::external::main_input_macroquad::InputMacroquad as InputSource;
use logic::screen::drawer_trait::DrawerTrait;
use logic::screen::gui::set_skin;
use logic::screen::main_scene_input::MainSceneInputTrait;
use macroquad::color;
use macroquad::color::colors::LIGHTGRAY;
use macroquad::texture::Texture2D;
use macroquad::window::{next_frame, Conf};
use std::str::FromStr;

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
    let mut drawer = drawer_factory_from_name(drawer_name, tileset.await);
    set_skin(drawer.as_mut());
    let input_source = Box::new(InputSource::new());
    (drawer, input_source)
}

fn drawer_factory_from_name(
    drawer_type_name: Option<String>,
    textures: Vec<Texture2D>,
) -> Box<dyn DrawerTrait> {
    let drawer_type_res = UiBackend::from_str(&drawer_type_name.unwrap_or("egui".to_string()));
    let drawer_type = match drawer_type_res {
        Ok(t) => t,
        Err(_) => UiBackend::Egui,
    };
    drawer_factory(drawer_type, textures)
}

static mut WIDTH: f32 = 300.0;

fn frame(drawer: &mut Box<dyn DrawerTrait>, input_source: &mut Box<InputSource>) -> bool {
    let input = input_source.get_input();
    drawer.clear_background(LIGHTGRAY);
    // drawer.draw_transparent_texture(&ExtraTextures::ZoomedRobot, 0.0, 0.0, 5.0, 1.0);
    drawer.draw_rectangle(600.0, 100.0, unsafe { WIDTH }, 200.0, color::ORANGE);
    drawer.ui_run(&mut |drawer: &mut dyn DrawerTrait| {
        drawer.ui_named_group(
            "test buttons",
            800.0,
            50.0,
            unsafe { WIDTH },
            200.0,
            &mut |drawer: &mut dyn DrawerTrait| {
                drawer.ui_button("click me");
                if drawer.ui_button("increase width +10").is_clicked() {
                    unsafe {
                        WIDTH += 10.0;
                    }
                }
                if drawer.ui_button("decrease width -10").is_clicked() {
                    unsafe {
                        WIDTH -= 10.0;
                    }
                }
                drawer.ui_text(&format!("current width: {}", unsafe { WIDTH }));
            },
        );
        // drawer.ui_named_group(
        //     "named group",
        //     300.0,
        //     400.0,
        //     100.0,
        //     200.0,
        //     &mut |drawer: &mut dyn DrawerTrait| {
        //         drawer.ui_button("click button inside group");
        //         drawer.ui_text("this is some text after the button");
        //         drawer.ui_texture(ExtraTextures::ZoomedRobot.into());
        //     },
        // );

        drawer.ui_named_group(
            "window",
            600.0,
            400.0,
            unsafe { WIDTH },
            200.0,
            &mut |drawer: &mut dyn DrawerTrait| {
                drawer.ui_text("size:");
                drawer.ui_text("w=300");
                drawer.ui_text("h=200");
            },
        );
        drawer.debug_ui();
    });
    drawer.ui_draw();
    !input.quit
}
