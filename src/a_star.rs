use crate::heuristic::Heuristic;
use crate::node::{Node, NodeList};

pub struct AStar {
    heuristic: Box<dyn Heuristic>,
    open_list: NodeList,
    closed_list: NodeList,
    start_node: Node,
    current_node: Node,
    target_node: Node,
}

impl AStar {
    pub fn new(&mut self, heuristic: Box<dyn Heuristic>, nodes: NodeList, start_node: Node, target_node: Node) -> AStar{
        let mut star = AStar {
            heuristic,
            open_list: nodes,
            closed_list: NodeList::new(),
            start_node,
            current_node: start_node,
            target_node,
        };

        star.open_list.push(start_node);

        star
    }

    pub fn get_min_cost_node(&mut self, node_list: NodeList) -> Node {
        let mut min_cost_node = node_list.nodes[0];
        for node in node_list.nodes {
            if node.f < min_cost_node.f {
                min_cost_node = node;
            }
        }
        min_cost_node
    }

    pub fn evaluate(&mut self) {
        let mut current_node = self.get_min_cost_node(self.open_list.clone());
        // Move square to closed list
        // For every valid node:
            // If not in open list, add to open list. Record F, G, and H costs.
            // If in open list, check if new path to node is better than old path. If so, update costs.
    }
}