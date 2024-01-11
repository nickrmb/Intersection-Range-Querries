use crate::*;
use std::cmp::Ordering::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eip::force_eip;
    use rand::Rng;

    const DELTA: f64 = 0.00000001;

    fn equal(x1: f64, x2: f64) -> bool {
        x1 - DELTA <= x2 && x1 + DELTA >= x2
    }

    #[test]
    fn correctness() {
        for _ in 0..100 {
            let k = 500;
            let mut lines: Vec<Line> = Vec::new();

            let mut rng = rand::thread_rng();
            for i in 0..k {
                let m: f64 = rng.gen();
                let m = (m * 2.0 - 1.0) * 10000.0;
                let b: f64 = rng.gen();
                let b = (b * 2.0 - 1.0) * 10000.0;
                lines.push(Line { m, b, idx: i });
            }

            let result1 = envelope_algorithm(&mut lines);
            let result2 = force_eip(&lines);

            for i in 0..lines.len() {
                assert!(
                    equal(result1.0[i], result2.0[i]),
                    "{} != {}",
                    result1.0[i],
                    result2.0[i]
                );
                assert!(
                    equal(result1.1[i], result2.1[i]),
                    "{} != {}\n{:?}",
                    result1.1[i],
                    result2.1[i],
                    lines
                );
            }
        }
    }
}

#[derive(Debug)]
struct PolyChain<'a> {
    seg: Vec<(&'a Line, f64)>, // line with start point
}

impl<'a> PolyChain<'a> {
    fn new(n: usize) -> PolyChain<'a> {
        PolyChain {
            seg: Vec::with_capacity(n),
        }
    }

    fn next_b(&mut self, next: &'a Line) {
        if self.seg.len() == 0 {
            self.seg.push((next, f64::MIN));
            return;
        }
        loop {
            let (line, x) = self.seg.last().unwrap();
            let intersection = match next.intersection_with_line(line) {
                Some(x) => x,
                None => f64::MIN,
            };

            if intersection < *x {
                // left of segment

                self.seg.pop();
            } else {
                // hit segment

                self.seg.push((next, intersection));
                return;
            }
        }
    }

    fn prev_a(&mut self, prev: &'a Line) -> Vec<(&'a Line, f64)> {
        if self.seg.len() == 0 {
            self.seg.push((prev, f64::MIN));
            return Vec::new();
        }

        let mut instructions: Vec<(&'a Line, f64)> = Vec::new();

        loop {
            let (line, x) = self.seg.last().unwrap();
            let intersection = match prev.intersection_with_line(line) {
                Some(x) => x,
                None => f64::MIN,
            };

            if intersection < *x {
                // left of segment

                instructions.push(self.seg.pop().unwrap());
            } else {
                // hit segment

                self.seg.push((prev, intersection));
                return instructions;
            }
        }
    }

    fn next_a(&mut self, mut instruction: Vec<(&'a Line, f64)>) {
        self.seg.pop();

        while let Some(seg) = instruction.pop() {
            self.seg.push(seg);
        }
    }
}

pub fn envelope_algorithm(lines: &mut Vec<Line>) -> (Vec<f64>, Vec<f64>) {
    let n = lines.len();

    let right = compute_right(lines, false);

    // mirror horizontally
    for i in 0..n {
        lines[i].m = -lines[i].m;
    }
    // change slope order accordingly
    for i in 0..(n / 2) {
        lines.swap(i, n - i - 1);
    }

    let mut left = compute_right(lines, true);

    // mirror horizontally back
    for i in 0..n {
        lines[i].m = -lines[i].m;
        left[i] = -left[i];
    }

    (left, right)
}

pub fn compute_right(lines: &mut Vec<Line>, sorted: bool) -> Vec<f64> {
    let n = lines.len();

    if !sorted {
        lines.sort_by(|a, b| {
            // sort decreasing slope
            match a.m.total_cmp(&b.m) {
                Less => Greater,
                Greater => Less,
                Equal => match a.b.total_cmp(&b.b) {
                    // and decreasing y for same slope
                    Less => Greater,
                    Greater => Less,
                    Equal => Equal,
                },
            }
        });
    }

    let mut a = PolyChain::new(n); // = A_n
    let mut b = PolyChain::new(n); // = B_0

    // instructions to create A_(i+1) from A_i for i > 0
    let mut rev_instructions: Vec<Vec<(&Line, f64)>> = Vec::with_capacity(n);

    // Construct A_0
    for i in 0..n {
        rev_instructions.push(a.prev_a(&lines[n - i - 1]));
    }

    let mut li = vec![f64::MIN; n];
    li[lines[0].idx] = a.seg.last().unwrap().1;

    a.next_a(rev_instructions.pop().unwrap());
    b.next_b(&lines[0]);

    for i in 1..(n - 1) {
        // compute right intersection
        let mut i_a = a.seg.len() - 1;
        let mut i_b = b.seg.len() - 1;

        let a_l_idx = a.seg[i_a].0.idx;
        let b_l_idx = b.seg[i_b].0.idx;

        li[a_l_idx] = li[a_l_idx].max(a.seg[i_a].1);
        li[b_l_idx] = li[b_l_idx].max(b.seg[i_b].1);

        let mut cur_x;
        let mut bound_by_a;
        if a.seg[i_a].1 >= b.seg[i_b].1 {
            cur_x = a.seg[i_a].1;
            bound_by_a = true;
        } else {
            cur_x = b.seg[i_b].1;
            bound_by_a = false;
        }

        loop {
            let x = match a.seg[i_a].0.intersection_with_line(&b.seg[i_b].0) {
                Some(x) => x,
                None => f64::MIN,
            };

            if x >= cur_x {
                // intersection in bound

                li[a_l_idx] = li[a_l_idx].max(x);
                li[b_l_idx] = li[b_l_idx].max(x);
                break;
            } else {
                // intersection out of bound

                if bound_by_a {
                    i_a -= 1;
                } else {
                    i_b -= 1;
                }

                if a.seg[i_a].1 >= b.seg[i_b].1 {
                    cur_x = a.seg[i_a].1;
                    bound_by_a = true;
                } else {
                    cur_x = b.seg[i_b].1;
                    bound_by_a = false;
                }
            }
        }

        a.next_a(rev_instructions.pop().unwrap());
        b.next_b(&lines[i]);
    }

    b.next_b(&lines[n - 1]);
    li[lines.last().unwrap().idx] = b.seg.last().unwrap().1;

    return li;
}
