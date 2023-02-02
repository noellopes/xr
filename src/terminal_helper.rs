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

impl TerminalOutput {
    pub fn new() -> TerminalOutput {
        TerminalOutput {
            stdout: StandardStream::stdout(termcolor::ColorChoice::Auto),
            stderr: StandardStream::stderr(termcolor::ColorChoice::Auto),
        }
    }

    fn set_color_spec(&mut self, color_spec: &ColorSpec) {
        self.stdout.set_color(color_spec).ok();
    }

    fn set_color(&mut self, color: Color) {
        self.set_color_spec(ColorSpec::new().set_fg(Some(color)));
    }

    fn set_default_color(&mut self) {
        self.set_color_spec(&termcolor::ColorSpec::new());
    }

    fn set_success_color(&mut self) {
        self.set_color(Color::Green);
    }

    fn set_error_color(&mut self) {
        self.set_color(Color::Red);
    }

    pub fn writeln_success<T: Display>(&mut self, text: T) {
        self.set_success_color();
        write!(&mut self.stdout, "{text}").ok();
        self.set_default_color();
    }

    pub fn writeln_error<T: Display>(&mut self, text: T) {
        self.set_error_color();
        write!(&mut self.stderr, "Error: ").ok();
        self.set_default_color();
        writeln!(&mut self.stderr, "{text}").ok();
    }

    pub fn writeln<T: Display>(&mut self, text: T) {
        writeln!(&mut self.stdout, "{text}").ok();
    }
}
