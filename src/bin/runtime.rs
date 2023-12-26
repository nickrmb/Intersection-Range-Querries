use irq::*;
use rand::Rng;
use std::time::Instant;

fn main() {

    let mut alg_time: Vec<f64> = Vec::new();
    let mut force_time: Vec<f64> = Vec::new();

    println!("len,algorithm,force");

    for i in 2..=200 {

        let k = i * i;

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
        let _ = compute_eip(&mut lines);
        let t2 = Instant::now();
        let _ = force_eip(&lines);
        let t3 = Instant::now();

        let alg_time_cur = t2.duration_since(t1).as_secs_f64() * 1000.0;
        let force_time_cur = t3.duration_since(t2).as_secs_f64() * 1000.0;

        alg_time.push(alg_time_cur);
        force_time.push(force_time_cur);

        eprintln!("Computed {i} in {} ms", (alg_time_cur + force_time_cur).floor());

        println!("{k},{},{}", alg_time_cur, force_time_cur);
    }

}
