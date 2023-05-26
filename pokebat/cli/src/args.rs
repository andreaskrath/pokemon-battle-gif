use clap::Parser;

#[derive(Parser)]
#[command(
    author = "Andreas Krath (https://github.com/andreaskrath)",
    version = "0.0.1",
    about = "A program that performs a pokemon typed grid battle, visualized in an output gif.",
    arg_required_else_help(true),
)]
pub struct Cli {
    /// The amount of frames generated for the resulting gif file.
    #[clap(value_name = "frame amount")]
    frame_amount: u32,

    /// The width of the resulting gif file.
    #[arg(short = 'w', long, default_value_t = 500)]
    width: u32,

    /// The height of the resulting gif file.
    #[arg(short = 'l', long, default_value_t = 500)]
    height: u32,

    /// If specified, the pokemons prioritize fighting other pokemon of a weaker type.
    #[arg(short = 'c', long, default_value_t = false)]
    clever: bool,
}

impl Cli {
    pub fn get_args() -> Self {
        Cli::parse()
    }
}
