use crate::*;
use priority_queue::PriorityQueue;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::time::Instant;
    use eip::force_eip;

    const DELTA: f64 = 0.00000001;

    fn equal(x1: f64, x2: f64) -> bool {
        x1 - DELTA <= x2 && x1 + DELTA >= x2
        // x1 == x2
    }

    #[test]
    fn correctness() {
        let n = 100;
        let k = 100;
        let mut alg_time: u128 = 0;
        let mut force_time: u128 = 0;
        for _ in 0..n {

            let mut lines: Vec<Line> = Vec::new();

            let mut rng = rand::thread_rng();
            for i in 0..k {
                let m: f64 = rng.gen();
                let m = (m * 2.0 - 1.0) * 10000.0;
                let b: f64 = rng.gen();
                let b = (b * 2.0 - 1.0) * 10000.0;
                lines.push(Line { m, b, idx: i });
            }

            let t1 = Instant::now();
            let result1 = block_algorithm(&mut lines);
            let t2 = Instant::now();
            let result2 = force_eip(&lines);
            let t3 = Instant::now();

            alg_time += t2.duration_since(t1).as_micros();
            force_time += t3.duration_since(t2).as_micros();

            for i in 0..k {
                assert!(
                    equal(result1.0[i], result2.0[i]),
                    "{} != {}",
                    result1.0[i],
                    result2.0[i]
                );
                assert!(
                    equal(result1.1[i], result2.1[i]),
                    "{} != {}",
                    result1.1[i],
                    result2.1[i]
                );
            }
        }
        println!("algorithm time: {alg_time} microseconds");
        println!("force time: {force_time} microseconds");
    }
}

pub fn block_algorithm(lines: &mut Vec<Line>) -> (Vec<f64>,Vec<f64>) {
    let left = compute_eip_left(lines);
    
    for i in 0..lines.len() {
        lines[i].m *= -1.0;
    }
    let mut right = compute_eip_left(lines);
    for i in 0..lines.len() {
        lines[i].m *= -1.0;
        right[i] = -right[i];
    }

    (left,right)
}

fn compute_eip_left(lines: &mut Vec<Line>) -> Vec<f64> {
    let n = lines.len();

    // sort by slope
    lines.sort_by(|a, b| a.m.total_cmp(&b.m));

    // potential intersection with (above or below)
    let mut piw: Vec<usize> = vec![0; n];

    // pq setup

    let mut pq: PriorityQueue<usize, F64> = PriorityQueue::new();

    if let Some(x) = lines[0].intersection_with_line(&lines[1]) {
        pq.push(0, F64(-x));
    } else {
        pq.push(0, F64(f64::MIN));
    }
    piw[0] = 1;

    for i in 1..n - 1 {
        // find intersections with neighboring lines
        let x_before = match lines[i].intersection_with_line(&lines[i - 1]) {
            Some(x) => x,
            None => f64::MAX,
        };
        let x_after = match lines[i].intersection_with_line(&lines[i + 1]) {
            Some(x) => x,
            None => f64::MAX,
        };

        // both parallel parallel
        if x_before == x_after && x_after == f64::MAX {
            pq.push(i, F64(f64::MIN));
            piw[i] = i;
            continue;
        }

        // get more left intersection line
        if x_before < x_after {
            pq.push(i, F64(-x_before));
            piw[i] = i - 1;
        } else {
            pq.push(i, F64(-x_after));
            piw[i] = i + 1;
        }
    }

    if let Some(x) = lines[n - 1].intersection_with_line(&lines[n - 2]) {
        pq.push(n - 1, F64(-x));
    } else {
        pq.push(n - 1, F64(f64::MIN));
    }
    piw[n - 1] = n - 2;

    // first intersection
    let mut fi: Vec<f64> = vec![f64::MAX; n];

    // block setup
    let mut blocks: Vec<Block> = Vec::new();

    // vector of line states, if is_in[i] != n, then line L_i is in blocks[i]
    let mut is_in: Vec<usize> = vec![n; n];

    // loop

    while let Some((i, F64(x))) = pq.pop() {
        let x = -x;
        assert_eq!(is_in[i], n, "Is not a free line!");

        let li = &lines[i];

        // set intersection point
        fi[lines[i].idx] = x;

        // get intersecting line idx
        let j = piw[i];

        let block: &mut Block;
        let mut block_id;

        if is_in[j] == n {
            // lj is free line
            fi[lines[j].idx] = x;

            // get intersection line
            let lj = &lines[j];

            // remove from pq
            let _o = pq.remove(&j);
            assert_ne!(_o, None, "Free line was not in pq");

            // create new block
            let idx = blocks.len();
            let new_block = if li.m < lj.m {
                Block {
                    upper_envelope: Envelope {
                        seg: vec![(f64::MIN, f64::MAX, &lj)],
                    },
                    lower_envelope: Envelope {
                        seg: vec![(f64::MIN, f64::MAX, &li)],
                    },
                    upper_idx: i.max(j),
                    lower_idx: i.min(j),
                }
            } else {
                Block {
                    upper_envelope: Envelope {
                        seg: vec![(x, f64::MAX, &li)],
                    },
                    lower_envelope: Envelope {
                        seg: vec![(x, f64::MAX, &lj)],
                    },
                    upper_idx: i.max(j),
                    lower_idx: i.min(j),
                }
            };
            blocks.push(new_block);

            block_id = idx;
            block = &mut blocks[idx];

            is_in[i] = idx;
            is_in[j] = idx;
        } else {
            // is block

            is_in[i] = is_in[j]; // add i to the block

            block_id = is_in[j];
            block = &mut blocks[block_id];

            // update block bounds
            block.lower_idx = block.lower_idx.min(i);
            block.upper_idx = block.upper_idx.max(i);

            // if slopes are different new envelope is created
            if lines[j].m != lines[i].m {
                // merge line with block

                if j < i {
                    // update upper envelope
                    if let Some((p_seg, x_seg)) =
                        li.intersection_with_envelope(&block.upper_envelope)
                    {
                        block.upper_envelope.seg.truncate(p_seg + 1);
                        block.upper_envelope.seg[p_seg].1 = x_seg;
                        block.upper_envelope.seg.push((x_seg, f64::MAX, &lines[i]));
                    } else {
                        assert!(false, "should have not happened")
                    }
                } else {
                    // update lower envelope
                    if let Some((p_seg, x_seg)) =
                        li.intersection_with_envelope(&block.lower_envelope)
                    {
                        block.lower_envelope.seg.truncate(p_seg + 1);
                        block.lower_envelope.seg[p_seg].1 = x_seg;
                        block.lower_envelope.seg.push((x_seg, f64::MAX, &lines[i]));
                    } else {
                        assert!(false, "should have not happened")
                    }
                }
            }
        }

        let block_upper_idx = block.upper_idx;

        // check for potential superblock merges

        // check above
        if block.lower_idx > 0 && is_in[block.lower_idx - 1] != n {
            // block above
            let upper_idx = is_in[block.lower_idx - 1];

            // merge with block above
            blocks.push(Block::empty()); // add empty block
            let lower = blocks.swap_remove(block_id); // use swap for constant time
            let upper = &mut blocks[upper_idx];
            Block::merge(upper, lower);

            // change blockid of most down element
            block_id = upper_idx;
            is_in[upper.upper_idx] = block_id;
        }

        // check under
        if block_upper_idx < n - 1 && is_in[block_upper_idx + 1] != n {
            // block under
            let lower_idx = is_in[block_upper_idx + 1];

            // merge with block above
            blocks.push(Block::empty()); // add empty block
            let lower = blocks.swap_remove(lower_idx); // use swap for constant time
            let upper = &mut blocks[block_id];
            Block::merge(upper, lower);

            // change blockid of most up element
            is_in[upper.upper_idx] = block_id;
        }

        let block = &blocks[block_id];

        // update free line intersections above and below with (new) (super)block
        // check line above
        if block.lower_idx > 0 {
            let idx = block.lower_idx - 1;
            let above = &lines[idx];

            let a = above.intersection_with_envelope(&block.upper_envelope);
            if let Some((_, x)) = a {
                let prio = pq.get_priority(&idx);
                if let Some(F64(prio)) = prio {
                    if -x > *prio {
                        pq.change_priority(&idx, F64(-x));
                        piw[idx] = idx + 1;
                    }
                } else {
                    assert!(false, "Shouldnt have happened");
                }
            } else {
                assert!(false, "Shouldnt have happened");
            }
        }

        // check line under
        if block.upper_idx < n - 1 {
            let idx = block.upper_idx + 1;
            let under = &lines[idx];

            let a = under.intersection_with_envelope(&block.lower_envelope);
            if let Some((_, x)) = a {
                let prio = pq.get_priority(&idx);
                if let Some(F64(prio)) = prio {
                    if -x > *prio {
                        pq.change_priority(&idx, F64(-x));
                        piw[idx] = idx - 1;
                    }
                } else {
                    assert!(false, "Shouldnt have happened");
                }
            } else {
                assert!(false, "Shouldnt have happened");
            }
        }
    }

    fi
}
