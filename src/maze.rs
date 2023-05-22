use crate::node::Node;
use colored::Colorize;
use crossterm;
use crossterm::ExecutableCommand;
use std::io::Write;
use crossterm::style::Stylize;

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

    pub fn render(&self) {
        for row in &self.maze {
            println!("{:?}", row);
        }
    }

    pub fn render_path(&mut self, mut path: Option<Box<Node>>, start: Node, target: Node, iters: u32) {
        self.stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();

        // let mut path = Some(Box::new(path));
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
                    1 => string_maze.last_mut().unwrap().push(Colorize::red("• ").to_string()),
                    _ => string_maze.last_mut().unwrap().push(Colorize::blue("• ").to_string())
                }
            }
        }

        for i in nodes.iter().rev() {
            string_maze[i.x as usize][i.y as usize] = Colorize::bright_magenta("• ").to_string();
        }

        string_maze[start.x as usize][start.y as usize] = Colorize::green("S ").to_string();
        string_maze[target.x as usize][target.y as usize] = Colorize::red("T ").to_string();

        let mut to_print = String::new();

        to_print.push_str(&format!("Iteration: {}\n", iters));

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