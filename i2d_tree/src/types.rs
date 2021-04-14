use std::ops::Index;
use super::Item;
use super::Node;
use super::Point;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Axis {
    Latitude,
    Longitude,
}

use Axis::{Latitude, Longitude};

impl Axis {
    pub fn next(&self) -> Self {
        match self {
            Latitude => Longitude,
            Longitude => Latitude,
        }
    }
}

#[test]
fn axis_next_returns_opposite_value() {
    assert_eq!(Latitude.next(), Longitude);
    assert_eq!(Longitude.next(), Latitude);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Nearest<'a, T: Copy> {
    pub square_distance: f64,
    pub item: &'a Item<T>,
}

impl<'a, T: Copy> Nearest<'a, T> {
    pub fn new(square_distance: f64, item: &'a Item<T>) -> Nearest<'a, T> {
        Nearest { square_distance, item }
    }

    pub fn min(&self, other: Option<Nearest<'a, T>>) -> Nearest<'a, T> {
        match other {
            None => *self,
            Some(other) if self.square_distance < other.square_distance => *self,
            Some(other) => other,
        }
    }
}

#[test]
fn nearest_new_fills_fiels() {
    let item = Item::new(1.0, 20.0, "foo");
    let nearest = Nearest::new(100.0, &item);

    assert_eq!(nearest.square_distance, 100.0);
    assert_eq!(nearest.item.value, "foo");
}

#[test]
fn nearest_min_with_none_second_returns_first() {
    let item = Item::new(1.0, 20.0, "foo");
    let first = Nearest::new(100.0, &item);
    let second = None;

    let actual = first.min(second);

    assert_eq!(actual.item.value, "foo");
}

#[test]
fn nearest_min_with_nearer_second_returns_second() {
    let item1 = Item::new(1.0, 20.0, "foo");
    let item2 = Item::new(2.0, 10.0, "bar");
    let first = Nearest::new(100.0, &item1);
    let nearer_second = Some(Nearest::new(80.0, &item2));

    let actual = first.min(nearer_second);

    assert_eq!(actual.item.value, "bar");
}

#[test]
fn nearest_min_with_farer_second_returns_first() {
    let item1 = Item::new(1.0, 20.0, "foo");
    let item2 = Item::new(2.0, 10.0, "baz");
    let first = Nearest::new(100.0, &item1);
    let farer_second = Some(Nearest::new(120.0, &item2));

    let actual = first.min(farer_second);

    assert_eq!(actual.item.value, "foo");
}

impl Node<&str> {
    pub fn value(&self) -> Option<&str> {
        match self {
            Node::Empty => None,
            Node::Area { item : Item { value, .. }, .. } => Some(*value),
        }
    }

    pub fn left(&self) -> Option<&Node<&str>> {
        match self {
            Node::Empty => None,
            Node::Area { left, .. } if **left == Node::Empty => None,
            Node::Area { left, .. } => Some(left),
        }
    }

    pub fn right(&self) -> Option<&Node<&str>> {
        match self {
            Node::Empty => None,
            Node::Area { right, .. } if **right == Node::Empty => None,
            Node::Area { right, .. } => Some(right),
        }
    }
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
