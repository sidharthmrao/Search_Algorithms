use crate::node::{Node};

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

pub struct ManhattanCost {}

impl CostFunction for ManhattanCost {
    fn cost(&self, node: &Node, other_node: &Node) -> f32 {
        let x = (node.x - other_node.x).abs();
        let y = (node.y - other_node.y).abs();
        let z = (node.z - other_node.z).abs();
        x + y + z
    }
}

pub struct StaticCost {
    pub cost: f32,
}

impl CostFunction for StaticCost {
    fn cost(&self, node: &Node, other_node: &Node) -> f32 {
        self.cost
    }
}

pub trait Heuristic {
    fn distance(&self, node: &Node, other_node: &Node) -> f32;
}

pub struct ManhattanDistance {}

impl Heuristic for ManhattanDistance {
    fn distance(&self, node: &Node, target_node: &Node) -> f32 {
        let x = (node.x - target_node.x).abs();
        let y = (node.y - target_node.y).abs();
        let z = (node.z - target_node.z).abs();
        ((x + y + z) as f32).sqrt()
    }
}

pub struct EuclideanDistance {}


impl Heuristic for EuclideanDistance {
    fn distance(&self, node: &Node, target_node: &Node) -> f32 {
        let x = (node.x - target_node.x).powf(2.0);
        let y = (node.y - target_node.y).powf(2.0);
        let z = (node.z - target_node.z).powf(2.0);
        ((x + y + z) as f32).sqrt()
    }
}

pub struct OctileDistance {}

impl Heuristic for OctileDistance {
    fn distance(&self, node: &Node, target_node: &Node) -> f32 {
        let x = (node.x - target_node.x).abs();
        let y = (node.y - target_node.y).abs();
        let z = (node.z - target_node.z).abs();
        let min = x.min(y).min(z);
        let max = x.max(y).max(z);
        (min * (3.0 as f32).sqrt() + (max - min)) as f32
    }
}

pub struct ChebyshevDistance {}

impl Heuristic for ChebyshevDistance {
    fn distance(&self, node: &Node, target_node: &Node) -> f32 {
        let x = (node.x - target_node.x).abs();
        let y = (node.y - target_node.y).abs();
        let z = (node.z - target_node.z).abs();
        x.max(y).max(z)
    }
}

pub struct DiagonalDistance {}

impl Heuristic for DiagonalDistance {
    fn distance(&self, node: &Node, target_node: &Node) -> f32 {
        let x = (node.x - target_node.x).abs();
        let y = (node.y - target_node.y).abs();
        let z = (node.z - target_node.z).abs();
        (x + y + z).max(x.max(y).max(z))
    }
}

pub struct DijkstraDistance {}

impl Heuristic for DijkstraDistance {
    fn distance(&self, node: &Node, target_node: &Node) -> f32 {
        0.0
    }
}
