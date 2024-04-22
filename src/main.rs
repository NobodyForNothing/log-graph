use crate::graph::draw;
use clap::Parser;

mod data;
mod graph;
mod parser;

/// Primitive tool for creating graphs from simple log files
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the input file
    input_file: String,

    /// Path of the output image
    output_png: String,

    /// Whether to generate a graph of the rate of change of the values
    #[arg(short, long)]
    rate: bool,

    /// Width of the line in the graph
    #[arg(short, long, default_value_t = 8.)]
    width: f32,
}

fn main() {
    let args = Args::parse();

    let mut data = parser::parse_file(args.input_file).expect("Failed to parse input file.");
    if args.rate {
        data = data.rate();
    }
    draw(data, args.output_png, args.width).expect("Failed to draw graph.");

    //println!("{:?}", &f.unwrap().len());
    //println!("{:?}", &f.unwrap().values().collect::<Vec<_>>().first().unwrap().len());
}
