extern crate time;

use time::precise_time_s;
use std::collections::BTreeSet;

fn get_primes_below(max_digit: usize)-> BTreeSet<usize> {
    let mut sieve: Vec<bool> = Vec::with_capacity(max_digit);
    for _ in 0..max_digit {
        sieve.push(true);
    }
    let stop = sieve.len();
    let top = (max_digit as f64).sqrt() as usize + 1;
    for (_, i) in (3..top).enumerate().filter(|&(index, _)| index % 2 == 0) {
        if sieve[i] == true {
            for (_, j) in (i*i..stop).enumerate().filter(|&(index, _)| index % (2 * i) == 0 ) {
                sieve[j] = false;
            }
        }
    }
    let mut result = Box::new(BTreeSet::new());
    result.insert(2);
    for (_, i) in (3..max_digit).enumerate().filter(|&(index, _)| index % 2 == 0) {
        if sieve[i] == true {
            result.insert(i);
        };
    }
    *result
}

fn check(n: usize, primes: &BTreeSet<usize>) -> bool {
    if ![0usize, 2, 8].contains(&(n % 10))
    {
        return false;
    }
    if !primes.contains(&(n / 2 + 2))
    {
        return false;
    }
    for d in 3..((n as f64).sqrt() as usize) {
        if (n % d) == 0
        {
            if !primes.contains(&(d + n / d)) {
                return false;
            }
        }
    }
    return true;
}

fn find(n: usize) -> usize {
    let start_time = precise_time_s();
    let primes = Box::new(get_primes_below(n));
    println!("{} sec.", precise_time_s() - start_time);
    let mut sum = 7usize;
    for x in primes.iter() {
        let _x = x - 1;
        if check(_x, &primes) == true {
            sum += _x;
        }
    }

    println!("{} sec.", precise_time_s() - start_time);
    sum
}

fn main() {
    let n = 1000_000_00usize;
    let result = find(n);
    println!("answer = {}", result);
//    for (index, value) in (3..20).enumerate().filter(|&(index, _)| index % 2 == 0) {
//        println!("{}", value)
//    }
}

#[test]
fn it_works() {
    let n = 1000_000_00usize;
    assert!(find(n) == 1739023853137usize);
}

//#[bench]
//fn bench_it(b: &mut Bencher) {
//    b.iter(|| find(1000));
//}
