use std::io::Write;

mod terminal_helper;

fn main() {
    let mut output = terminal_helper::Output::new();

    output.set_success_color();
    writeln!(&mut output.stdout, "XR Parser").ok();

    output.set_default_color();
    let version = env!("CARGO_PKG_VERSION");    
    writeln!(&mut output.stdout, "version {version}").ok();
}
