use crate::external::backends::UiBackend;
use clap::Parser;
use git_version::git_version;

pub const GIT_VERSION: &str = git_version!(args = ["--tags"]);

#[derive(Parser, Debug)]
#[clap(version = GIT_VERSION)]
pub struct CliArgs {
    #[clap(long, help = "Measure and print profiling information.")]
    pub profile: bool,

    #[clap(
        long,
        help = "Enable fluid simulation. Game will have worse performance."
    )]
    pub fluids: bool,

    #[clap(
        long,
        help = "Choose UI backend, egui or macroquad.",
        default_value = "egui"
    )]
    pub ui: UiBackend,
}
