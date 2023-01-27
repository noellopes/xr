use std::io::Write;

mod terminal_helper;

fn main() {
    const VERSION: &'static str = "0.0.1";

    let mut output = terminal_helper::Output::new();

    output.set_success_color();
    writeln!(&mut output.stdout, "XR Parser").ok();

    output.set_default_color();
    writeln!(&mut output.stdout, "version {VERSION}").ok();
}
