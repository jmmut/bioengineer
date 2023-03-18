
use clap::Parser;
use git_version::git_version;
use macroquad::window::Conf;
use macroquad::window::next_frame;

use bioengineer::external::drawer_macroquad::DrawerMacroquad as DrawerImpl;
use bioengineer::external::input_macroquad::InputMacroquad as InputSource;
use bioengineer::external::assets_macroquad::load_tileset;
use bioengineer::frame;
use bioengineer::screen::drawer_trait::DrawerTrait;
use bioengineer::screen::Screen;
use bioengineer::world::map::chunk::chunks::cache::print_cache_stats;
use bioengineer::world::World;

const DEFAULT_WINDOW_WIDTH: i32 = 1365;
const DEFAULT_WINDOW_HEIGHT: i32 = 768;
const DEFAULT_WINDOW_TITLE: &str = "Bioengineer";

const GIT_VERSION: &str = git_version!(args = ["--tags"]);

#[derive(Parser, Debug)]
#[clap(version = GIT_VERSION)]
struct CliArgs {
    #[clap(long, help = "Measure and print profiling information.")]
    profile: bool,

    #[clap(
        long,
        help = "Enable fluid simulation. Game will have worse performance."
    )]
    fluids: bool,
}

// #[macroquad::main(window_conf)]
async fn old_main() {
    let (mut screen, mut world) = factory().await;

    while frame(&mut screen, &mut world) {
        next_frame().await
    }
    print_cache_stats(world.game_state.profile);
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        ..Default::default()
    }
}

async fn factory() -> (Screen, World) {
    let args = CliArgs::parse();
    println!("Running Bioengineer version {}", GIT_VERSION);
    let tileset = load_tileset("assets/image/tileset.png");
    let drawer = Box::new(DrawerImpl::new(tileset.await));
    let input_source = Box::new(InputSource::new());
    let world = World::new_with_options(args.profile);
    (Screen::new(drawer, input_source), world)
}

use macroquad::miniquad;

struct Stage {
    egui_mq: egui_miniquad::EguiMq,
    show_egui_demo_windows: bool,
    egui_demo_windows: egui_demo_lib::DemoWindows,
    color_test: egui_demo_lib::ColorTest,
    pixels_per_point: f32,
}

impl Stage {
    fn new(ctx: &mut miniquad::Context) -> Self {
        Self {
            egui_mq: egui_miniquad::EguiMq::new(ctx),
            show_egui_demo_windows: true,
            egui_demo_windows: Default::default(),
            color_test: Default::default(),
            pixels_per_point: ctx.dpi_scale(),
        }
    }
}

impl miniquad::EventHandler for Stage {
    fn update(&mut self, _ctx: &mut miniquad::Context) {}

    fn draw(&mut self, mq_ctx: &mut miniquad::Context) {
        mq_ctx.clear(Some((1., 1., 1., 1.)), None, None);
        mq_ctx.begin_default_pass(miniquad::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        mq_ctx.end_render_pass();

        let dpi_scale = mq_ctx.dpi_scale();

        // Run the UI code:
        self.egui_mq.run(mq_ctx, |_mq_ctx, egui_ctx| {
            if self.show_egui_demo_windows {
                self.egui_demo_windows.ui(egui_ctx);
            }

            egui::Window::new("egui ❤ miniquad").show(egui_ctx, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.checkbox(&mut self.show_egui_demo_windows, "Show egui demo windows");

                ui.group(|ui| {
                    ui.label("Physical pixels per each logical 'point':");
                    ui.label(format!("native: {:.2}", dpi_scale));
                    ui.label(format!("egui:   {:.2}", ui.ctx().pixels_per_point()));
                    ui.add(
                        egui::Slider::new(&mut self.pixels_per_point, 0.75..=3.0).logarithmic(true),
                    )
                    .on_hover_text("Physical pixels per logical point");
                    if ui.button("Reset").clicked() {
                        self.pixels_per_point = dpi_scale;
                    }
                });

                #[cfg(not(target_arch = "wasm32"))]
                {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                }
            });

            // Don't change scale while dragging the slider
            if !egui_ctx.is_using_pointer() {
                egui_ctx.set_pixels_per_point(self.pixels_per_point);
            }

            egui::Window::new("Color Test").show(egui_ctx, |ui| {
                egui::ScrollArea::both()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.color_test.ui(ui);
                    });
            });
        });

        // Draw things behind egui here

        self.egui_mq.draw(mq_ctx);

        // Draw things in front of egui here

        mq_ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, _: &mut miniquad::Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, _: &mut miniquad::Context, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut miniquad::Context,
        mb: miniquad::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut miniquad::Context,
        mb: miniquad::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        _ctx: &mut miniquad::Context,
        character: char,
        _keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut miniquad::Context,
        keycode: miniquad::KeyCode,
        keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut miniquad::Context, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Log to stdout (if you run with `RUST_LOG=debug`).
        // tracing_subscriber::fmt::init();
    }

    let conf = miniquad::conf::Conf {
        high_dpi: true,
        window_width: 1200,
        window_height: 1024,
        ..Default::default()
    };
    miniquad::start(conf, |mut ctx| Box::new(Stage::new(&mut ctx)));
}
