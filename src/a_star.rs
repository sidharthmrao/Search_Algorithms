use crate::heuristic::{CostFunction, Heuristic};
use crate::node::{Node, NodeList};

pub struct AStar {
    heuristic: Box<dyn Heuristic>,
    cost_function: Box<dyn CostFunction>,
    all_nodes: NodeList,
    open_list: NodeList,
    closed_list: NodeList,
    start_node: Node,
    pub(crate) current_node: Option<Box<Node>>,
    target_node: Node,
}

impl AStar {
    pub fn new(heuristic: Box<dyn Heuristic>, cost_function: Box<dyn CostFunction>, nodes: NodeList, start_node: Node, target_node: Node) -> AStar {
        let mut star = AStar {
            heuristic,
            cost_function,
            all_nodes: nodes.clone(),
            open_list: NodeList::new(),
            closed_list: NodeList::new(),
            start_node: start_node.clone(),
            current_node: None,
            target_node,
        };

        star.open_list.push(Some(Box::new(star.start_node.clone().off_of(None))));

        star
    }

    // pub fn get_min_cost_node(&mut self, current_node: Node, mut node_list: NodeList) -> Option<Box<Node>> {
    //     if node_list.nodes.len() == 0 {
    //         return None;
    //     }
    //
    //     node_list.update_nodes(self.target_node.clone(), &self.heuristic, &self.cost_function);
    //
    //     let mut min_cost_node = node_list.nodes[0].clone();
    //
    //     for mut node in node_list.nodes {
    //         if node.f < min_cost_node.f {
    //             min_cost_node = node;
    //         }
    //     }
    //     Some(Box::new(min_cost_node))
    // }

    pub fn evaluate(&mut self) -> Node {
        let current_node = *self.open_list.get_min_cost_node_from_node(self.target_node.clone(), &self.heuristic, &self.cost_function).unwrap();
        self.open_list.nodes.remove(self.open_list.nodes.iter().position(|x| *x == current_node).unwrap());
        self.closed_list.push(Some(Box::new(current_node.clone())));

        let possible_nodes = self.all_nodes.find_walkable(current_node.clone());

        for mut i in possible_nodes {
            if i == self.target_node {
                println!("Found target node!");
                let mut node = self.target_node.off_of(Some(Box::new(current_node.clone())));
                node.update(&self.target_node.clone(), &self.heuristic, &self.cost_function);
                return node;
            }

            i.parent = Some(Box::new(current_node.clone()));
            i.update(&self.target_node.clone(), &self.heuristic, &self.cost_function);

            if !self.closed_list.nodes.contains(&i) {
                if !self.open_list.nodes.contains(&i) {
                    self.open_list.push(Some(Box::new(i.clone())));
                } else {
                    let position = self.open_list.nodes.iter().position(|x| *x == i).unwrap();
                    let mut node = self.open_list.nodes.get_mut(position);
                    if node.cloned().unwrap().g > i.g {
                        node = Some(&mut *Box::new(i.clone()));
                    }
                }
            }
        }

        if self.open_list.nodes.len() == 0 {
            println!("No path found!");
            return self.target_node.clone();
        }

        self.current_node = Some(Box::new(current_node.clone()));

        self.evaluate()




        // let prev_node = self.current_node.clone();
        // let mut current_node = self.get_min_cost_node(self.current_node.clone(), self.open_list.clone());
        // self.open_list.nodes.remove(self.open_list.nodes.iter().position(|x| *x == current_node).unwrap());
        // self.current_node = current_node.off_of(prev_node);
        // self.closed_list.push(current_node.clone());
        //
        // println!("Current node: {:?}", current_node);
        // println!("{:?}", self.open_list);
        // println!("{:?}", self.closed_list);
        // // Move square to closed list
        // // For every valid node:
        //     // If not in open list, add to open list. Record F, G, and H costs.
        //     // If in open list, check if new path to node is better than old path. If so, update costs.
    }
}