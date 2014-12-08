extern crate time;
extern crate test;

use std::mem::size_of;
use std::num::{Float, Int};
use test::Bencher;
use time::precise_time_s;
use std::collections::bitv;
use std::collections::HashSet;
use std::iter;

fn get_primes_below(n: uint) -> Vec<uint> {
    let mut sieve = bitv::Bitv::with_capacity(n, true);
    let stop = sieve.len();
    for i in iter::range_step(3, (n as f64).sqrt() as uint + 1u, 2) {
        if sieve[i] == true {
            for j in iter::range_step(i * i, stop, 2 * i) {
                sieve.set(j, false);
            }
        }
    }

    let mut result:Vec<uint> = Vec::with_capacity(n);
    result.push(2);
    for i in iter::range_step(3, n, 2) {
        if sieve[i] == true {
            result.push(i)
        };
    }
    result
}

fn check(n: uint, primes: &HashSet<uint>) -> bool {
    if ![0u, 2, 8].contains(&(n % 10))
    {
        return false;
    }
    if !primes.contains(&(n / 2 + 2))
    {
        return false;
    }
    for d in range(3, (n as f64).sqrt() as uint) {
        if (n % d) == 0
        {
            if !primes.contains(&(d + n / d)) {
                return false;
            }
        }
    }
    return true;
}


fn main() {
    let start_time = precise_time_s();
    let a: uint = 10u.pow(8);
    let primes_vec = get_primes_below(a);
    println!("{} sec.", precise_time_s() - start_time);
    let primes: HashSet<uint> = primes_vec.iter().map(|&x| x).collect();
    println!("{} sec.", precise_time_s() - start_time);
    let mut sum: uint = 7;
    for x in primes.iter() {
        let _x = *x - 1u;
        if check(_x, &primes) == true {
            sum += _x;
        }
    }

    println!("{} sec.", precise_time_s() - start_time);
    println!("answer = {}", sum);
}

#[bench]
fn main_test(b: &mut Bencher) {
    b.iter(main);
}