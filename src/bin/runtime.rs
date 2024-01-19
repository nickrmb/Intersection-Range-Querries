use ::irq::*;
use rand::Rng;
use std::time::Instant;

fn main() {

    println!("len,force,block,envelope");

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
        let _ = eip::block_algorithm(&mut lines);
        let t2 = Instant::now();
        let _ = eip::force_eip(&lines);
        let t3 = Instant::now();
        let _ = eip::envelope_algorithm(&mut lines);
        let t4 = Instant::now();

        let block_time_cur = t2.duration_since(t1).as_secs_f64() * 1000.0;
        let force_time_cur = t3.duration_since(t2).as_secs_f64() * 1000.0;
        let env_time_cur = t4.duration_since(t3).as_secs_f64() * 1000.0;

        eprintln!("Computed {i} in {} ms", (block_time_cur + force_time_cur + env_time_cur).floor());

        println!("{k},{},{},{}", force_time_cur, block_time_cur, env_time_cur);
    }

}
