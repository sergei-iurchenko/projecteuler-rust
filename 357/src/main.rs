extern crate time;
extern crate test;

use std::num::{Float};
use time::precise_time_s;
use std::collections::{BitVec, BitSet};
use std::iter;
use test::Bencher;

fn get_primes_below(n: usize)-> BitSet {
    let mut sieve = BitVec::from_elem(n, true);
    let stop = sieve.len();
    for i in iter::range_step(3, (n as f64).sqrt() as usize + 1, 2) {
        if sieve[i] == true {
            for j in iter::range_step(i * i, stop, 2 * i) {
                sieve.set(j, false);
            }
        }
    }
    let mut result = Box::new(BitSet::with_capacity(n/15));
    result.insert(2);
    for i in iter::range_step(3, n, 2) {
        if sieve[i] == true {
            result.insert(i);
        };
    }
    *result
}

fn check(n: usize, primes: &BitSet) -> bool {
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
    //println!("{} sec.", precise_time_s() - start_time);
    let mut sum = 7usize;
    for x in primes.iter() {
        let _x = x - 1;
        if check(_x, &primes) == true {
            sum += _x;
        }
    }

    //println!("{} sec.", precise_time_s() - start_time);
    sum
}

fn main() {
    let n = 1000_000_00usize;
    let result = find(n);
    println!("answer = {}", result);
}

#[test]
fn it_works() {
    let n = 1000_000_00usize;
    assert!(find(n) == 1739023853137usize);
}

#[bench]
fn bench_it(b: &mut Bencher) {
    b.iter(|| find(1000));
}
