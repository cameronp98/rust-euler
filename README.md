# project-euler

Easily download problems from project euler and generate an empty solution program for each

Run `$ cargo run` to download the problems into the examples directory


An empty solution program looks something like this:

```rust
/*
[https://projecteuler.net/problem=3]

The prime factors of 13195 are 5, 7, 13
and 29.
What is the largest prime factor of the
number 600851475143 ?

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