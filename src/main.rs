use clap::Parser;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod terminal_helper;

#[derive(Parser, Debug)]
#[command(author, version, about = "XR parser", long_about = None)]
struct Args {
    #[arg(short, long)]
    directory: Option<PathBuf>,

    #[arg(short, long)]
    filenames: Vec<PathBuf>,
}

impl Args {
    fn working_dir(self) -> PathBuf {
        if let Some(dir) = self.directory {
            dir
        } else {
            std::env::current_dir().unwrap_or(PathBuf::from("."))
        }
    }
}

fn main() {
    let mut output = terminal_helper::Output::new();

    let args = Args::parse();

    writeln!(&mut output.stdout, "XR Parser").ok();
    let version = env!("CARGO_PKG_VERSION");
    writeln!(&mut output.stdout, "version {version}").ok();

    let mut files_processed = 0;

    for entry in WalkDir::new(args.working_dir())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let filename = entry.file_name();
        if filename.to_string_lossy().to_lowercase().ends_with(".xr") {
            process_file(entry.path());
            files_processed += 1
        }
    }

    output.set_success_color();
    println!("{files_processed} files processed");
    output.set_default_color();
}

fn process_file(path: &Path) {
    println!("Processing file {:?}", path);
}
