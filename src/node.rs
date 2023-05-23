use crate::heuristic::{CostFunction, Heuristic};

pub trait Validity {
    fn is_valid(&self, other_node: Node) -> bool;
}

pub fn get_path_cost(mut path: Option<Box<Node>>) -> f32 {
    return path.unwrap().g;
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

    pub fn find_gen_walkable(&self, node: Node) -> Vec<Node> {
        let mut walkable_nodes: Vec<Node> = Vec::new();
        for n in &self.nodes {
            if !n.equals(&node) && (((n.x - node.x).abs() == 2.0 && n.y == node.y) || ((n.y - node.y).abs() == 2.0 && n.x == node.x)) {
                walkable_nodes.push(n.clone());
            }
        }
        walkable_nodes
    }

    pub fn update_nodes(&mut self, target_node: Node, heuristic: &Box<dyn Heuristic>, cost_function: &Box<dyn CostFunction>) {
        for node in &mut self.nodes {
            node.update(&target_node, heuristic, cost_function);
        }
    }

    pub fn get_min_cost_node_from_node(&mut self, target_node: Node, heuristic: &Box<dyn Heuristic>, cost_function: &Box<dyn CostFunction>) -> Option<Box<Node>> {
        if self.nodes.len() == 0 {
            return None;
        }

        self.update_nodes(target_node, heuristic, cost_function);

        let mut min_cost_node = self.nodes[0].clone();
        for node in &mut self.nodes {
            if node.f < min_cost_node.f {
                min_cost_node = node.clone();
            }
        }
        Some(Box::new(min_cost_node))
    }

    pub fn contains(&self, node: &Node) -> bool {
        for n in &self.nodes {
            if n.equals(node) {
                return true;
            }
        }
        false
    }

    pub fn get_position(&self, node: &Node) -> Option<usize> {
        for (i, n) in self.nodes.iter().enumerate() {
            if n.equals(node) {
                return Some(i);
            }
        }
        None
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

    pub core_cost: f32,

    pub parent: Option<Box<Node>>,
}

impl Validity for Node {
    fn is_valid(&self, other_node: Node) -> bool {
        !self.equals(&other_node) &&
            // (self.x - other_node.x).abs() <= 1.0 && (self.y - other_node.y).abs() <= 1.0 && (self.z - other_node.z).abs() <= 1.0
            ((self.x - other_node.x).abs() + (self.y - other_node.y).abs() + (self.z - other_node.z).abs()) <= 1.0
    }
}

impl Node {
    pub(crate) fn new(x: f32, y: f32, z: f32, core_cost: f32, parent: Option<Box<Node>>) -> Node {
        let node = Node {
            x,
            y,
            z,
            f: 0.0,
            g: 0.0,
            h: 0.0,
            core_cost,
            parent
        };

        node
    }

    pub fn update(&mut self, target: &Node, heuristic: &Box<dyn Heuristic>, cost_function: &Box<dyn CostFunction>) {
        let parent = self.parent.clone();

        match parent {
            Some(p) => {
                let p = *p;
                self.g = p.g + cost_function.cost(&p, self) + self.core_cost;
            },
            None => {
                self.g = 0.0;
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
            core_cost: self.core_cost,
            parent,
        }
    }

    pub fn equals(&self, other_node: &Node) -> bool {
        self.x == other_node.x && self.y == other_node.y && self.z == other_node.z
    }
}
