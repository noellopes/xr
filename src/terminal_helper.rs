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
