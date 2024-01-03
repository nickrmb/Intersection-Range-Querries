# IRQ - Intersection Range Querries

<div align="center">
<pre>
 ______      _______        ______  
|      \    |       \      /      \ 
 \▓▓▓▓▓▓    | ▓▓▓▓▓▓▓\    |  ▓▓▓▓▓▓\
  | ▓▓      | ▓▓__| ▓▓    | ▓▓  | ▓▓
  | ▓▓      | ▓▓    ▓▓    | ▓▓  | ▓▓
  | ▓▓      | ▓▓▓▓▓▓▓\    | ▓▓ _| ▓▓
 _| ▓▓_     | ▓▓  | ▓▓    | ▓▓/ \ ▓▓
|   ▓▓ \    | ▓▓  | ▓▓     \▓▓ ▓▓ ▓▓
 \▓▓▓▓▓▓     \▓▓   \▓▓      \▓▓▓▓▓▓\
                                \▓▓▓
- - - - - - - - - - - - - -
Intersection Range Querries
</pre>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![cargo](https://img.shields.io/badge/Cargo-1.74.1-darkred.svg
)](https://crates.io/)

</div>

## Extreme Intersection Points

An extreme intersection point (eip) of a line is an outer intersection point, hence all other intersections lie on one side of the point.

<p>
    <img src="img/eip.svg" width="100%" style="margin-bottom: 10px">
    <em>The red points are eip's of the green line.</em>
</p>

The computation of all eip's is a key part of IRQ.
<br>
We have implemented a $\mathcal{O}(n \log^2 n)$ algorithm.
To check correctness we can run the following command to get a visual interactive demo:
```sh
cargo run --bin eip-demo
```
Alternatively we can run `cargo test eip::tests::correctness -- --exact` to check correctness.
It creates multiple random instances, applies the algorithm and verifies the result by comparison to a simple $\mathcal{O}(n^2)$ brute-force approach.

For further information on the algorithm look at the report [eip.pdf](reports/eip.pdf).