use super::types::Axis;
use super::Item;
use super::Node;

pub fn upsert_recursive<T: Copy>(axis: Axis, node: &mut Node<T>, new_item: Item<T>) {
    match node {
        Node::Empty =>
            *node = Node::new_area(new_item, Node::Empty, Node::Empty),
        Node::Area { ref mut item, .. } if item.point == new_item.point =>
            item.value = new_item.value,
        Node::Area { item, left, right } => {
            if new_item.point[axis] >= item.point[axis] {
                upsert_recursive(axis.next(), right, new_item);
            } else {
                upsert_recursive(axis.next(), left, new_item);
            }
        }
    }
}

#[test]
fn upsert_recursive_with_empty_changes_item_to_area() {
    let mut node = Node::<&str>::Empty;

    upsert_recursive(Axis::Latitude, &mut node, Item::new(20.0, 70.0, "foo"));

    assert_eq!(node.value(), Some("foo"));
}

#[test]
fn upsert_recursive_with_same_place_changes_value() {
    let mut node = Node::new_area(Item::new(20.0, 70.0, "foo"), Node::Empty, Node::Empty);

    upsert_recursive(Axis::Latitude, &mut node, Item::new(20.0, 70.0, "bar"));

    assert_eq!(node.value(), Some("bar"));
}

#[test]
fn upsert_recursive_with_more_north_item_inserts_it_to_right() {
    let mut node = Node::new_area(Item::new(20.0, 70.0, "foo"), Node::Empty, Node::Empty);

    upsert_recursive(Axis::Latitude, &mut node, Item::new(30.0, 70.0, "bar"));

    assert_eq!(node.right().and_then(Node::value), Some("bar"));
}

#[test]
fn upsert_recursive_with_more_south_item_inserts_it_to_left() {
    let mut node = Node::new_area(Item::new(20.0, 70.0, "foo"), Node::Empty, Node::Empty);

    upsert_recursive(Axis::Latitude, &mut node, Item::new(10.0, 70.0, "baz"));

    assert_eq!(node.left().and_then(Node::value), Some("baz"));
}

#[test]
fn upsert_recursive_with_left_changes_left() {
    let mut items = vec![Item::new(20.0, 70.0, "parent"),
                         Item::new(10.0, 70.0, "left"),
                         Item::new(30.0, 70.0, "right")];
    let mut node = Node::build(&mut items);

    upsert_recursive(Axis::Latitude, &mut node, Item::new(10.0, 70.0, "foo"));

    assert_eq!(node.left().and_then(Node::value), Some("foo"));
}

#[test]
fn upsert_recursive_with_right_changes_left() {
    let mut items = vec![Item::new(20.0, 70.0, "parent"),
                         Item::new(10.0, 70.0, "left"),
                         Item::new(30.0, 70.0, "right")];
    let mut node = Node::build(&mut items);

    upsert_recursive(Axis::Latitude, &mut node, Item::new(30.0, 70.0, "bar"));

    assert_eq!(node.right().and_then(Node::value), Some("bar"));
}
