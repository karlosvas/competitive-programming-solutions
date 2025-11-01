use std::io::{self, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        // Lee n y k
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let nk: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = nk[0];
        let k = nk[1];

        // Lee los días
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let dias: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut sol = 0;
        let mut i = 0;
        while i + k <= n {
            if dias[i..i + k].iter().all(|&x| x == 0) {
                sol += 1;
                i += k + 1;
            } else {
                i += 1;
            }
        }
        println!("{}", sol);
    }
}
