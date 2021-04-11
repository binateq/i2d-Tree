use super::kd_tree::Item;

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
    pub distance: f64,
    pub item: &'a Item<T>,
}

impl<'a, T: Copy> Nearest<'a, T> {
    pub fn new(distance: f64, item: &'a Item<T>) -> Nearest<'a, T> {
        Nearest { distance, item }
    }

    pub fn min(&self, other: Option<Nearest<'a, T>>) -> Nearest<'a, T> {
        match other {
            None => *self,
            Some(other) if self.distance < other.distance => *self,
            Some(other) => other,
        }
    }
}

#[test]
fn nearest_new_fills_fiels() {
    let item = Item::new(1.0, 20.0, "foo");
    let nearest = Nearest::new(100.0, &item);

    assert_eq!(nearest.distance, 100.0);
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