//A command-line tool to generate fake data
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "0.1.0",
    author = "Alison",
    about = "A fake data generator and plotter"
)]
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
        #[clap(short, long)]
        range: f64,
        #[clap(short, long)]
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
            let (x, y) = clt::data(len, range, noisemax);
            let corr = clt::correlation(&x, &y);
            clt::plot(x, y, range, noisemax);
            println!("{}", corr);
        }
        None => println!("Missing function parameter"),
    }
}
