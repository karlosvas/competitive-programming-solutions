use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().unwrap();

    // Creamos un set para guardar las letras diferentes
    let mut set = std::collections::HashSet::new();
    for c in input.chars() {
        set.insert(c);
    }

    // Si el numero de letras diferente es par es mujer si no es hombre
    if set.len() % 2 == 0 {
        println!("CHAT WITH HER!");
    } else {
        println!("IGNORE HIM!");
    }
}
