use std::cmp::Ordering;
use std::f32::consts::PI;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn origin() -> Point {
        return Point { x: 0, y: 0 };
    }

    pub fn to_string(&self) -> String {
        return format!("{}{}{}", self.x.to_string(), ",", self.y.to_string());
    }

    #[allow(dead_code)]
    pub fn from_string(string: String) -> Point {
        let pieces: Vec<i32> = string
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        return Point {
            x: pieces[0],
            y: pieces[1],
        };
    }

    pub fn angle_to(&self, other: &Point) -> f32 {
        let y = self.y - other.y; //reversed because image space is backwards
        let x = other.x - self.x;
        let mut rads = (y as f32).atan2(x as f32);
        if rads < 0f32 {
            rads = ((-1f32 * PI) - rads).abs() + PI;
        }
        rads
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PointDist {
    pub point: Point,
    pub dist: i32,
}

impl Ord for PointDist {
    fn cmp(&self, other: &PointDist) -> Ordering {
        // inverted comparison on dist for min priority queue (Day 10)
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for PointDist {
    fn partial_cmp(&self, other: &PointDist) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
pub struct UTreeNode {
    pub parent: String,
    pub children: Vec<String>,
    pub value: String,
    pub visited: bool,
}

impl UTreeNode {
    pub fn new(parent: String, val: String) -> UTreeNode {
        return UTreeNode {
            parent: parent,
            value: val,
            children: Vec::new(),
            visited: false,
        };
    }
}

pub fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    return (a.x - b.x).abs() + (a.y - b.y).abs();
}

pub fn distance(a: &Point, b: &Point) -> f32 {
    let x = (a.x - b.x).abs();
    let y = (a.y - b.y).abs();
    return ((x * x + y * y) as f32).sqrt();
}

pub fn distance_i32(a: &Point, b: &Point) -> i32 {
    return (distance(a, b) * 1000000f32).floor() as i32;
}