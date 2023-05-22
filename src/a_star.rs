use std::thread;
use crate::heuristic::{CostFunction, Heuristic};
use crate::maze::Maze;
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
    maze: Maze,
    iters: u32,
}

impl AStar {
    pub fn new(heuristic: Box<dyn Heuristic>, cost_function: Box<dyn CostFunction>, nodes: NodeList, start_node: Node, target_node: Node, maze: Maze) -> AStar {
        let mut star = AStar {
            heuristic,
            cost_function,
            all_nodes: nodes.clone(),
            open_list: NodeList::new(),
            closed_list: NodeList::new(),
            start_node: start_node.clone(),
            current_node: None,
            target_node,
            maze,
            iters: 0
        };

        star.open_list.push(Some(Box::new(star.start_node.clone().off_of(None))));

        star
    }

    pub fn evaluate(&mut self) -> (Option<Box<Node>>, u32) {
        while self.open_list.nodes.len() > 0 {
            thread::sleep(std::time::Duration::from_millis(20));
            self.iters += 1;

            let current_node = *self.open_list.get_min_cost_node_from_node(self.target_node.clone(), &self.heuristic, &self.cost_function).unwrap();
            self.open_list.nodes.remove(self.open_list.nodes.iter().position(|x| *x == current_node).unwrap());
            self.closed_list.push(Some(Box::new(current_node.clone())));

            if self.iters % 10 == 0 {
                self.maze.render_path(Some(Box::new(current_node.clone())), self.start_node.clone(), self.target_node.clone(), self.iters);
            }

            let possible_nodes = self.all_nodes.find_walkable(current_node.clone());

            for mut i in possible_nodes {
                if i == self.target_node {
                    println!("Found target node!");
                    let mut node = self.target_node.off_of(Some(Box::new(current_node.clone())));
                    node.update(&self.target_node.clone(), &self.heuristic, &self.cost_function);
                    return (Some(Box::new(node)), self.iters);
                }

                i.parent = Some(Box::new(current_node.clone()));
                i.update(&self.target_node.clone(), &self.heuristic, &self.cost_function);

                if !self.closed_list.contains(&i) {
                    if !self.open_list.contains(&i) {
                        self.open_list.push(Some(Box::new(i.clone())));
                    } else {
                        let position = self.open_list.get_position(&i).unwrap();
                        let mut node = self.open_list.nodes.get_mut(position);
                        if node.cloned().unwrap().g > i.g {
                            node = Some(&mut *Box::new(i.clone()));
                        }
                    }
                }
            }

            self.current_node = Some(Box::new(current_node.clone()));
        }
        println!("No path found!");
        return (None, self.iters);
    }
}