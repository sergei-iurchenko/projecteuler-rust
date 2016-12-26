fn find() -> usize {
    let limit = 4_000_000usize;
    let mut fibonachi: Vec<usize> = Vec::new();
    fibonachi.push(1);
    fibonachi.push(2);
    let mut i = 2usize;
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
    let sum = fibonachi.iter().map(|&x| {if x % 2 == 0 {x} else {0} }).sum();
    sum
}

fn main() {
    let sum = find();
    println!("sum of even = {}", sum);
}

#[test]
fn it_works() {
    assert!(find()==4613732usize);

}
