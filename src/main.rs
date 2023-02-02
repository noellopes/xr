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

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

mod arguments;
use arguments::Args;

mod terminal_helper;
use terminal_helper::TerminalOutput;

fn main() {
    let mut output = TerminalOutput::new();

    let args = Args::obtain();

    output.writeln("XR Parser");
    let version = env!("CARGO_PKG_VERSION");
    output.writeln(format!("version {version}"));

    let filenames = args.files_to_process();

    for f in &filenames {
        process_file(f, &mut output);
    }

    output.writeln_success(format!("{} file(s) processed", filenames.len()));
}

fn process_file(file: &PathBuf, output: &mut TerminalOutput) {
    let filename = file.to_str().unwrap_or_default();

    output.writeln(format!("Processing file '{filename}'"));

    match fs::read_to_string(file) {
        Ok(contents) => generate_file(file, contents, output),
        Err(_) => output.writeln_error(format!("Could not read file '{filename}'")),
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Token {
    Space,
    Other,
    None,
}

struct Sequence<'a> {
    token: Token,
    text: &'a str,
}

fn generate_file(original_file: &PathBuf, contents: String, output: &mut TerminalOutput) {
    let mut new_file = original_file.clone();

    if !new_file.set_extension("rs") {
        output.writeln_error("Failed to generate output file");
    } else {
        let filename = new_file.to_str().unwrap_or_default();

        let result = parse_spaces(&contents);

        if let Ok(mut file) = File::create(&new_file) {
            for t in result {
                match t.token {
                    Token::Space => {
                        if file.write_all(format!("[{}]", t.text).as_bytes()).is_err() {
                            output.writeln_error(format!("Failed to write to file '{filename}'"));
                            return;
                        }
                    }
                    Token::Other => {
                        if file.write_all(t.text.as_bytes()).is_err() {
                            output.writeln_error(format!("Failed to write to file '{filename}'"));
                            return;
                        }
                    }
                    Token::None => (),
                }
            }
        } else {
            output.writeln_error(format!("Failed to create file '{filename}'"));
        }
    }
}

fn parse_spaces(text: &str) -> Vec<Sequence> {
    let mut result = Vec::<Sequence>::new();

    //let mut previous_sequence: Option<Sequence> = None;
    let mut last_token = Token::None;
    let mut start_index: usize = 0;
    let mut end_index: usize = 0;

    for (i, c) in text.char_indices() {
        let token = if c.is_whitespace() {
            Token::Space
        } else {
            Token::Other
        };

        if token == last_token {
            end_index = i;
        } else {
            if token != Token::None {
                result.push(Sequence {
                    token: last_token,
                    text: &text[start_index..end_index + 1],
                });
            }

            last_token = token;
            start_index = i;
            end_index = i;
        }
    }

    if last_token != Token::None {
        result.push(Sequence {
            token: last_token,
            text: &text[start_index..end_index + 1],
        });
    }

    result
}
