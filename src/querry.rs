use crate::*;

use crate::HalfPlane;

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn test() {
        let k = 500;
        let mut lines: Vec<Line> = Vec::new();

        let mut rng = rand::thread_rng();
        for i in 0..(k+1) {
            let m: f64 = rng.gen();
            let m = (m * 2.0 - 1.0) * 10000.0;
            let b: f64 = rng.gen();
            let b = (b * 2.0 - 1.0) * 10000.0;
            lines.push(Line { m, b, idx: i });
        }


        let last = lines.pop().unwrap();
        let hp = HalfPlane::new(&last, true);
        
        let result1 = line_halfplane(&mut lines, &hp);
        let result2 = _force_line_halfplane(&mut lines, &hp, true);

        assert_eq!(result1.len(), result2.len());

        for i in 0..result1.len() {
            let r1 = &result1[i];
            let r2 = &result2[i];
            assert_eq!(r1.len(), r2.len());
            
            for j in 0..r1.len(){
                assert_eq!(r1[j], r2[j]);
            }
        }
    }
}

pub fn line_halfplane(lines: &mut Vec<Line>, hp: &HalfPlane) -> Vec<Vec<usize>> {
    let eips = crate::eip::envelope_algorithm(lines);
    let in_hp = crate::selector::force(hp, lines, &eips, false);
    let result: Vec<Vec<usize>> = crate::intersections::force(lines, in_hp, hp, true);

    return result;
}

fn _force_line_halfplane(lines: &mut Vec<Line>, hp: &HalfPlane, sorted: bool) -> Vec<Vec<usize>> {
    let mut all: Vec<usize> = vec![0; lines.len()];
    for i in 1..all.len() {
        all[i] = i;
    }
    return crate::intersections::force(lines, all, hp, sorted);
}
