#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

impl Point {
    pub fn new (latitude: f64, longitude: f64) -> Point {
        Point { latitude, longitude }
    }

    pub fn distance(&self, other: Point) -> f64 {
        (self.latitude - other.latitude).hypot(self.longitude - other.longitude)
    }
}

#[test]
fn point_new_fill_fields() {
    let point = Point::new(10.0, 200.0);

    assert_eq!(point.latitude, 10.0);
    assert_eq!(point.longitude, 200.0);
}

#[test]
fn point_distance_for_3_and_4_returns_5() {
    let point1 = Point::new(10.0, 200.0);
    let point2 = Point::new(10.0 + 3.0, 200. + 4.0);

    let actual = point1.distance(point2);

    use approx::AbsDiffEq;

    assert!(actual.abs_diff_eq(&5.0, f64::default_epsilon()));
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
