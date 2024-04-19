use crate::graph::draw;

mod parser;
mod graph;


fn main() {
    let data = parser::parse_file("results.txt");
    let _ = draw(data.unwrap());

    //println!("{:?}", &f.unwrap().len());
    //println!("{:?}", &f.unwrap().values().collect::<Vec<_>>().first().unwrap().len());
}
