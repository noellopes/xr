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

use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

pub struct Output {
    pub stdout: StandardStream,
}

impl Output {
    pub fn new() -> Output {
        Output {
            stdout: StandardStream::stdout(termcolor::ColorChoice::Auto),
        }
    }

    fn set_color_spec(&mut self, color_spec: &ColorSpec) {
        self.stdout.set_color(color_spec).ok();
    }

    fn set_color(&mut self, color: Color) {
        self.set_color_spec(ColorSpec::new().set_fg(Some(color)));
    }

    pub fn set_default_color(&mut self) {
        self.set_color_spec(&termcolor::ColorSpec::new());
    }

    pub fn set_success_color(&mut self) {
        self.set_color(Color::Green);
    }
}
