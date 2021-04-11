use core::cmp::Ordering;
use core::ops::Index;
use super::inner::Axis;
use super::inner::Axis::{Latitude, Longitude};
use super::inner::Nearest;
use super::geo::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Item<T: Copy> {
    pub point: Point,
    pub value: T,
}

impl<T: Copy> Item<T> {
    pub fn new(latitude: f64, longitude: f64, value: T) -> Item<T> {
        Item { point: Point::new(latitude, longitude), value }
    }
}

#[test]
fn item_new_fills_fields() {
    let item = Item::new(1.0, 20.0, "foo");

    assert_eq!(item.point, Point::new(1.0, 20.0));
    assert_eq!(item.value, "foo");
}

#[derive(Debug, PartialEq)]
pub enum Node<T: Copy> {
    Empty,
    Area { item: Item<T>, left: Box<Node<T>>, right: Box<Node<T>> }
}

use Node::{Area, Empty};

impl<T: Copy> Node<T> {
    pub fn build(items: &mut [Item<T>]) -> Self {
        build_recursive(Latitude, items)
    }

    pub fn upsert(&mut self, item: Item<T>) {
        upsert_recursive(Latitude, self, item);
    }

    pub fn find_nearest(&self, point: Point) -> Option<Item<T>> {
        match find_nearest_recursive(self, Latitude, point) {
            None => None,
            Some(nearest) => Some(*nearest.item),
        }
    }

    fn new_area(item: Item<T>, left: Node<T>, right: Node<T>) -> Node<T> {
        Area { item, left: Box::new(left), right: Box::new(right) }
    }
}

fn build_recursive<T: Copy>(axis: Axis, items: &mut [Item<T>]) -> Node<T> {
    if items.is_empty() {
        Empty
    } else if items.len() == 1 {
        Node::new_area(items[0], Empty, Empty)
    } else {
        sort_by_axis(axis, items);
        
        let next_axis = axis.next();
        let median_index = items.len() / 2;
        let left = build_recursive(next_axis, &mut items[..median_index]);
        let right = build_recursive(next_axis, &mut items[median_index + 1..]);

        Node::new_area(items[median_index], left, right)
    }
}

fn sort_by_axis<T: Copy>(axis: Axis, items: &mut [Item<T>]) {
    match axis {
        Latitude => items.sort_by(cmp_by_latitude),
        Longitude => items.sort_by(cmp_by_longitude),
    }
}

fn cmp_by_latitude<T: Copy>(a: &Item<T>, b: &Item<T>) -> Ordering {
    a.point.latitude.partial_cmp(&b.point.latitude).unwrap()
}

#[test]
fn cmp_by_latitude_when_a_is_north_of_b_returns_greater() {
    let more_north = Item::new(1.0, 20.0, 300);
    let more_south = Item::new(9.0, 23.0, 400);

    let actual = cmp_by_latitude(&more_south, &more_north);

    assert_eq!(actual, Ordering::Greater);
}

fn cmp_by_longitude<T: Copy>(a: &Item<T>, b: &Item<T>) -> Ordering {
    a.point.longitude.partial_cmp(&b.point.longitude).unwrap()
}

#[test]
fn cmp_by_longitude_when_a_is_east_of_b_returns_greater() {
    let more_west = Item::new(1.0, 21.0, 300);
    let more_east = Item::new(0.0, 27.0, 400);

    let actual = cmp_by_longitude(&more_west, &more_east);

    assert_eq!(actual, Ordering::Less);
}

impl Index<Axis> for Point {
    type Output = f64;
    
    fn index(&self, axis: Axis) -> &<Self as std::ops::Index<Axis>>::Output {
        match axis {
            Latitude => &self.latitude,
            Longitude => &self.longitude,
        }
    }
}

#[test]
fn point_index_returns_valid_components() {
    let point = Point::new(1.0, 20.0);

    assert_eq!(point[Latitude], 1.0);
    assert_eq!(point[Longitude], 20.0);
}

fn upsert_recursive<T: Copy>(axis: Axis, node: &mut Node<T>, new_item: Item<T>) {
    match node {
        Empty =>
            *node = Node::new_area(new_item, Empty, Empty),
        Area { ref mut item, .. } if item.point == new_item.point =>
            item.value = new_item.value,
        Area { item, left, right } => {
            if new_item.point[axis] >= item.point[axis] {
                upsert_recursive(axis.next(), right, new_item);
            } else {
                upsert_recursive(axis.next(), left, new_item);
            }
        }
    }
}

fn find_nearest_recursive<'a, T: Copy>(node: &'a Node<T>, axis: Axis, point: Point) -> Option<Nearest<'a, T>> {
    match node {
        Empty =>
            None,
        Area { item, .. } if item.point.distance(point) == 0.0 =>
            Some(Nearest::new(0.0, item)),
        Area { item, left, right } => {
            let parent_nearest = Nearest::new(item.point.distance(point), item);

            let child = if point[axis] >= item.point[axis] { right } else { left };
            let child_nearest = find_nearest_recursive(child, axis.next(), point);
            let mut nearest = parent_nearest.min(child_nearest);

            let need_check_other_child = if point[axis] >= item.point[axis] {
                point[axis] - item.point[axis] < nearest.distance
            } else {
                item.point[axis] - point[axis] < nearest.distance
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! items {
        ( $( $latitude: expr, $longitude: expr, $value: expr );* ) => {
            <[_]>::into_vec(Box::new([ $(Item::new($latitude, $longitude, $value)),* ]))
        };
    }

    impl Node<&str> {
        fn value(&self) -> Option<&str> {
            match self {
                Empty => None,
                Area { item : Item { value, .. }, .. } => Some(*value),
            }
        }
    
        fn left(&self) -> Option<&Node<&str>> {
            match self {
                Empty => None,
                Area { left, .. } if **left == Empty => None,
                Area { left, .. } => Some(left),
            }
        }
    
        fn right(&self) -> Option<&Node<&str>> {
            match self {
                Empty => None,
                Area { right, .. } if **right == Empty => None,
                Area { right, .. } => Some(right),
            }
        }
    }

    #[test]
    fn node_new_area_fills_fields() {
        let node = Node::new_area(Item::new(10.0, 20.0, "foo"), Empty, Empty);

        assert_eq!(node.value(), Some("foo"));
        assert_eq!(node.left(), None);
        assert_eq!(node.right(), None);
    }

    #[test]
    fn build_recursive_with_empty_array_returns_empty() {
        let mut items = items![];

        let actual = build_recursive::<&str>(Latitude, &mut items);

        assert_eq!(actual, Empty);
    }

    #[test]
    fn build_recursive_with_1_element_returns_area_without_children() {
        let mut items = items![1.0, 20.0, "foo"];

        let node = build_recursive(Latitude, &mut items);

        assert_eq!(node.value(), Some("foo"));
        assert_eq!(node.left(), None);
        assert_eq!(node.right(), None);
    }

    #[test]
    fn build_recursive_with_3_elements_returns_area_with_children() {
        let mut nodes = items![3.0, 10.0, "foo";
                               2.0, 10.0, "bar";
                               1.0, 10.0, "baz"];

        let node = build_recursive(Latitude, &mut nodes);

        assert_eq!(node.value(), Some("bar"));
        assert_eq!(node.left().and_then(Node::value), Some("baz"));
        assert_eq!(node.right().and_then(Node::value), Some("foo"));
    }

    #[test]
    fn sort_by_axis_with_latitude_sorts_by_latitude() {
        let mut items = items![4.0, 3.0, "foo";
                               2.0, 1.0, "bar";
                               3.0, 2.0, "baz";
                               1.0, 4.0, "qux"];
        sort_by_axis(Latitude, &mut items);

        let actual = items.iter()
                          .map(|n| n.value);

        assert!(actual.eq(vec!["qux", "bar", "baz", "foo"]));
    }

    #[test]
    fn sort_by_axis_with_longitude_sorts_by_longitude() {
        let mut items = items![4.0, 3.0, "foo";
                               2.0, 1.0, "bar";
                               3.0, 2.0, "baz";
                               1.0, 4.0, "qux"];
        sort_by_axis(Longitude, &mut items);

        let actual = items.iter()
                          .map(|n| n.value);

        assert!(actual.eq(vec!["bar", "baz", "foo", "qux"]));
    }

    #[test]
    fn upsert_recursive_with_empty_changes_item_to_area() {
        let mut node = Node::<&str>::Empty;

        upsert_recursive(Latitude, &mut node, Item::new(20.0, 70.0, "foo"));

        assert_eq!(node.value(), Some("foo"));
    }

    #[test]
    fn upsert_recursive_with_same_place_changes_value() {
        let mut node = Node::new_area(Item::new(20.0, 70.0, "foo"), Empty, Empty);

        upsert_recursive(Latitude, &mut node, Item::new(20.0, 70.0, "bar"));

        assert_eq!(node.value(), Some("bar"));
    }

    #[test]
    fn upsert_recursive_with_more_north_item_inserts_it_to_right() {
        let mut node = Node::new_area(Item::new(20.0, 70.0, "foo"), Empty, Empty);

        upsert_recursive(Latitude, &mut node, Item::new(30.0, 70.0, "bar"));

        assert_eq!(node.right().and_then(Node::value), Some("bar"));
    }

    #[test]
    fn upsert_recursive_with_more_south_item_inserts_it_to_left() {
        let mut node = Node::new_area(Item::new(20.0, 70.0, "foo"), Empty, Empty);

        upsert_recursive(Latitude, &mut node, Item::new(10.0, 70.0, "baz"));

        assert_eq!(node.left().and_then(Node::value), Some("baz"));
    }

    #[test]
    fn upsert_recursive_with_left_changes_left() {
        let mut items = items![20.0, 70.0, "parent";
                               10.0, 70.0, "left";
                               30.0, 70.0, "right"];
        let mut node = Node::build(&mut items);

        upsert_recursive(Latitude, &mut node, Item::new(10.0, 70.0, "foo"));

        assert_eq!(node.left().and_then(Node::value), Some("foo"));
    }

    #[test]
    fn upsert_recursive_with_right_changes_left() {
        let mut items = items![20.0, 70.0, "parent";
                               10.0, 70.0, "left";
                               30.0, 70.0, "right"];
        let mut node = Node::build(&mut items);

        upsert_recursive(Latitude, &mut node, Item::new(30.0, 70.0, "bar"));

        assert_eq!(node.right().and_then(Node::value), Some("bar"));
    }

    #[test]
    fn find_nearest_recursive_with_empty_returns_none() {
        let point = Point::new(1.0, 20.0);

        let actual = find_nearest_recursive(&Node::<&str>::Empty, Latitude, point);

        assert_eq!(actual, None);
    }

    #[test]
    fn find_nearest_recursive_when_children_are_empty_returns_parent() {
        let node = Node::new_area(Item::new(20.0, 70.0, "parent"), Empty, Empty);
        let point = Point::new(20.0 + 3.0, 70.0 + 4.0);

        let actual = find_nearest_recursive(&node, Latitude, point);

        assert_eq!(actual, Some(Nearest::new(5.0, &Item::new(20.0, 70.0, "parent"))));
    }

    #[test]
    fn find_nearest_recursive_with_same_point_return_item() {
        let mut items = items![20.0, 70.0, "parent";
                               10.0, 70.0, "left";
                               30.0, 70.0, "right"];
        let node = Node::build(&mut items);
        let point = Point::new(20.0, 70.0);

        let actual = find_nearest_recursive(&node, Latitude, point);

        assert_eq!(actual, Some(Nearest::new(0.0, &Item::new(20.0, 70.0, "parent"))));
    }

    #[test]
    fn find_nearest_recursive_when_point_on_right_and_near_to_right_child_returns_right_child()
    {
        let mut items = items![20.0, 70.0, "parent";
                               10.0, 70.0, "left";
                               30.0, 70.0, "right"];
        let node = Node::build(&mut items);
        let point = Point::new(30.0 + 3.0, 70.0 + 4.0);

        let actual = find_nearest_recursive(&node, Latitude, point);
        assert_eq!(actual, Some(Nearest::new(5.0, &Item::new(30.0, 70.0, "right"))));
    }

    #[test]
    fn find_nearest_recursive_when_point_on_right_and_near_to_left_child_returns_left_child()
    {
        let mut items = items![20.0, 60.0, "parent";
                               18.0, 70.0, "left";
                               30.0, 70.0, "right"];
        let node = Node::build(&mut items);
        let point = Point::new(18.0 + 3.0, 70.0 + 4.0);

        let actual = find_nearest_recursive(&node, Latitude, point);
        assert_eq!(actual, Some(Nearest::new(5.0, &Item::new(18.0, 70.0, "left"))));
    }
}
