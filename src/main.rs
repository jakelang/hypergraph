mod hypergraph;
mod json;

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use clap::{crate_description, crate_name, crate_version, App, Arg};
use log::{debug, error, info, Level};
use serde_json::to_string_pretty;

use hypergraph::{DirectedGraph, DirectedHyperGraph};
use json::to_sigma_json;

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
                //.required(true)
                .takes_value(true),
        )
        .get_matches();

    // Initialize logger
    let log_level = if matches.is_present("VERBOSE") {
        Level::Debug
    } else {
        Level::Info
    };
    simple_logger::init_with_level(log_level).expect("Cannot fail");

    let hypergraph = DirectedHyperGraph::ternary_self_loop();
    let unrolled = hypergraph.unroll_to_graph();
    info!("Unrolled hypergraph");

    let json = to_string_pretty(&to_sigma_json(unrolled)).expect("Cannot fail");
    info!("Serialized graph data");

    let mut file = if !Path::new("./data.json").exists() {
        File::create("./data.json").expect("Fuck")
    } else {
        OpenOptions::new().write(true).open("./data.json" ).expect("Fuck")
    };

    file.write_all(&json.as_bytes());
    info!("Wrote output to 'data.json'");
}
