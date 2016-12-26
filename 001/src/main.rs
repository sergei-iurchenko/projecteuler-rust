fn find()-> usize {
    let below = 1000usize;
    let mut all_multiple: Vec<usize> = Vec::new();
    for i in 2..below {
        if i % 3 == 0{
            all_multiple.push(i);
            continue;
        }
        if i % 5 == 0 {
            all_multiple.push(i);
            continue;
        }
    }
    let sum = all_multiple.iter().map(|&x| x).sum();
    sum
}

fn main() {
    let sum = find();
    println!("{}", sum);
}

#[test]
fn it_works() {
    assert!(find() == 233168usize)
}
