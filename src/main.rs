use chrono::{DateTime, Utc, NaiveDate, TimeZone};

struct Point {
    latitude: f64,
    longitude: f64,
}

impl Point {
    fn new(latitude: f64, longitude: f64) -> Point {
        Point {
            latitude: latitude,
            longitude: longitude,
        }
    }

    fn distance(&self, point: &Point) -> f64 {
        let latitude_cathetus = self.latitude - point.latitude;
        let longtitude_cathetus = self.longitude - point.longitude;
        let hypotenuse = latitude_cathetus.powi(2) + longtitude_cathetus.powi(2);

        hypotenuse.sqrt()
    }
}

#[test]
fn new_when_called_fills_latitude_and_longitude() {
    let point = Point::new(123.456, -123.789);

    assert_eq!(123.456, point.latitude);
    assert_eq!(-123.789, point.longitude)
}

#[test]
fn distance_with_vertical_segment_returns_height() {
    let point1 = Point::new(90.0, 50.0);
    let point2 = Point::new(80.0, 50.0);

    let actual = point1.distance(&point2);

    assert_eq!(actual, 90.0 - 80.0);
}

#[test]
fn distance_with_horizontal_segment_returns_width() {
    let point1 = Point::new(50.0, 70.0);
    let point2 = Point::new(50.0, 40.0);

    let actual = point1.distance(&point2);

    assert_eq!(actual, 70.0 - 40.0);
}

#[test]
fn distance_with_pythagorean_triple_returns_hypotenuse() {
    let point1 = Point::new(3.0, 4.0);
    let point2 = Point::new(0.0, 0.0);

    let actual = point1.distance(&point2);

    assert_eq!(actual, 5.0);
}

struct Rectangle {
    left_top: Point,
    right_bottom: Point
}

impl Rectangle {
    fn new(latitude1: f64, longitude1: f64, latitude2: f64, longitude2: f64) -> Rectangle {
        Rectangle {
            left_top: Point {
                latitude: latitude1,
                longitude: longitude1
            },
            right_bottom: Point {
                latitude: latitude2,
                longitude: longitude2
            },
        }
    }

    fn distance(&self, point: &Point) -> f64 {
        0.0
    }
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