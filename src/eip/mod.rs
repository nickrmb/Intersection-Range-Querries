use crate::*;

mod block;
pub use block::*;

pub fn force_eip(lines: &Vec<Line>) -> (Vec<f64>,Vec<f64>) {
    let n = lines.len();
    let mut left: Vec<f64> = vec![0.0; n];
    let mut right: Vec<f64> = vec![0.0; n];

    for (i, l1) in lines.iter().enumerate() {
        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        for l2 in &lines[0..i] {
            if let Some(x) = l1.intersection_with_line(l2) {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
        }
        for l2 in &lines[i + 1..] {
            if let Some(x) = l1.intersection_with_line(l2) {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
        }
        left[lines[i].idx] = x_min;
        right[lines[i].idx] = x_max;
    }

    (left,right)
}

