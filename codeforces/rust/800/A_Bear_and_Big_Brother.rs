use std::io::{self, stdin, BufRead};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut nums = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let (mut a, mut b) = (nums.next().unwrap(), nums.next().unwrap());

    let mut years: i32 = 0;
    while a <= b {
        a *= 3;
        b *= 2;
        years += 1;
    }

    println!("{}", years);
}
