//A command-line tool to generate fake data
use clap::Parser;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Alison", about = "A fake data generator")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "0.1.0", author = "Alison")]
    Generate {
        #[clap(short, long)]
        noise: f64,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Generate { noise }) => {
            let (_x, _y, _z) = clt::data(&noise);
            println!("Data Generated");
        }
        None => println!("Missing noise value"),
    }
}
