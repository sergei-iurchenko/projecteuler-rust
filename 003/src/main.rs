

fn main() {
    let limit = 13195u;
    let mut factors: Vec<uint> = Vec::new();
    for i in range(2, limit) {
        if limit % i == 0 {
            factors.push(i);
        }
    }
    println!("{}", factors);
}
