use std::iter::AdditiveIterator;

fn main() {
    let below = 1000u;
    let mut all_multiple: Vec<uint> = Vec::new();
    for i in range(2, below) {
        if i % 3 == 0{
            all_multiple.push(i);
            continue;
        }
        if i % 5 == 0 {
            all_multiple.push(i);
            continue;
        }
    }
    //println!("{}", all_multiple);
    let sum = all_multiple.iter().map(|&x| x).sum();
    println!("{}", sum);
}
