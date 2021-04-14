use std::cmp::Ordering;
use std::ops::RangeFrom;
use std::ops::RangeTo;
use super::Item;
use super::Node;
use super::types::Axis;

pub fn build_recursive<T: Copy>(axis: Axis, items: &mut [Item<T>]) -> Node<T> {
    if items.is_empty() {
        Node::Empty
    } else if items.len() == 1 {
        Node::new_area(items[0], Node::Empty, Node::Empty)
    } else {
        sort_by_axis(axis, items);

        let (median, left_range, right_range) = split_in_halves(&items);
        let next_axis = axis.next();
        let left = build_recursive(next_axis, &mut items[left_range]);
        let right = build_recursive(next_axis, &mut items[right_range]);

        Node::new_area(median, left, right)
    }
}

#[test]
fn build_recursive_with_empty_array_returns_empty() {
    let mut items = Vec::<Item<&str>>::new();

    let actual = build_recursive::<&str>(Axis::Latitude, &mut items);

    assert_eq!(actual, Node::Empty);
}

#[test]
fn build_recursive_with_1_element_returns_area_without_children() {
    let mut items = vec![Item::new(1.0, 20.0, "foo")];

    let node = build_recursive(Axis::Latitude, &mut items);

    assert_eq!(node.value(), Some("foo"));
    assert_eq!(node.left(), None);
    assert_eq!(node.right(), None);
}

#[test]
fn build_recursive_with_3_elements_returns_area_with_children() {
    let mut nodes = vec![Item::new(3.0, 10.0, "foo"),
                         Item::new(2.0, 10.0, "bar"),
                         Item::new(1.0, 10.0, "baz")];

    let node = build_recursive(Axis::Latitude, &mut nodes);

    assert_eq!(node.value(), Some("bar"));
    assert_eq!(node.left().and_then(Node::value), Some("baz"));
    assert_eq!(node.right().and_then(Node::value), Some("foo"));
}

fn sort_by_axis<T: Copy>(axis: Axis, items: &mut [Item<T>]) {
    match axis {
        Axis::Latitude => items.sort_by(cmp_by_latitude),
        Axis::Longitude => items.sort_by(cmp_by_longitude),
    }
}

#[test]
fn sort_by_axis_with_latitude_sorts_by_latitude() {
    let mut items = vec![Item::new(4.0, 3.0, "foo"),
                         Item::new(2.0, 1.0, "bar"),
                         Item::new(3.0, 2.0, "baz"),
                         Item::new(1.0, 4.0, "qux")];
    sort_by_axis(Axis::Latitude, &mut items);

    let actual = items.iter()
                      .map(|n| n.value);

    assert!(actual.eq(vec!["qux", "bar", "baz", "foo"]));
}

#[test]
fn sort_by_axis_with_longitude_sorts_by_longitude() {
    let mut items = vec![Item::new(4.0, 3.0, "foo"),
                         Item::new(2.0, 1.0, "bar"),
                         Item::new(3.0, 2.0, "baz"),
                         Item::new(1.0, 4.0, "qux")];
    sort_by_axis(Axis::Longitude, &mut items);

    let actual = items.iter()
                      .map(|n| n.value);

    assert!(actual.eq(vec!["bar", "baz", "foo", "qux"]));
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

fn split_in_halves<T: Copy>(items: &[Item<T>]) -> (Item<T>, RangeTo<usize>, RangeFrom<usize>) {
    let median_index = items.len()/2;
    let median = items[median_index];
    let left_range = ..median_index;
    let right_range = median_index + 1..;

    (median, left_range, right_range)
}

#[test]
#[should_panic]
fn split_in_halves_with_empty_panics() {
    let items = Vec::<Item<&str>>::new();

    let (_median, _left_range, _right_range) = split_in_halves(&items);
}

#[test]
fn split_in_halves_with_1_element_returns_median_0_left_0_right_0() {
    let items = vec![Item::new(1.0, 20.0, "foo")];

    let (median, left_range, right_range) = split_in_halves(&items);

    assert_eq!(median.value, "foo");
    assert_eq!(items[left_range].len(), 0);
    assert_eq!(items[right_range].len(), 0);
}

#[test]
fn split_in_halves_with_4_elements_returns_median_2_left_2_right_1() {
    let items = vec![Item::new(4.0, 3.0, "foo"),
                     Item::new(2.0, 1.0, "bar"),
                     Item::new(3.0, 2.0, "baz"),
                     Item::new(1.0, 4.0, "qux")];

    let (median, left_range, right_range) = split_in_halves(&items);

    assert_eq!(median.value, "baz");
    assert_eq!(items[left_range].len(), 2);
    assert_eq!(items[right_range].len(), 1);
}

#[test]
fn split_in_halves_with_5_elements_returns_median_2_left_2_right_2() {
    let items = vec![Item::new(4.0, 3.0, "foo"),
                     Item::new(2.0, 1.0, "bar"),
                     Item::new(3.0, 2.0, "baz"),
                     Item::new(1.0, 4.0, "qux"),
                     Item::new(6.0, 6.0, "bat")];

    let (median, left_range, right_range) = split_in_halves(&items);

    assert_eq!(median.value, "baz");
    assert_eq!(items[left_range].len(), 2);
    assert_eq!(items[right_range].len(), 2);
}
