use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    #[arg(default_value = ".")]
    pub name: String,

    #[arg(short, long, default_value = "")]
    pub target: String,
}
