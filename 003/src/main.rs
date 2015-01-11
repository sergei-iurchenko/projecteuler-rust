use std::iter;

fn is_prime(n: uint) -> bool {
    if n == 2 {return true};
    if n == 3 {return true};
    if n % 2 == 0 {return false};
    if n % 3 == 0 {return false};
    for i in iter::range_step(4, n, 6) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let limit = 600851475143u;
    //let limit = 13195u;
    let mut factors: Vec<uint> = Vec::new();
    for i in range(2, limit) {
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
