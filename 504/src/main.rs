extern crate time;

use std::collections::BitSet;

use time::precise_time_s;

fn gcd(a: usize, b: usize) -> usize {
        if b == 0usize {
            a
        } else {
            gcd(b, a % b)
        }
}

fn calc_sum(a: usize, b: usize, c: usize, d: usize) -> usize {
    let CB = gcd(c,b);
    let AB = gcd(a,b);
    let DA = gcd(d,a);
    let CD = gcd(c,d);
    let G = CB + AB + DA + CD;
    let square = a*b + b*c + c*d + d*a;
    let B2 = square - G + 2;
//    if B2 < 0 {
//        println!("square-{}", square);
//        println!("G-{}", G);
//        println!("{}-{}", B2, B2/2);
//        println!("{} {} {} {}", a, b, c, d);
//        println!("{} {} {} {}", CB, AB, DA, CD);
//        panic!("B2<0");
//    }
//    if B2 % 2 != 0 {
//        println!("square-{}", square);
//        println!("G-{}", G);
//        println!("{}-{}", B2, B2/2);
//        println!("{} {} {} {}", a, b, c, d);
//        println!("{} {} {} {}", CB, AB, DA, CD);
//        panic!("B2 % 2 != 0");
//    }
    
    B2 / 2
}

fn cycle_simple(n: usize) -> usize {
    let mut powers = BitSet::new();
    for i in 1..2*n+1 {
        powers.insert((i*i) as usize);
    }

    let m = n + 1;
    let mut quantity = 0usize;
    for a in 1..m {
        for b in 1..m {
            for c in 1..m {
                for d in 1..m {
                    let result = calc_sum(a, b, c, d);
                    if powers.contains(&result) {
                        quantity += 1;
                    };
                }
            }
        }
    }
    quantity
}

fn main() {
    let start_time = precise_time_s();
    for i in 100..101 {
        let result = cycle_simple(i);
        println!("Итого при m={}: {}", i, result);
    }
    println!("{} sec.", precise_time_s() - start_time);
}

