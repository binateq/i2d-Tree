use super::Point;
use super::Item;

macro_rules! items {
    ( $( $latitude: expr, $longitude: expr, $value: expr );* ) => {
        <[_]>::into_vec(Box::new([ $(Item::new($latitude, $longitude, $value)),* ]))
    };
}

#[test]
fn point_new_fill_fields() {
    let point = Point::new(10.0, 200.0);

    assert_eq!(point.latitude, 10.0);
    assert_eq!(point.longitude, 200.0);
}

#[test]
fn point_eq_checks_for_equality() {
    let point = Point::new(1.0, 20.0);
    let same = Point::new(1.0, 20.0);
    let different = Point::new(2.0, 10.0);

    assert!(point == same);
    assert!(point != different);
}

#[test]
fn item_new_fills_fields() {
    let item = Item::new(1.0, 20.0, "foo");

    assert_eq!(item.point, Point::new(1.0, 20.0));
    assert_eq!(item.value, "foo");
}
