fn is_prime(n: usize) -> bool {
    if n == 2 {return true};
    if n == 3 {return true};
    if n % 2 == 0 {return false};
    if n % 3 == 0 {return false};
    for (_, i) in (4..6).enumerate().filter(|&(index, _)| index % 6 == 0) {
    //for i in iter::range_step(4, n, 6) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    //let limit = 600851475143usize;
    let limit = 13195usize;
    let mut factors: Vec<usize> = Vec::new();
    for i in 2..limit {
        if limit % i == 0 {
            factors.push(i);
        }
    }
    println!("{}", factors.len());
    for i in factors.iter().rev() {
        if is_prime(*i) {
            println!("{}", *i);
            break;
        }
    }
}
