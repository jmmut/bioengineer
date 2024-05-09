use clap::Parser;
use git_version::git_version;
use std::str::FromStr;

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
        default_value = "macroquad"
    )]
    pub ui: UiBackend,
}

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
