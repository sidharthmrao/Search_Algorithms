use crate::heuristic::Heuristic;
use crate::node::{Node, NodeList};

pub struct AStar {
    heuristic: Box<dyn Heuristic>,
    open_list: NodeList,
    closed_list: NodeList,
    start_node: Node,
    current_node: Option<Box<Node>>,
    target_node: Node,
}

impl AStar {
    pub fn new(heuristic: Box<dyn Heuristic>, nodes: NodeList, start_node: Node, target_node: Node) -> AStar{
        let mut star = AStar {
            heuristic,
            open_list: nodes,
            closed_list: NodeList::new(),
            start_node: start_node.clone(),
            current_node: None,
            target_node,
        };

        star.open_list.push(Some(Box::new(start_node)));

        star
    }

    pub fn get_min_cost_node(&mut self, node_list: NodeList) -> Option<Box<Node>> {
        if node_list.nodes.len() == 0 {
            return None;
        }

        let mut min_cost_node = node_list.nodes[0].clone();
        for node in node_list.nodes {
            if node.f < min_cost_node.f {
                min_cost_node = node;
            }
        }
        Some(Box::new(min_cost_node))
    }

    pub fn evaluate(&mut self) {
        let prev_node = self.current_node.clone();
        let mut current_node = self.get_min_cost_node(self.open_list.clone());
        self.open_list.nodes.remove(self.open_list.nodes.iter().position(|x| *x == current_node).unwrap());
        self.current_node = current_node.off_of(prev_node);
        self.current_node.update(&self.target_node, &self.heuristic);
        self.closed_list.push(current_node.clone());

        println!("Current node: {:?}", current_node);
        println!("{:?}", self.open_list);
        println!("{:?}", self.closed_list);
        // Move square to closed list
        // For every valid node:
            // If not in open list, add to open list. Record F, G, and H costs.
            // If in open list, check if new path to node is better than old path. If so, update costs.
    }
}