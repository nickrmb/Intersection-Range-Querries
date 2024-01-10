use serde::{Deserialize, Serialize};

pub mod eip;

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test1() {
        let l1 = Line { m: 0.6, b: 2.0, idx: 1};
        let l2 = Line { m: 0.8, b: 3.0, idx: 2};
        let e = Envelope {
            seg: vec![
                (-1.7976931348623157e308, -4.999999999999998, &l1),
                (-4.999999999999998, 1.7976931348623157e308, &l2),
            ],
        };

        let l3 = Line { m: 0.2, b: 4.0, idx: 3};

        println!("{:?}", l3.intersection_with_envelope(&e));
    }
}

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

    pub fn intersection_with_envelope(&self, env: &Envelope) -> Option<(usize, f64)> {
        // binary search

        let mut l: usize = 0;
        let mut r: usize = env.seg.len() - 1;

        while l <= r {
            let m = (l + r) / 2;
            let (xl, xr, line) = env.seg[m];

            // intersection with segment line
            let x = self.intersection_with_line(&line);

            if let Some(x) = x {
                // has intersection
                if xl <= x && xr >= x {
                    // hit segment
                    return Some((m, x));
                }

                // miss segment
                if x < xl {
                    // search left
                    if m == 0 {
                        break;
                    }
                    r = m - 1;
                } else {
                    // search right
                    l = m + 1;
                }
                continue;
            } else {
                // parallel
                return None;
            }
        }

        None
    }

    pub fn y_at(&self, x: f64) -> f64 {
        self.m * x + self.b
    }
}

#[derive(Debug)]
pub struct Envelope<'a> {
    pub seg: Vec<(f64, f64, &'a Line)>,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub upper_envelope: Envelope<'a>,
    pub lower_envelope: Envelope<'a>,
    pub upper_idx: usize,
    pub lower_idx: usize,
}

impl Envelope<'_> {
    pub fn intersection_with_envelope(&self, other: &Envelope) -> Option<(usize, usize, f64)> {
        // all slopes of other are more steep than self

        // binary search on segments of other

        let mut l: usize = 0;
        let mut r: usize = other.seg.len() - 1;

        while l <= r {
            let m = (l + r) / 2;

            let (xl, xr, line) = other.seg[m];

            let x = line.intersection_with_envelope(self); // hit x

            if let Some((p, x)) = x {
                if x >= xl && x <= xr {
                    return Some((p, m, x));
                }
                if x < xl {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else {
                println!("{:?}\n{:?}", line, self);
                assert!(false, "Should have not happened");
            }
        }

        None
    }
}

impl Block<'_> {
    pub fn merge<'a>(upper: &mut Block<'a>, mut lower: Block<'a>) {
        upper.upper_idx = lower.upper_idx;

        // Update upper envelope

        // upper hit
        let (i, j, x) = match upper
            .upper_envelope
            .intersection_with_envelope(&lower.upper_envelope)
        {
            Some(a) => a,
            None => todo!("Should have not happened"),
        };

        // slice segments
        upper.upper_envelope.seg.truncate(i + 1);
        lower.upper_envelope.seg.drain(0..j);

        // change edge points accordingly
        upper.upper_envelope.seg[i].1 = x;
        lower.upper_envelope.seg[0].0 = x;

        // append envelopes
        upper
            .upper_envelope
            .seg
            .append(&mut lower.upper_envelope.seg);

        // update lower envelope

        // lower hit
        let (i, j, x) = match lower
            .lower_envelope
            .intersection_with_envelope(&upper.lower_envelope)
        {
            Some(a) => a,
            None => todo!("Should have not happened"),
        };

        // slice segments
        lower.lower_envelope.seg.truncate(i + 1);
        upper.lower_envelope.seg.drain(0..j);

        // change edge points accordingly
        lower.lower_envelope.seg[i].1 = x;
        upper.lower_envelope.seg[0].0 = x;

        // append envelopes
        lower
            .lower_envelope
            .seg
            .append(&mut upper.lower_envelope.seg);

        upper.lower_envelope = lower.lower_envelope;
    }

    pub fn empty<'a>() -> Block<'a> {
        Block {
            upper_envelope: Envelope { seg: Vec::new() },
            lower_envelope: Envelope { seg: Vec::new() },
            upper_idx: 0,
            lower_idx: 0,
        }
    }
}

pub fn intersections(lines: &Vec<Line>) -> Vec<f64> {
    let n = lines.len();
    let mut intersections: Vec<f64> = Vec::new();
    for i in 0..n - 1 {
        for j in i + 1..n {
            match lines[i].intersection_with_line(&lines[j]) {
                Some(x) => intersections.push(x),
                None => {}
            }
        }
    }
    intersections
}

#[derive(Debug)]
struct F64(f64);
impl PartialEq for F64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for F64 {}
impl PartialOrd for F64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for F64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
