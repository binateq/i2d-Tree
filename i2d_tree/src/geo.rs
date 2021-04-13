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

    assert!(point == same);
    assert!(point != different);
}
