# Project Euler Rust

I am currently trying to learn Rust and am trying to familiarize myself with Rust by implementing as many problems in project Euler as I can in it.
**BTW** the code might not be the most elegant, because I have just recently learned Rust and am trying to improve my programming skills in Rust.

Each challenge is written in a function in order (from problem 1 to 2 to 3 to 4 ...) the function names are the snake case of the real name of the problem listed on the website. Inside each function, there will also contain at least two comments explaining what the function does and the problem number. Example:

``` rust
fn multiples_of_3_and_5(limit: u64) -> u64 {
    // Problem 1
    // Get sum of all multiples of 3 and 5 that are below some limit
    let mut sum: u64 = 0;
    for i in 0..limit {
        if i % 3 == 0 {
            sum += i;
        } else if i % 5 == 0 {
            sum += 1
        }
    }
    sum
}
```

The `main()` function will also print the numbers returned from every function and the original value that was in the question (e.g. if the question asks you to sum up to 1000 it will pass 1000 as the argument).

## Thanks

* [Rust](https://www.rust-lang.org/)
* [Project Euler](https://projecteuler.net/)
* [The Book](https://doc.rust-lang.org/book/)
