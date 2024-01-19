use crate::*;

pub fn force(hp: &HalfPlane, lines: &mut Vec<Line>, eips: &(Vec<f64>, Vec<f64>), sorted: bool) -> Vec<usize> {
    if !sorted {
        lines.sort_by(|a, b| a.idx.cmp(&b.idx));
    }

    let mut result: Vec<usize> = Vec::new();

    for i in 0..lines.len() {
        let l = &lines[i];
        let eipl_x = eips.0[i];
        let eipr_x = eips.1[i];

        let eipl_y = l.y_at(eipl_x);
        let eipr_y = l.y_at(eipr_x);

        if hp.contains_point(eipl_x, eipl_y) || hp.contains_point(eipr_x, eipr_y) {
            result.push(i);
        }
    }

    return result;
}
