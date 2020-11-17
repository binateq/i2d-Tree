use chrono::{DateTime, Utc, NaiveDate, TimeZone};

struct Point {
    latitude: f64,
    longitude: f64,
}

struct Rectangle {
    left_top: Point,
    right_bottom: Point
}

struct Leaf {
    point: Point,
    timestamp: DateTime<Utc>,
}

enum Node {
    Leaves(Vec<Leaf>),
    Rectangles(Rectangle, Vec<Node>)
}

fn main() {
    let date_time = NaiveDate::from_ymd(2020, 11, 17).and_hms(11, 39, 00);
    let utc_date_time = Utc.from_utc_datetime(&date_time);

    let tree = Node::Rectangles(
        Rectangle {
            left_top: Point {
                latitude: 51.0,
                longitude: 31.0,
            },
            right_bottom: Point {
                latitude: 59.0,
                longitude: 39.0,
            },
        },
        vec![
            Node::Leaves(vec![
                Leaf {
                    point: Point {
                        latitude: 57.0,
                        longitude: 37.0,
                    },
                    timestamp: utc_date_time,
                }
            ])
        ]
    );

    print_tree(&tree);
}

fn print_tree(root: &Node) {
    match root {
        Node::Leaves(leaves) =>
            for leaf in leaves
            {
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
            for node in nodes
            {
                print_tree(node)
            }
        }
    }
}