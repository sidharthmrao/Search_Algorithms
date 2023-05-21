use crate::a_star::AStar;
use crate::node::Node;
use crate::heuristic::ManhattanDistance;

mod node;
mod a_star;
mod heuristic;

fn main() {
    let mut a_star = AStar::new(
        Box::new(ManhattanDistance {}),
        node::NodeList::from_vec(
            Vec::from(
                [
                    Node::new(0.0, 0.0, 0.0, None),
                    Node::new(0.5, 0.5, 0.5, None),
                    Node::new(1.0, 1.0, 1.0, None),
                    Node::new(1.5, 1.5, 1.5, None),
                ],
            )
        ),
        Node::new(0.0, 0.0, 0.0, None),
        Node::new(1.5, 1.5, 1.5, None),
    );

    a_star.evaluate();
}
