# rust-euler

[![Build Status](https://travis-ci.com/cameronp98/rust-euler.svg?branch=master)](https://travis-ci.com/cameronp98/rust-euler)

Easily download problems from project euler and generate an empty solution program for each.
Some common functions are provided, e.g. prime factorization.

Run `$ cargo run` to download the problems into the examples directory


The empty solution program is based on `examples/solution_templates.rs` and looks something like this:

```rust
/*
[https://projecteuler.net/problem=3]

Largest prime factor

The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143
?
*/

fn solution() -> u64 {
    42
}

fn main() {
    println!("{}", solution());
}
```


You can run the solution like this:
`$cargo run --example p001`


The library for common functions is called `euler` as defined in `Cargo.toml`.
You can add extend it by adding your code to `src/lib.rs`.
These functions can be used like you would any other library:

```rust
use euler::prime::factors;

for f in prime_factors(12345) {
    println!("{}", factor);
}
```