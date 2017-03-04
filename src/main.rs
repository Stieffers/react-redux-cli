extern crate argparse;
extern crate ansi_term;

use argparse::{ArgumentParser, StoreTrue, Store, Print};

mod ctrl;

fn main() {
    let mut verbose = false;
    let mut name = "".to_string();
    let mut component_type = "component".to_string();
    let mut dir = "".to_string();
    let mut extension = "jsx".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("React / Redux Command Line Tools: Create new React components from templates");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Verbose output");
        ap.add_option(&["-V", "--version"], Print(env!("CARGO_PKG_VERSION").to_string()), "Show version");
        ap.refer(&mut name)
            .add_argument("name", Store, "React component name / base name").required();
        ap.refer(&mut component_type)
            .add_option(&["-t", "--type"], Store, "React component type: component (stateless), container (stateful), handler (reducer, action creator, action handler, redux connected container)");
        ap.refer(&mut dir)
            .add_option(&["-d", "--dir"], Store, "Directory to create component in");
        ap.refer(&mut extension)
            .add_option(&["-e", "--extension"], Store, "Extension of files to create -- exclude period (default 'jsx')");
        ap.parse_args_or_exit();
    }

    if !name.eq("") {
        let mut generator = ctrl::generator::Generator::new(name, verbose, dir, extension);
        generator.run(component_type);
    }
}
