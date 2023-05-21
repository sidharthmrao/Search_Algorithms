use crate::heuristic::Heuristic;

pub trait Validity {
    fn is_valid(&self, other_node: Node) -> bool;
}

#[derive(Clone, Debug)]
pub struct NodeList {
    pub nodes: Vec<Node>,
}

impl NodeList {
    pub fn new() -> NodeList {
        NodeList {
            nodes: Vec::new(),
        }
    }

    pub fn from_vec(nodes: Vec<Node>) -> NodeList {
        NodeList {
            nodes,
        }
    }

    pub fn push(&mut self, node: Option<Box<Node>>) {
        self.nodes.push(*node.unwrap());
    }

    pub fn find_walkable(&self, node: Node) -> Vec<Node> {
        let mut walkable_nodes: Vec<Node> = Vec::new();
        for n in &self.nodes {
            if n.is_valid(node.clone()) {
                walkable_nodes.push(n.clone());
            }
        }
        walkable_nodes
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub f: f32,
    pub g: f32,
    pub h: f32,

    pub parent: Option<Box<Node>>,
}

impl Validity for Node {
    fn is_valid(&self, other_node: Node) -> bool {
        self.x != other_node.x && self.y != other_node.y && self.z != other_node.z &&
            (self.x - other_node.x).abs() < 1.0 && (self.y - other_node.y).abs() < 1.0 && (self.z - other_node.z).abs() < 1.0
    }
}

impl Node {
    pub(crate) fn new(x: f32, y: f32, z: f32, parent: Option<Box<Node>>) -> Node {
        let mut node = Node {
            x,
            y,
            z,
            f: 0.0,
            g: 0.0,
            h: 0.0,
            parent
        };

        node
    }

    pub fn update(&mut self, target: &Node, heuristic: &Box<dyn Heuristic>) {
        let parent = self.parent.clone();

        match parent {
            Some(p) => {
                let p = *p;
                self.g = p.g + 1.0;
            },
            None => {
                self.g = 1.0;
            },
        }

        self.h = heuristic.distance(self, target);
        self.f = self.g + self.h;
    }

    pub fn off_of(&mut self, parent: Option<Box<Node>>) -> Node {
        Node {
            x: self.x,
            y: self.y,
            z: self.z,
            f: self.f,
            g: self.g,
            h: self.h,
            parent,
        }
    }
}
