use super::*;
use super::types::*;

pub fn find_nearest_recursive<'a, T: Copy>(node: &'a Node<T>, axis: Axis, point: Point) -> Option<Nearest<'a, T>> {
    match node {
        Node::Empty =>
            None,
        Node::Area { item, .. } if item.point.square_distance(point) == 0.0 =>
            Some(Nearest::new(0.0, item)),
        Node::Area { item, left, right } => {
            let parent_nearest = Nearest::new(item.point.square_distance(point), item);

            let child = if point[axis] >= item.point[axis] { right } else { left };
            let child_nearest = find_nearest_recursive(child, axis.next(), point);
            let mut nearest = parent_nearest.min(child_nearest);

            let need_check_other_child = if point[axis] >= item.point[axis] {
                point[axis] - item.point[axis] < nearest.square_distance
            } else {
                item.point[axis] - point[axis] < nearest.square_distance
            };

            if need_check_other_child {
                let other = if point[axis] >= item.point[axis] { left } else { right };
                let other_nearest = find_nearest_recursive(other, axis.next(), point);

                nearest = nearest.min(other_nearest);
            }

            Some(nearest)
        }
    }
}

fn square(x: f64) -> f64 {
    x * x
}

impl Point {
    fn square_distance(&self, other: Point) -> f64 {
        square(self.latitude - other.latitude) + square(self.longitude - other.longitude)
    }
}

#[test]
fn square_distance_with_diffs_3_and_4_returns_25() {
    let point1 = Point::new(1.0, 20.0);
    let point2 = Point::new(1.0 + 3.0, 20.0 + 4.0);

    let actual = point1.square_distance(point2);

    use approx::AbsDiffEq;
    assert!(actual.abs_diff_eq(&25.0, f64::default_epsilon()));
}

#[test]
fn find_nearest_recursive_with_empty_returns_none() {
    let point = Point::new(1.0, 20.0);

    let actual = find_nearest_recursive(&Node::<&str>::Empty, Axis::Latitude, point);

    assert_eq!(actual, None);
}

#[test]
fn find_nearest_recursive_when_children_are_empty_returns_parent() {
    let node = Node::new_area(Item::new(20.0, 70.0, "parent"), Node::Empty, Node::Empty);
    let point = Point::new(20.0 + 3.0, 70.0 + 4.0);

    let actual = find_nearest_recursive(&node, Axis::Latitude, point);

    assert_eq!(actual, Some(Nearest::new(25.0, &Item::new(20.0, 70.0, "parent"))));
}

#[test]
fn find_nearest_recursive_with_same_point_return_item() {
    let mut items = vec![Item::new(20.0, 70.0, "parent"),
                         Item::new(10.0, 70.0, "left"),
                         Item::new(30.0, 70.0, "right")];
    let node = Node::build(&mut items);
    let point = Point::new(20.0, 70.0);

    let actual = find_nearest_recursive(&node, Axis::Latitude, point);

    assert_eq!(actual, Some(Nearest::new(0.0, &Item::new(20.0, 70.0, "parent"))));
}

#[test]
fn find_nearest_recursive_when_point_on_right_and_near_to_right_child_returns_right_child()
{
    let mut items = vec![Item::new(20.0, 70.0, "parent"),
                         Item::new(10.0, 70.0, "left"),
                         Item::new(30.0, 70.0, "right")];
    let node = Node::build(&mut items);
    let point = Point::new(30.0 + 3.0, 70.0 + 4.0);

    let actual = find_nearest_recursive(&node, Axis::Latitude, point);
    assert_eq!(actual, Some(Nearest::new(25.0, &Item::new(30.0, 70.0, "right"))));
}

#[test]
fn find_nearest_recursive_when_point_on_right_and_near_to_left_child_returns_left_child()
{
    let mut items = vec![Item::new(20.0, 60.0, "parent"),
                         Item::new(18.0, 70.0, "left"),
                         Item::new(30.0, 70.0, "right")];
    let node = Node::build(&mut items);
    let point = Point::new(18.0 + 3.0, 70.0 + 4.0);

    let actual = find_nearest_recursive(&node, Axis::Latitude, point);
    assert_eq!(actual, Some(Nearest::new(25.0, &Item::new(18.0, 70.0, "left"))));
}
