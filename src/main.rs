use irq::*;

fn main() {
    let mut lines = vec![
        Line {
            m: -0.07505518763796909,
            b: 720.0198777221949,
            idx: 0
        },
        Line {
            m: 0.32707774798927614,
            b: 489.52369203132855,
            idx: 1
        },
        Line {
            m: 0.060514372163388806,
            b: 457.03228450434653,
            idx: 2
        },
        Line {
            m: 1.38,
            b: 150.6133435058594,
            idx: 3
        },
        Line {
            m: 0.7459727385377943,
            b: -35.13629713850247,
            idx: 4
        }
    ];

    // println!("{:?}", intersections(&lines));

    println!("{:?}", eip::block_algorithm(&mut lines));
}
