use std::iter::AdditiveIterator;

fn main() {
    let limit = 4_000_000u;
    let mut fibonachi: Vec<uint> = Vec::new();
    fibonachi.push(1);
    fibonachi.push(2);
    let mut i = 2u;
    loop {
        let k = fibonachi[i - 2] + fibonachi[i - 1];
        println!("{} - {}", i, k);
        if k < limit {
            fibonachi.push(k);
            i += 1;
        } else {
            break;
        }
    }
    println!("{}", fibonachi);
    let sum = fibonachi.iter().map(|&x| {if x % 2 == 0 {x} else {0} }).sum();
    println!("sum of even = {}", sum);
}
