use chrono::{DateTime, Utc, NaiveDate, TimeZone};
use float_cmp::ApproxEq;
use float_cmp::F64Margin;

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

    fn distance_to_segment(&self, start: &Point, end: &Point) -> f64 {
        let a = &self;
        let b = &start;
        let c = &end;

        let ab_dot_ac = (b.latitude - a.latitude) * (c.latitude - a.latitude)
                      + (b.longitude - a.longitude) * (c.longitude - a.longitude);

        let ab = a.distance(&b);
        let ac = a.distance(&c);

        let cos_a = ab_dot_ac / (ab * ac);

        if cos_a > 0.0 {
            ab.min(ac)
        }
        else {
            let bc = b.distance(&c);
            let sin_a = cos_a.acos().sin();

            (ac * ab * sin_a)/bc
        }
    }
}

fn assert_approx_eq(value1: f64, value2: f64) {
    assert!(value1.approx_eq(value2, F64Margin { ulps: 2, epsilon: 0.00001 }));
}

#[test]
fn point_new_when_called_fills_latitude_and_longitude() {
    let point = Point::new(123.456, -123.789);

    assert_approx_eq(123.456, point.latitude);
    assert_approx_eq(-123.789, point.longitude)
}

#[test]
fn point_distance_with_vertical_segment_returns_height() {
    const SAME_LONGITUDE: f64 = 50.0;

    let point1 = Point::new(90.0, SAME_LONGITUDE);
    let point2 = Point::new(80.0, SAME_LONGITUDE);

    let actual = point1.distance(&point2);

    assert_approx_eq(actual, 90.0 - 80.0);
}

#[test]
fn point_distance_with_horizontal_segment_returns_width() {
    const SAME_LATITUDE: f64 = 50.0;

    let point1 = Point::new(SAME_LATITUDE, 70.0);
    let point2 = Point::new(SAME_LATITUDE, 40.0);

    let actual = point1.distance(&point2);

    assert_approx_eq(actual, 70.0 - 40.0);
}

#[test]
fn point_distance_with_pythagorean_triple_cathetuses_returns_hypotenuse() {
    let point1 = Point::new(3.0, 4.0);
    let point2 = Point::new(0.0, 0.0);

    let actual = point1.distance(&point2);

    assert_approx_eq(actual, 5.0);
}

#[test]
fn distance_to_segment_opposite_to_horizontal_segment_returns_same_distance() {
    const SAME_LATITUDE: f64 = 50.0;
    const LEFT_LONGITUDE: f64 = 30.0;
    let left = Point::new(SAME_LATITUDE, LEFT_LONGITUDE);
    let right = Point::new(SAME_LATITUDE, LEFT_LONGITUDE + 30.0);

    const DISTANCE: f64 = 10.0;
    let opposite1 = Point::new(SAME_LATITUDE + DISTANCE, LEFT_LONGITUDE + 0.0);
    let opposite2 = Point::new(SAME_LATITUDE + DISTANCE, LEFT_LONGITUDE + 10.0);
    let opposite3 = Point::new(SAME_LATITUDE + DISTANCE, LEFT_LONGITUDE + 20.0);
    let opposite4 = Point::new(SAME_LATITUDE + DISTANCE, LEFT_LONGITUDE + 30.0);

    assert_approx_eq(DISTANCE, opposite1.distance_to_segment(&left, &right));
    assert_approx_eq(DISTANCE, opposite2.distance_to_segment(&left, &right));
    assert_approx_eq(DISTANCE, opposite3.distance_to_segment(&left, &right));
    assert_approx_eq(DISTANCE, opposite4.distance_to_segment(&left, &right));
}

#[test]
fn distance_to_segment_with_pythagorean_triple_diagonal_returns_hypotenuse() {
    const SAME_LATITUDE: f64 = 50.0;
    const LEFT_LONGITUDE: f64 = 30.0;
    let left = Point::new(SAME_LATITUDE, LEFT_LONGITUDE);
    let right = Point::new(SAME_LATITUDE, LEFT_LONGITUDE + 30.0);

    let diagonal = Point::new(SAME_LATITUDE + 4.0, LEFT_LONGITUDE - 3.0);

    assert_approx_eq(5.0, diagonal.distance_to_segment(&left, &right));
}

struct Rectangle {
    left_top: Point,
    right_bottom: Point,
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

    fn is_inside(&self, point: &Point) -> bool {
        point.latitude >= self.left_top.latitude &&
        point.latitude <= self.right_bottom.latitude &&
        point.longitude >= self.left_top.longitude &&
        point.longitude <= self.right_bottom.longitude
    }

    fn distance(&self, point: &Point) -> f64 {
        if self.is_inside(&point) {
            0.0
        }
        else {
            0.0
        }
    }
}

#[test]
fn rectangle_new_when_called_fills_pair_of_points() {
    let rectangle = Rectangle::new(50.0, 60.0, 70.0, 80.0);

    assert_approx_eq(50.0, rectangle.left_top.latitude);
    assert_approx_eq(60.0, rectangle.left_top.longitude);
    assert_approx_eq(70.0, rectangle.right_bottom.latitude);
    assert_approx_eq(80.0, rectangle.right_bottom.longitude);
}


#[test]
fn rectangle_new_with_invalid_order_of_points_keeps_invalid_order() {
    // right_top.latitude < left_top.latitude, it's incorrect
    let rectangle = Rectangle::new(70.0, 80.0, 50.0, 60.0);

    assert_approx_eq(70.0, rectangle.left_top.latitude);
    assert_approx_eq(80.0, rectangle.left_top.longitude);
    assert_approx_eq(50.0, rectangle.right_bottom.latitude);
    assert_approx_eq(60.0, rectangle.right_bottom.longitude);
}

#[test]
fn rectangle_is_inside_with_inside_point_returns_true() {
    let rectangle = Rectangle::new(0.0, 0.0, 10.0, 10.0);
    let inside_point = Point::new(5.0, 5.0);

    assert!(rectangle.is_inside(&inside_point));
}

#[test]
fn rectangle_is_inside_with_outside_points_returns_false() {
    let rectangle = Rectangle::new(0.0, 0.0, 10.0, 10.0);
    let left_point = Point::new(5.0, -5.0);
    let top_point = Point::new(15.0, 5.0);
    let right_point = Point::new(5.0, 15.0);
    let bottom_point = Point::new(-5.0, 5.0);

    assert!(!rectangle.is_inside(&left_point));
    assert!(!rectangle.is_inside(&top_point));
    assert!(!rectangle.is_inside(&right_point));
    assert!(!rectangle.is_inside(&bottom_point));
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