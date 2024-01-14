use serde::{Deserialize, Serialize};

pub mod eip;

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
