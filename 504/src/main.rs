use std::num::{SignedInt, Float};

fn is_square_number(number: isize) -> bool {
    let number_precise = (number as f32).sqrt();
    let number_rounded = number_precise.trunc();
    if number_precise - number_rounded == Float::zero() {true} else {false}
}

fn get_y(y: f32) -> isize {
    if y == Float::zero() {return 0}
    let _y = y.abs();
    let y_floor = _y.floor();
    if _y - y_floor == Float::zero() {
        y_floor as isize - 1
    }
    else {
      _y as isize
    }
}

fn calc_sum(a: isize, b: isize, c: isize, d: isize) -> isize {
    let mut sum = a + b + c + d - 3;
    for x in (-c + 1)..0 {
        let y = ((b as f32) / (c as f32) * (x as f32)) + (b as f32);
        sum += get_y(y);        
//        println!("a={}, b={}, c={}, d={}", a, b, c, d);
//        println!("x={}, y={}, y_pres={}", x, y, get_y(y));
    }
    for x in 1..a {
        let y = (-(b as f32) / (a as f32) * (x as f32)) + (b as f32);
        sum += get_y(y);        
    }
    for x in (-c + 1)..0 {
        let y = (-(d as f32) / (c as f32) * (x as f32)) - (d as f32);
        sum += get_y(y);        
    }
    for x in 1..a {
        let y = ((d as f32) / (a as f32) * (x as f32)) - (d as f32);
        sum += get_y(y);        
    }
    sum
}

fn cycle_simple(n: isize) -> isize {
    let m = n + 1;
    let mut quantity = 0isize;
    for a in 1..m {
        for b in 1..m {
            for c in 1..m {
                for d in 1..m {
                    let result = calc_sum(a, b, c, d);
                    if is_square_number(result) {
//                        println!("{}", result);
                        quantity += 1
                    };
                }
            }
        }
    }
    quantity
}

fn main() {
    let m = 100;
    let result = cycle_simple(m);
    println!("Итого при m={}: {}", m, result);
}

#[test]
fn test_get_y() {
    assert!(get_y(5f32) == 4isize);
    assert!(get_y(5.1f32) == 5isize);
    assert!(get_y(-5.1f32) == 5isize);
    assert!(get_y(-1.1f32) == 1isize);
    assert!(get_y(-0.9f32) == 0isize);
    assert!(get_y(0.9f32) == 0isize);
    assert!(get_y(0.0f32) == 0isize);
    assert!(get_y(1.0f32) == 0isize);
}
