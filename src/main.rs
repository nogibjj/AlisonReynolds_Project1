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
        len: i32,
        range: f64,
        noisemax: f64,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Generate {
            len,
            range,
            noisemax,
        }) => {
            let (_x, _y) = clt::data(len, range, noisemax);
            let r2 = clt::r_squared(&_x, &_y);
            println!("{}", r2);
        }
        None => println!("Missing function parameter"),
    }
}
