use serde::{Deserialize, Serialize};

pub mod eip;
pub mod selector;
pub mod intersections;
pub mod querry;

#[derive(Serialize, Deserialize, Debug)]
pub struct Line {
    pub m: f64,
    pub b: f64,
    pub idx: usize,
}

impl Clone for Line {
    fn clone(&self) -> Self {
        Self {
            m: self.m.clone(),
            b: self.b.clone(),
            idx: self.idx.clone(),
        }
    }
}

impl Line {
    pub fn new(m: f64, b: f64, idx: usize) -> Line {
        Line { m, b, idx }
    }

    pub fn intersection_with_line(&self, other: &Line) -> Option<f64> {
        let mdiff = self.m - other.m;
        if mdiff == 0.0 {
            return None;
        }
        Some((other.b - self.b) / mdiff)
    }

    pub fn y_at(&self, x: f64) -> f64 {
        self.m * x + self.b
    }
}

pub struct HalfPlane<'a> {
    pub boundary: &'a Line,
    pub bounds_above: bool
}

impl<'a> HalfPlane<'a> {

    pub fn new(boundary: &'a Line, bounds_above: bool) -> HalfPlane {
        HalfPlane {boundary, bounds_above}
    }

    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        let y_bound = self.boundary.m * x + self.boundary.b;
        if self.bounds_above {
            return y_bound >= y;
        }
        return y_bound <= y;
    }

}
