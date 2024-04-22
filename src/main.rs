use clap::Parser;
use crate::graph::draw;

mod parser;
mod graph;

/// Primitive tool for creating graphs from simple log files
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the input file
    input_file: String,

    /// Path of the output image
    output_png: String,
}

fn main() {
    let args = Args::parse();

    let data = parser::parse_file(args.input_file)
        .expect("Failed to parse input file.");
    draw(data, args.output_png)
        .expect("Failed to draw graph.");

    //println!("{:?}", &f.unwrap().len());
    //println!("{:?}", &f.unwrap().values().collect::<Vec<_>>().first().unwrap().len());
}
