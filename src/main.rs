use clap::Parser;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod terminal_helper;

#[derive(Parser, Debug)]
#[command(author, version, about = "XR parser", long_about = None)]
struct Args {
    #[arg(short, long, group = "files")]
    directory: Option<PathBuf>,

    #[arg(short, long, group = "files")]
    filenames: Option<Vec<PathBuf>>,
}

impl Args {
    fn working_dir(self) -> PathBuf {
        if let Some(dir) = self.directory {
            dir
        } else {
            std::env::current_dir().unwrap_or(PathBuf::from("."))
        }
    }

    fn files_to_process(self) -> Vec<PathBuf> {
        if let Some(filenames) = self.filenames {
            filenames
        } else {
            let mut filenames = Vec::<PathBuf>::new();

            for entry in WalkDir::new(self.working_dir())
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let filename = entry.file_name();
                if filename.to_string_lossy().to_lowercase().ends_with(".xr") {
                    filenames.push(entry.path().to_path_buf());
                }
            }
            filenames
        }
    }
}

fn main() {
    let mut output = terminal_helper::Output::new();

    let args = Args::parse();

    writeln!(&mut output.stdout, "XR Parser").ok();
    let version = env!("CARGO_PKG_VERSION");
    writeln!(&mut output.stdout, "version {version}").ok();

    let filenames = args.files_to_process();

    for f in &filenames {
        process_file(&f);
    }

    output.set_success_color();
    println!("{} file(s) processed", filenames.len());
    output.set_default_color();
}

fn process_file(path: &PathBuf) {
    println!("Processing file {:?}", path);
}
