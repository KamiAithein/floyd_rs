use super::matrix::Matrix;
use super::{Node, Edge};

pub fn floyd_warshall(v: &Vec<Node>, e: &Vec<Edge>) -> (Matrix<f32>, Matrix<usize>) {

    let mut dist = Matrix::new_with(v.len(), f32::INFINITY);
    let mut next = Matrix::new_with(v.len(), usize::MAX);
        
    for diag in 0..v.len() {
        *dist.at(diag, diag) = 0.;
        *next.at(diag, diag) = v[diag].id;
    }

    for edge in e {
        //ugraph, both ways
        *dist.at(edge.node_start_i, edge.node_end_i) = edge.weight;
        *next.at(edge.node_start_i, edge.node_end_i) = v[edge.node_end_i].id;

        *dist.at(edge.node_end_i, edge.node_start_i) = edge.weight;
        *next.at(edge.node_end_i, edge.node_start_i) = v[edge.node_start_i].id;
    }

    for k in 0..v.len() {
        println!("{}% complete", (k as f32)/(v.len() as f32));
        for source in 0..v.len() {
            if *dist.at(source, k) == f32::INFINITY {
                continue;
            }
            for dest in 0..v.len() {
                if *dist.at(k, dest) != f32::INFINITY {
                    if *dist.at(source, dest) == f32::INFINITY || 
                       *dist.at(source, dest) > *dist.at(source, k) + *dist.at(k, dest) {
                        *dist.at(source, dest) = *dist.at(source, k) + *dist.at(k, dest);
                        *next.at(source, dest) = *next.at(source, k);
                    }
                }
            }
        }
    }

    (dist, next)
}