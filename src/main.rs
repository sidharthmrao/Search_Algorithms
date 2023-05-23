use std::str::FromStr;
use crate::a_star::AStar;
use crate::node::{get_path_cost, Node, NodeList};
use crate::heuristic::{ChebyshevDistance, DiagonalDistance, EuclideanCost, EuclideanDistance, ManhattanCost, ManhattanDistance, OctileDistance, StaticCost};
use crate::maze::{DefaultMaze, Maze, MazeGenerator};

mod node;
mod a_star;
mod heuristic;
mod maze;

fn main() {
    let debug = true;

    let maze = DefaultMaze::new(40, 40).maze;

    let mut x = MazeGenerator::new(maze.clone(), debug);
    x.randomized_dfs(Node::new(0.0, 0.0, 0.0, 0.0, None));

    let mut y = Maze::new(x.maze.clone());
    y.render();

    let maze = y.maze;

    let mut node_list = NodeList::new();
    let mut start_node = Node::new(0.0, 0.0, 0.0, 0.0,None);
    let mut target_node = Node::new(0.0, 0.0, 0.0, 0.0,None);

    for (i, row) in maze.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if value == &0 {
                node_list.push(Some(Box::new(Node::new(i as f32, j as f32, 0.0, 0.0,None))));
            } else if value == &2 {
                node_list.push(Some(Box::new(Node::new(i as f32, j as f32, 0.0, 0.0,None))));
                target_node = Node::new(i as f32, j as f32, 0.0, 0.0,None);
            } else if value == &-1 {
                node_list.push(Some(Box::new(Node::new(i as f32, j as f32, 0.0, 0.0,None))));
                start_node = Node::new(i as f32, j as f32, 0.0, 0.0,None);
            } else if value != &1 {
                node_list.push(Some(Box::new(Node::new(i as f32, j as f32, 0.0, *value as f32,None))));
            }
        }
    }

    let mut a_star = AStar::new(
        Box::new(ManhattanDistance {}),
        Box::new(ManhattanCost {}),
        node_list,
        start_node.clone(),
        target_node.clone(),
        Maze::new(maze.clone()),
        debug
    );

    let mut resp = a_star.evaluate();
    let mut path = resp.0;
    let mut iters = resp.1;

    let mut maze = Maze::new(maze.clone());
    maze.render_path(path.clone(), start_node, target_node, iters);

    match path {
        Some(_) => println!("Path found in {} iterations.", iters),
        None => println!("No path found after {} iterations.", iters),
    }

    if !a_star.debug {
        println!("Elapsed time: {}ms", a_star.elapsed_time.elapsed().unwrap().as_millis());
    }

    println!("Cost: {}", get_path_cost(path));
}
