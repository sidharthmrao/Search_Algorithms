use crate::a_star::AStar;
use crate::node::{Node, NodeList};
use crate::heuristic::{EuclideanCost, ManhattanDistance};
use crate::maze::Maze;

mod node;
mod a_star;
mod heuristic;
mod maze;

fn main() {
    let maze = vec![
        vec![-1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![4, 1, 1, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 2, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    ];

    let mut node_list = NodeList::new();
    let mut start_node = Node::new(0.0, 0.0, 0.0, 0.0,None);
    let mut target_node = Node::new(0.0, 0.0, 0.0, 0.0,None);

    for (i, row) in maze.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if value == &0 || value == &2 || value == &-1 {
                node_list.push(Some(Box::new(Node::new(i as f32, j as f32, 0.0, 0.0,None))));
            } else if value == &3 {
                node_list.push(Some(Box::new(Node::new(i as f32, j as f32, 0.0, 1.0,None))));
            }
            else if value == &4 {
                node_list.push(Some(Box::new(Node::new(i as f32, j as f32, 0.0, 3.0,None))));
            }
            if value == &2 {
                target_node = Node::new(i as f32, j as f32, 0.0, 0.0,None);
            }
            if value == &-1 {
                start_node = Node::new(i as f32, j as f32, 0.0, 0.0,None);
            }
        }
    }

    let mut a_star = AStar::new(
        Box::new(ManhattanDistance {}),
        Box::new(EuclideanCost {}),
        node_list,
        start_node,
        target_node,
        Maze::new(maze.clone()),
    );

    let mut path = Some(Box::new(a_star.evaluate()));

    let mut nodes: Vec<Node> = Vec::new();

    while path.is_some() {
        nodes.push(path.clone().unwrap().off_of(None));
        path = path.clone().unwrap().parent;
    }

    let mut string_maze = Vec::from([]);
    for row in maze.iter() {
        string_maze.push(Vec::from([]));
        for value in row.iter() {
            string_maze.last_mut().unwrap().push(value.to_string());
        }
    }

    for i in nodes.iter().rev() {
        string_maze[i.x as usize][i.y as usize] = "•".to_string();
    }

    for row in string_maze.iter() {
        println!("{:?}", row);
    }
}
