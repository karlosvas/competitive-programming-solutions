use std::io::{self, BufRead};
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums = input.split_whitespace().map(|x| x.parse::<u64>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();
    let a = nums.next().unwrap();
    let tiles = ((n + a - 1) / a) * ((m + a - 1) / a);
    println!("{}", tiles);
}
