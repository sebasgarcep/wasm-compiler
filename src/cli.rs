use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLIArguments {
    #[arg(short, long)]
    pub mode: String,

    #[arg(short, long)]
    pub input: String,

    #[arg(short, long)]
    pub output: String,
}
