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

use std::{io::Write, path::PathBuf};

mod arguments;
use arguments::Args;

mod terminal_helper;

fn main() {
    let mut output = terminal_helper::Output::new();

    let args = Args::obtain();

    writeln!(&mut output.stdout, "XR Parser").ok();
    let version = env!("CARGO_PKG_VERSION");
    writeln!(&mut output.stdout, "version {version}").ok();

    let filenames = args.files_to_process();

    for f in &filenames {
        writeln!(&mut output.stdout, "Processing file {:?}", f).ok();
        process_file(&f);
    }

    output.set_success_color();
    writeln!(&mut output.stdout, "{} file(s) processed", filenames.len()).ok();
    output.set_default_color();
}

fn process_file(path: &PathBuf) {}
