use clap::Parser;
use std::io::Write;
use std::path::PathBuf;

mod terminal_helper;

#[derive(Parser, Debug)]
#[command(author, version, about = "XR parser", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./")]
    directory: PathBuf,

    #[arg(short, long)]
    filenames: Vec<PathBuf>,
}

fn main() {
    let mut output = terminal_helper::Output::new();

    output.set_success_color();
    writeln!(&mut output.stdout, "XR Parser").ok();

    output.set_default_color();
    let version = env!("CARGO_PKG_VERSION");
    writeln!(&mut output.stdout, "version {version}").ok();

    let args = Args::parse();
    println!("{:?}", args)
}
