extern crate geoloc;

use geoloc::rtree::{Node, Leaf};

fn main() {
    let tree = Node::new_rectangles(51.0, 31.0, 59.0, 39.0, vec![
        Node::new_leaves(vec![
            Leaf::new(55.0, 35.0, 2020, 11, 22, 19, 20, 25)
        ])
    ]);

    print_tree(&tree);
}

fn print_tree(root: &Node) {
    match root {
        Node::Leaves(leaves) =>
            for leaf in leaves {
                println!("Lat {} Long {} TS {}",
                    leaf.point.latitude,
                    leaf.point.longitude,
                    leaf.timestamp)
            },
        Node::Rectangles(rectangle, nodes) =>
        {
            println!("({}, {}) - ({}, {})",
                rectangle.left_top.latitude,
                rectangle.left_top.longitude,
                rectangle.right_bottom.latitude,
                rectangle.right_bottom.longitude);

            for node in nodes {
                print_tree(&node)
            }
        }
    }
}