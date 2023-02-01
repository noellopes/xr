/*
    Copyright 2023 Noel Lopes

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the "Software"),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
*/

use clap::Parser;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about = "XR parser", long_about = None)]
pub struct Args {
    #[arg(short, long, group = "files")]
    directory: Option<PathBuf>,

    #[arg(short, long, group = "files")]
    filenames: Option<Vec<PathBuf>>,
}

impl Args {
    pub fn obtain() -> Args {
        Args::parse()
    }

    fn working_dir(self) -> PathBuf {
        match self.directory {
            Some(dir) => dir,
            None => std::env::current_dir().unwrap_or(PathBuf::from(".")),
        }
    }

    pub fn files_to_process(self) -> Vec<PathBuf> {
        if let Some(filenames) = self.filenames {
            filenames
        } else {
            let mut filenames = Vec::<PathBuf>::new();

            for entry in WalkDir::new(self.working_dir())
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let filename = entry.path();

                if let Some(extension) = filename.extension() {
                    if extension.to_ascii_lowercase() == "xr" {
                        filenames.push(filename.to_path_buf());
                    }
                }
            }
            filenames
        }
    }
}
