use crate::node::{Node, NodeList};
use colored::Colorize;
use crossterm;
use crossterm::ExecutableCommand;
use std::io::Write;
use std::{thread, time};
use crossterm::style::Stylize;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Maze {
    pub maze: Vec<Vec<i32>>,
    stdout: std::io::Stdout,
}

impl Maze {
    pub(crate) fn new(maze: Vec<Vec<i32>>) -> Maze {
        Maze {
            maze,
            stdout: std::io::stdout(),
        }
    }

    pub fn render(&mut self) {
        self.stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();

        let mut string_maze = Vec::from([]);

        for row in self.maze.iter() {
            string_maze.push(Vec::from([]));
            for value in row.iter() {
                match value {
                    0 => string_maze.last_mut().unwrap().push(Colorize::black("• ").to_string()),
                    1 => string_maze.last_mut().unwrap().push(Colorize::blue("||").to_string()),
                    _ => string_maze.last_mut().unwrap().push(Colorize::red("||").to_string())
                }
            }
        }

        let mut to_print = String::new();

        to_print.push_str("--------------------\n");

        for row in string_maze.iter() {
            for i in row.iter() {
                to_print.push_str(&(i.to_string() + &"".to_string()));
            }
            to_print.push('\n');
        }
        to_print.push_str("--------------------");
        writeln!(self.stdout, "{}", to_print).unwrap();
    }

    pub fn render_path(&mut self, mut path: Option<Box<Node>>, start: Node, target: Node, iters: u32) {
        self.stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();

        let mut nodes: Vec<Node> = Vec::new();

        while path.is_some() {
            nodes.push(path.clone().unwrap().off_of(None));
            path = path.clone().unwrap().parent;
        }

        let mut string_maze = Vec::from([]);
        for row in self.maze.iter() {
            string_maze.push(Vec::from([]));
            for value in row.iter() {
                match value {
                    0 => string_maze.last_mut().unwrap().push(Colorize::black("• ").to_string()),
                    1 => string_maze.last_mut().unwrap().push(Colorize::red("||").to_string()),
                    _ => string_maze.last_mut().unwrap().push(Colorize::blue("||").to_string())
                }
            }
        }

        for i in nodes.iter().rev() {
            string_maze[i.x as usize][i.y as usize] = Colorize::bright_magenta("• ").to_string();
        }

        string_maze[start.x as usize][start.y as usize] = Colorize::green("S ").to_string();
        string_maze[target.x as usize][target.y as usize] = Colorize::blue("T ").to_string();

        let mut to_print = String::new();

        to_print.push_str(&format!("Iteration: {}\n", iters));
        to_print.push_str("--------------------\n");

        for row in string_maze.iter() {
            for i in row.iter() {
                to_print.push_str(&(i.to_string() + &"".to_string()));
            }
            to_print.push('\n');
        }
        to_print.push_str("--------------------");
        writeln!(self.stdout, "{}", to_print).unwrap();
    }
}

pub struct MazeGenerator {
    pub maze: Vec<Vec<i32>>,
    pub visited: Vec<Node>,
    pub nodes: NodeList,
    pub debug: bool,
}

impl MazeGenerator {
    pub fn new(maze: Vec<Vec<i32>>, debug: bool) -> MazeGenerator {
        let mut maze = MazeGenerator {
            maze,
            visited: Vec::new(),
            nodes: NodeList::new(),
            debug
        };

        maze.make_node_list();

        maze
    }

    pub fn make_node_list(&mut self) {
        let mut node_list = NodeList::new();
        for (i, row) in self.maze.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if *value != 1 {
                    node_list.nodes.push(Node::new(i as f32, j as f32, 0.0, 0.0, None));
                }
            }
        }
        self.nodes = node_list;
    }

    pub fn randomized_dfs(&mut self, node: Node) {
        if self.debug {
            let mut y = Maze::new(self.maze.clone());
            y.render();
            thread::sleep(time::Duration::from_millis(25));
        }

        self.visited.push(node.clone());
        let mut neighbors = self.nodes.find_gen_walkable(node.clone());
        neighbors.shuffle(&mut thread_rng());

        for neighbor in neighbors {
            if !self.visited.contains(&neighbor) {
                self.maze[(((neighbor.x - node.x) / 2.0) + node.x) as usize][(((neighbor.y - node.y) / 2.0) + node.y) as usize] = 0;
                self.randomized_dfs(neighbor);
            }
        }
    }
}

pub struct DefaultMaze {
    pub maze: Vec<Vec<i32>>,
}

impl DefaultMaze {
    pub fn new(height: u32, width: u32) -> DefaultMaze {
        let width = width - (width % 2) + 1;

        let mut maze = vec![vec![1; width as usize]; height as usize];
        for (i, row) in maze.iter_mut().enumerate() {
            for (j, value) in row.iter_mut().enumerate() {
                if i % 2 == 0 && j % 2 == 0 {
                    *value = 0;
                }
            }
        }

        maze[0][0] = -1;
        maze[height as usize - 1][width as usize - 1] = 2;

        DefaultMaze {
            maze,
        }
    }
}
