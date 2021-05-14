mod graph;
use graph::{Graph};
use graph::matrix::{Matrix};
use graph::algorithm::{floyd_warshall, shortest_path_with, shortest_path};
use graph::{parse_edges, parse_nodes};
use std::io::{self};
use std::env;
use std::path::Path;

const USAGE: &'static str = "
arg1: <file containing edge data>
arg2: <file containing node data>
arg3: <opt file location to write/read cached next matrix from>
arg4: <opt file location to write/read cached dist matrix from>";

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 && args.len() != 3 {
        panic!(USAGE);
    }
    let fedges = &args[1];
    let fnodes = &args[2];
    let fnext = if args.len() > 3 { Some(&args[3]) } else { None };
    let fdist = if args.len() > 3 { Some(&args[4]) } else { None };

    println!("reading files!");
    let graph = Graph {
        v: parse_nodes(fnodes)?,
        e: parse_edges(fedges)?,
    };

    println!("read files!");

    let fnext_exists = match fnext {
        Some(f) => Path::new(f).exists(),
        None    => false,
    };
    let fdist_exists = match fdist {
        Some(f) => Path::new(f).exists(),
        None    => false,
    };

    let need_floyd = !fnext_exists || !fdist_exists;

    let (dist, next) =  if need_floyd  {
        println!("running floyd-warshall!");
        floyd_warshall(&graph.v, &graph.e)
    } else {
        println!("reading from file!");
        (Matrix::new_from(fdist.unwrap(), 0.)?, Matrix::new_from(fnext.unwrap(), 0)?)
        
    };

    println!("obtained matrix!");

    println!("{:?}", shortest_path_with(&graph.v[0], &graph.v[graph.v.len()-1], &graph, &next));

    if need_floyd && !fnext.is_none() && !fdist.is_none() {
        Matrix::write_to(&next, fnext.unwrap())?;
        Matrix::write_to(&dist, fdist.unwrap())?;
        println!("written matrix");
    }

    Ok(())
}
