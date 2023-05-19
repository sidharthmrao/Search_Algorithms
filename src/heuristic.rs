use crate::node::{Node};

pub trait Heuristic {
    fn distance(&self, other_node: Node) -> f32;
}

pub struct ManhattanDistance {
    target: Node,
}

impl Heuristic for ManhattanDistance {
    fn distance(&self, other_node: Node) -> f32 {
        let x = (self.target.x - other_node.x).abs();
        let y = (self.target.y - other_node.y).abs();
        let z = (self.target.z - other_node.z).abs();
        (x + y + z) as f32
    }
}

pub struct EuclideanDistance {
    target: Node,
}

impl Heuristic for EuclideanDistance {
    fn distance(&self, other_node: Node) -> f32 {
        let x = (self.target.x - other_node.x).pow(2);
        let y = (self.target.y - other_node.y).pow(2);
        let z = (self.target.z - other_node.z).pow(2);
        (x + y + z).sqrt()
    }
}

pub struct ChebyshevDistance {
    target: Node,
}

impl Heuristic for ChebyshevDistance {
    fn distance(&self, other_node: Node) -> f32 {
        let x = (self.target.x - other_node.x).abs();
        let y = (self.target.y - other_node.y).abs();
        let z = (self.target.z - other_node.z).abs();
        x.max(y).max(z) as f32
    }
}

pub struct OctileDistance {
    target: Node,
}

impl Heuristic for OctileDistance {
    fn distance(&self, other_node: Node) -> f32 {
        let x = (self.target.x - other_node.x).abs();
        let y = (self.target.y - other_node.y).abs();
        let z = (self.target.z - other_node.z).abs();
        let min = x.min(y).min(z);
        let max = x.max(y).max(z);
        (min * (2.0_f32.sqrt() - 1.0) + max) as f32
    }
}

pub struct DiagonalDistance {
    target: Node,
}

impl Heuristic for DiagonalDistance {
    fn distance(&self, other_node: Node) -> f32 {
        let x = (self.target.x - other_node.x).abs();
        let y = (self.target.y - other_node.y).abs();
        let z = (self.target.z - other_node.z).abs();
        let min = x.min(y).min(z);
        let max = x.max(y).max(z);
        (min + (3.0_f32.sqrt() - 1.0) * max) as f32
    }
}

pub struct Dijkstra {
    target: Node,
}

impl Heuristic for Dijkstra {
    fn distance(&self, _other_node: Node) -> f32 {
        0.0
    }
}