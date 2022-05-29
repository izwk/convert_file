use clap::{Parser, Subcommand};

/// Simple program to convert files
#[derive(Parser, Debug)]
//#[clap(author, version, about, long_about = None)]
#[clap(author, version = "0.1.0", about = "Simple file converter", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Img {
        src: Option<String>,
        dest: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Img { src, dest } => {
            convert_file::convert_img(src.as_ref().unwrap(), dest.as_ref().unwrap());
        }
    }
}
