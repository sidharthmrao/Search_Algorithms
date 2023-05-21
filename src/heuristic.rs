use crate::node::{Node};

pub trait Heuristic {
    fn distance(&self, node: &Node, other_node: &Node) -> f32;
}

pub struct ManhattanDistance {}

impl Heuristic for ManhattanDistance {
    fn distance(&self, node: &Node, target_node: &Node) -> f32 {
        let x = (node.x - target_node.x).abs();
        let y = (node.y - target_node.y).abs();
        let z = (node.z - target_node.z).abs();
        (x + y + z) as f32
    }
}

pub trait CostFunction {
    fn cost(&self, node: &Node, other_node: &Node) -> f32;
}

pub struct EuclideanCost {}

impl CostFunction for EuclideanCost {
    fn cost(&self, node: &Node, other_node: &Node) -> f32 {
        let x = (node.x - other_node.x).powf(2.0);
        let y = (node.y - other_node.y).powf(2.0);
        let z = (node.z - other_node.z).powf(2.0);
        ((x + y + z) as f32).sqrt()
    }
}

//
// pub struct EuclideanDistance {
//     target: Node,
// }
//
// impl Heuristic for EuclideanDistance {
//     fn distance(&self, other_node: Node) -> f32 {
//         let x = (self.target.x - other_node.x).powf(2.0);
//         let y = (self.target.y - other_node.y).powf(2.0);
//         let z = (self.target.z - other_node.z).powf(2.0);
//         ((x + y + z) as f32).sqrt()
//     }
// }
//
// pub struct ChebyshevDistance {
//     target: Node,
// }
//
// impl Heuristic for ChebyshevDistance {
//     fn distance(&self, other_node: Node) -> f32 {
//         let x = (self.target.x - other_node.x).abs();
//         let y = (self.target.y - other_node.y).abs();
//         let z = (self.target.z - other_node.z).abs();
//         x.max(y).max(z) as f32
//     }
// }
//
// pub struct OctileDistance {
//     target: Node,
// }
//
// impl Heuristic for OctileDistance {
//     fn distance(&self, other_node: Node) -> f32 {
//         let x = (self.target.x - other_node.x).abs();
//         let y = (self.target.y - other_node.y).abs();
//         let z = (self.target.z - other_node.z).abs();
//         let min = x.min(y).min(z);
//         let max = x.max(y).max(z);
//         ((min as f32) * ((2.0_f32.sqrt() - 1.0) as f32) + max as f32) as f32
//     }
// }
//
// pub struct DiagonalDistance {
//     target: Node,
// }
//
// impl Heuristic for DiagonalDistance {
//     fn distance(&self, other_node: Node) -> f32 {
//         let x = (self.target.x - other_node.x).abs();
//         let y = (self.target.y - other_node.y).abs();
//         let z = (self.target.z - other_node.z).abs();
//         let min = x.min(y).min(z);
//         let max = x.max(y).max(z);
//         ((min as f32) + ((3.0_f32.sqrt() - 1.0) as f32) * max as f32) as f32
//     }
// }
//
// pub struct Dijkstra {
//     target: Node,
// }
//
// impl Heuristic for Dijkstra {
//     fn distance(&self, _other_node: Node) -> f32 {
//         0.0
//     }
// }