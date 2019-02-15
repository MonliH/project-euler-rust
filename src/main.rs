fn main() {
    println!("{}", multiples_of_3_and_5(1_000));
    println!("{}", even_fibonacci_numbers(4_000_000));
}

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

fn even_fibonacci_numbers(limit: u64) -> u64 {
    // Problem 2
    // Get sum of all even fibonacci numbers up to a certain number
    let mut pre: u64 = 1;
    let mut pre_pre: u64 = 0;
    let mut sum: u64 = 0;
    while pre < limit {
        let x: u64 = pre;
        if pre % 2 == 0 {
            sum += pre;
        }
        pre += pre_pre;
        pre_pre = x;
    }
    sum
}
