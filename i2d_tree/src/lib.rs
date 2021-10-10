extern crate approx;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

impl Point {
    pub fn new (latitude: f64, longitude: f64) -> Point {
        Point { latitude, longitude }
    }
}

#[test]
fn point_new_fill_fields() {
    let point = Point::new(10.0, 200.0);

    assert_eq!(point.latitude, 10.0);
    assert_eq!(point.longitude, 200.0);
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.latitude == other.latitude && self.longitude == other.longitude
    }
}

#[test]
fn point_eq_checks_for_equality() {
    let point = Point::new(1.0, 20.0);
    let same = Point::new(1.0, 20.0);
    let different = Point::new(2.0, 10.0);

    assert_eq!(point, same);
    assert_ne!(point, different);
}

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

mod types;
use types::Axis;

mod building;
use building::build_recursive;

mod updating;
use updating::upsert_recursive;

mod finding;
use finding::find_nearest_recursive;

impl<T: Copy> Node<T> {
    pub fn build(items: &mut [Item<T>]) -> Self {
        build_recursive(Axis::Latitude, items)
    }

    pub fn upsert(&mut self, item: Item<T>) {
        upsert_recursive(Axis::Latitude, self, item);
    }

    pub fn find_nearest(&self, point: Point) -> Option<Item<T>> {
        find_nearest_recursive(self, Axis::Latitude, point).map(|nearest| *nearest.item)
    }

    fn new_area(item: Item<T>, left: Node<T>, right: Node<T>) -> Node<T> {
        Node::Area { item, left: Box::new(left), right: Box::new(right) }
    }
}

#[test]
fn node_new_area_fills_fields() {
    let node = Node::new_area(Item::new(10.0, 20.0, "foo"), Node::Empty, Node::Empty);

    assert_eq!(node.value(), Some("foo"));
    assert_eq!(node.left(), None);
    assert_eq!(node.right(), None);
}
