use super::matrix::Matrix;
use super::{Node, Edge, Graph};

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
            let dist_source_to_k = *dist.at(source, k);
            if dist_source_to_k == f32::INFINITY { continue; }

            for dest in 0..v.len() {
                let dist_k_to_dest = *dist.at(k, dest); 
                if dist_k_to_dest == f32::INFINITY { continue };

                let dist_source_to_dest = *dist.at(source, dest);
                let k_dist = dist_source_to_k + dist_k_to_dest;
                if dist_source_to_dest != f32::INFINITY && 
                   dist_source_to_dest <= k_dist { continue; }
                    
                *dist.at(source, dest) = k_dist; 
                *next.at(source, dest) = *next.at(source, k);
            }
        }
    }
    (dist, next)
}
pub fn shortest_path_with<'a>(start: &'a Node, end: &'a Node, graph: &'a Graph, next: &Matrix<usize>) -> Vec<&'a Node> {
    
    let mut path: Vec<&'a Node> = vec![];
    
    if *next.at_read(start.id, end.id) == usize::MAX {
        return path;
    }

    path.push(start);

    let mut current: &'a Node = start;
    while current.id != end.id {
        current = &graph.v[*next.at_read(current.id, end.id)];
        path.push(current);
    }

    return path;

}
pub fn shortest_path<'a>(start: &'a Node, end: &'a Node, graph: &'a Graph) -> Vec<&'a Node> {
    let (_, next) = floyd_warshall(&graph.v, &graph.e);
    shortest_path_with(&start, &end, &graph, &next)
}