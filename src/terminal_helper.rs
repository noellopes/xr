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

use std::{fmt::Display, io::Write};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

pub struct TerminalOutput {
    stdout: StandardStream,
    stderr: StandardStream,
}

#[inline]
fn set_color(stream: &mut StandardStream, color_spec: &ColorSpec) {
    stream.set_color(color_spec).ok();
}

#[inline]
fn default_color_spec() -> ColorSpec {
    ColorSpec::new()
}

#[inline]
fn bold_color_spec() -> ColorSpec {
    let mut color = ColorSpec::new();
    color.set_bold(true);
    color
}

#[inline]
fn colored_bold_color_spec(c: Color) -> ColorSpec {
    let mut color = bold_color_spec();
    color.set_fg(Some(c));
    color
}

#[inline]
fn error_color_spec() -> ColorSpec {
    colored_bold_color_spec(Color::Red)
}

#[inline]
fn success_color_spec() -> ColorSpec {
    colored_bold_color_spec(Color::Green)
}

impl TerminalOutput {
    #[inline]
    pub fn new() -> TerminalOutput {
        TerminalOutput {
            stdout: StandardStream::stdout(termcolor::ColorChoice::Auto),
            stderr: StandardStream::stderr(termcolor::ColorChoice::Auto),
        }
    }

    #[inline]
    pub fn writeln_success<T: Display>(&mut self, text: T) {
        set_color(&mut self.stdout, &success_color_spec());
        write!(&mut self.stdout, "{text}").ok();
        set_color(&mut self.stdout, &default_color_spec());
    }

    #[inline]
    pub fn writeln_error<T: Display>(&mut self, text: T) {
        set_color(&mut self.stderr, &error_color_spec());
        write!(&mut self.stderr, "Error: ").ok();
        set_color(&mut self.stderr, &default_color_spec());
        writeln!(&mut self.stderr, "{text}").ok();
    }

    #[inline]
    pub fn writeln<T: Display>(&mut self, text: T) {
        writeln!(&mut self.stdout, "{text}").ok();
    }
}
