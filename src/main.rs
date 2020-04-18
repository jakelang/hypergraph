mod hypergraph;
mod json;

use clap::{crate_description, crate_name, crate_version, App, Arg};
use log::{debug, error, Level};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("VERBOSE")
                .short("v")
                .long("verbose")
                .help("Verbose debugging"),
        )
        .arg(
            Arg::with_name("NUM_ITERS")
                .short("i")
                .long("iterations")
                .help("Number of iterations")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    // Initialize logger
    let log_level = if matches.is_present("VERBOSE") {
        Level::Debug
    } else {
        Level::Warn
    };
    simple_logger::init_with_level(log_level).expect("Cannot fail");
}
