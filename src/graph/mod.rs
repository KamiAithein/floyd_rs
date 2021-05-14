pub mod matrix;
pub mod algorithm;

use std::io::{self, prelude::*, BufReader};
use std::fs::File;


pub struct Node {
    pub id: usize,
}
pub struct Edge {
    pub id: usize,
    pub weight: f32,
    pub node_start_i: usize,
    pub node_end_i: usize,
}
pub struct Graph {
    pub v: Vec<Node>,
    pub e: Vec<Edge>,
}


fn parse_data<F,P>(fname: &str, parser: F) -> Result<Vec<P>, io::Error> 
where 
F: Fn(Vec<&str>) -> P {

    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    let mut parsed: Vec<P> = vec![];

    for line in reader.lines() {
        let line_res = line?;
        let split = line_res.split_whitespace();
        let attr: Vec<&str> = split.collect();
        
        parsed.push(parser(attr));
    }
    
    Ok(parsed)
}

pub fn parse_nodes(fname: &str) -> Result<Vec<Node>, io::Error> {
    let node_parser = |attr: Vec<&str>| -> Node {
        assert!(attr.len() >= 1, "node input does not have correct num of attributes!");
        
        Node {
            id: attr[0].parse().unwrap(),
        }
    };
    
    return parse_data(fname, node_parser);
}

pub fn parse_edges(fname :&str) -> Result<Vec<Edge>, io::Error> {
    let edge_parser = |attr: Vec<&str>| -> Edge {
        assert!(attr.len() >= 4, "edge input does not have correct num of attributes!");

        Edge {
            id:             attr[0].parse().unwrap(),
            weight:         attr[3].parse().unwrap(),
            node_start_i:   attr[1].parse().unwrap(),
            node_end_i:     attr[2].parse().unwrap(),
        }
    };

    return parse_data(fname, edge_parser);
}