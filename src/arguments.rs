use clap::Parser;

/// Hello World! CLI
#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = false)]
pub struct Arguments {
    // See <https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html> for help.
    /// Outputs license information regarding this software and it's dependencies.
    #[arg(short, long)]
    pub license: bool,
}
