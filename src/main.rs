use clap::{Parser, Subcommand};

fn convert_img(src: &str, dest: &str) {
    let img = image::open(src).unwrap();
    img.save(dest).unwrap();
}

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
            if src.is_none() || dest.is_none() {
                panic!("Oops...Something went wrong...");
            }
            convert_img(src.as_ref().unwrap(), dest.as_ref().unwrap());
        }
    }
}
