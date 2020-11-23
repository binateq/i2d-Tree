use chrono::{DateTime, Utc, NaiveDate, TimeZone};
use crate::geo::{Point, Rectangle};

pub struct Leaf {
    pub point: Point,
    pub timestamp: DateTime<Utc>,
}

pub enum Node {
    Leaves(Vec<Leaf>),
    Rectangles(Rectangle, Vec<Node>)
}

impl Leaf {
    pub fn new(latitude: f64, longitude: f64, year: i32, month: u32, day: u32,
               hour: u32, minute: u32, second: u32) -> Leaf {
        let date_time = NaiveDate::from_ymd(year, month, day).and_hms(hour, minute, second);

        Leaf {
            point: Point::new(latitude, longitude),
            timestamp: Utc.from_utc_datetime(&date_time),
        }
    }
}

impl Node {
    pub fn new_leaves(leaves: Vec<Leaf>) -> Node {
        Node::Leaves(leaves)
    }

    pub fn new_rectangles(latitude1: f64, longitude1: f64, latitude2: f64, longitude2: f64,
                          nodes: Vec<Node>) -> Node {
        let rectangle = Rectangle::new(latitude1, longitude1, latitude2, longitude2);
        Node::Rectangles(rectangle, nodes)
    }
}

#[cfg(test)]
mod test {
    use chrono::{Datelike, Timelike};
    use super::Leaf;

    #[test]
    fn leaf_new_when_called_returns_constructed_object() {
        let leaf = Leaf::new(55.0, 37.0, 2020, 11, 22, 23, 19, 55);

        assert_eq!(55.0, leaf.point.latitude);
        assert_eq!(37.0, leaf.point.longitude);
        assert_eq!(2020, leaf.timestamp.year());
        assert_eq!(11, leaf.timestamp.month());
        assert_eq!(22, leaf.timestamp.day());
        assert_eq!(23, leaf.timestamp.hour());
        assert_eq!(19, leaf.timestamp.minute());
        assert_eq!(55, leaf.timestamp.second());
    }
}