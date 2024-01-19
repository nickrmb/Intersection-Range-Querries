use crate::*;

pub fn force(lines: &mut Vec<Line>, selected: Vec<usize>, hp: &HalfPlane, sorted: bool) -> Vec<Vec<usize>> {
    if !sorted {
        lines.sort_by(|a, b| a.idx.cmp(&b.idx));
    }

    let mut result: Vec<Vec<usize>> = vec![vec![]; lines.len()];

    for i in 0..selected.len() {
        let mut intersects_with: Vec<usize> = Vec::new();

        for j in 0..selected.len() {
            if i == j {
                continue;
            }
            let l_i = &lines[selected[i]];
            let l_j = &lines[selected[j]];
            
            let x = match l_i.intersection_with_line(l_j) {
                Some(x) => x,
                None => continue
            };
            let y = l_i.y_at(x);

            if hp.contains_point(x, y) {
                intersects_with.push(selected[j]);
            }
        }
        result[selected[i]] = intersects_with;
    }

    result
}