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

## Demo

<p>
    <img src="img/irq_example.png" width="100%" style="margin-bottom: 10px">
    <em>Example IRQ result (red) of lines (blue) and a halfplane (green).</em>
</p>

To get a visual demo of the current progress and to get an idea of IRQ run the following command:
```sh
cargo run --bin line-halfplane-demo
```
The demo currently shows intersection range querries for lines and halfplanes.
But there exist many other reasonable shapes and structures other than lines and halfplanes that can be used instead in IRQ.

## Extreme Intersection Points

An extreme intersection point (eip) of a line is an outer intersection point, hence all other intersections lie on one side of the point.

<p>
    <img src="img/eip.svg" width="100%" style="margin-bottom: 10px">
    <em>The red points are eip's of the green line.</em>
</p>

The computation of all eip's is a key part of IRQ.
<br>
We have implemented two algorithms:
- Block algorithm: $\mathcal{O}(n \log^2 n)$
- Envelope algorithm: $\mathcal{O}(n \log n)$ ($\mathcal{O}(n)$ if lines are sorted by slope)

To check correctness we also implemented a simple $\mathcal{O}(n^2)$ brute force algorithm that computes all intersections and checks whether it is an eip of one of the intersecting lines.
```sh
cargo test eip_correctness
```
It creates multiple random instances, applies the algorithms and verifies the result by comparison to the brute force approach.

For further information on the algorithms look at the report [eip.pdf](reports/eip.pdf).