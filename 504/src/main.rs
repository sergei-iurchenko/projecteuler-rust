use std::num::{SignedInt, Float};

fn is_square_number(number: isize) -> bool {
    let number_precise = (number as f32).sqrt();
    let number_rounded = number_precise.round();
    if number_precise - number_rounded == Float::zero() {true} else {false}
}

fn get_y(y: f32) -> isize {
    let y_floor = y.floor();
    if y - y_floor == Float::zero() {
        (y_floor as isize).abs() - 1
    }
    else {
      (y as isize).abs()
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
    let m = 4;
    //for i in 1..200 {
    //    println!("{} - {}", i, is_square_number(i));    
    //}
    let result = cycle_simple(m);
    println!("Итого при m={}: {}", m, result);
}

#[test]
fn test_get_y() {
    println!("{}", get_y(5f32));
    assert!(get_y(5f32) == 4isize);
    assert!(get_y(5.1f32) == 5isize);
}
