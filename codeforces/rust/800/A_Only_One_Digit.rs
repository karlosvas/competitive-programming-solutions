use std::io::{self, BufRead};

fn shares_digit(x: i32, y: i32) -> bool {
    let x_str = x.to_string();
    let y_str = y.to_string();

    for digit_x in x_str.chars() {
        if y_str.contains(digit_x) {
            return true;
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for index in 0..t {
        let x: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

        let mut y = 0;
        while !shares_digit(x, y) {
            y += 1;
        }

        println!("{}", y);
    }
}
