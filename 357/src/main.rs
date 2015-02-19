extern crate time;

use std::num::{Float};
use time::precise_time_s;
use std::collections::bitv;
use std::collections::HashSet;
use std::iter;

fn get_primes_below(n: usize) -> Vec<usize> {
    let mut sieve = bitv::Bitv::from_elem(n, true);
    let stop = sieve.len();
    for i in iter::range_step(3, (n as f64).sqrt() as usize + 1us, 2) {
        if sieve[i] == true {
            for j in iter::range_step(i * i, stop, 2 * i) {
                sieve.set(j, false);
            }
        }
    }

    let mut result:Vec<usize> = Vec::with_capacity(n);
    result.push(2);
    for i in iter::range_step(3, n, 2) {
        if sieve[i] == true {
            result.push(i)
        };
    }
    result
}

fn check(n: usize, primes: &HashSet<usize>) -> bool {
    if ![0us, 2, 8].contains(&(n % 10))
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


fn main() {
    let start_time = precise_time_s();
    let a = 100000000us;
    let primes_vec = get_primes_below(a);
    println!("{} sec.", precise_time_s() - start_time);
    let primes: HashSet<usize> = primes_vec.iter().map(|&x| x).collect();
    println!("{} sec.", precise_time_s() - start_time);
    let mut sum = 7us;
    for x in primes.iter() {
        let _x = *x - 1us;
        if check(_x, &primes) == true {
            sum += _x;
        }
    }

    println!("{} sec.", precise_time_s() - start_time);
    println!("answer = {}", sum);
}

