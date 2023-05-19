use crate::heuristic::Heuristic;

pub trait Validity {
    fn is_valid(&self, other_node: Node) -> bool;
}

pub struct NodeList {
    pub nodes: Vec<Node>,
}

impl NodeList {
    pub fn new() -> NodeList {
        NodeList {
            nodes: Vec::new(),
        }
    }

    pub fn find_walkable(&self, node: Node) -> Vec<Node> {
        let mut walkable_nodes: Vec<Node> = Vec::new();
        for n in &self.nodes {
            if n.is_valid(node) {
                walkable_nodes.push(n.clone());
            }
        }
        walkable_nodes
    }
}

#[derive(Clone, Copy)]
pub struct Node {
    pub x: i32,
    pub y: i32,
    pub z: i32,

    pub f: f32,
    pub g: f32,
    pub h: f32,

    pub parent: Option<Node>,
}

impl Validity for Node {
    fn is_valid(&self, other_node: Node) -> bool {
        self.x != other_node.x && self.y != other_node.y && self.z != other_node.z &&
            (self.x - other_node.x).abs() < 1 && (self.y - other_node.y).abs() < 1 && (self.z - other_node.z < 1).abs()
    }
}

impl Node {
    fn new(x: i32, y: i32, z: i32, parent: Node, heuristic: Box<dyn Heuristic>) -> Node {
        let mut node = Node {
            x,
            y,
            z,
            f: 0.0,
            g: 0.0,
            h: 0.0,
            parent,
        };

        node.update(parent, heuristic);

        node
    }

    fn update(&mut self, target: Node, heuristic: Box<dyn Heuristic>) {
        self.g = self.parent.g + 1.0;
        self.h = heuristic.distance(target);
        self.f = self.g + self.h;
    }
}
