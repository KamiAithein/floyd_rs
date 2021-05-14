mod graph;
use graph::matrix::{Matrix};
use graph::algorithm::{floyd_warshall};
use graph::{parse_edges, parse_nodes};
use std::io::{self};
use std::env;
use std::path::Path;

const USAGE: &'static str = "
arg1: <file containing edge data>
arg2: <file containing node data>
arg3: <file location to write/read cached next matrix from>
arg4: <file location to write/read cached dist matrix from>";

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        panic!(USAGE);
    }
    let fedges = &args[1];
    let fnodes = &args[2];
    let fnext = &args[3];
    let fdist = &args[4];

    println!("reading files!");
    let nodes = parse_nodes(fnodes)?;
    let edges = parse_edges(fedges)?;

    println!("read files!");

    let (dist, next) =  if Path::new(fnext).exists() && Path::new(fdist).exists() {
        println!("reading from file!");
        (Matrix::new_from(fdist, 0.)?, Matrix::new_from(fnext, 0)?)
    } else {
        println!("running floyd-warshall!");
        floyd_warshall(&nodes, &edges)
    };

    println!("obtained matrix!");

    Matrix::write_to(&next, fnext)?;
    Matrix::write_to(&dist, fdist)?;

    println!("written matrix");

    Ok(())
}
