use std::io::{self, stdin, BufRead};

fn main() {
    // Leemos los casos de prueba
    let mut lines = stdin().lock().lines();
    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Casos de prueba (t)
    for _ in 0..t {
        // Longitud de la matriz (longitud)
        let longitud: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let mut is_desarreglo: bool = true;

        // Leer todos los elementos en una línea separados por espacios
        let elementos_linea = lines.next().unwrap().unwrap();
        let elementos: Vec<i32> = elementos_linea
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // Elementos ordenados ascendentemente
        let mut shorted_elements = elementos.clone();
        shorted_elements.sort();

        // Verificar si es un desarreglo
        for i in 0..elementos.len() {
            let right = elementos[i];
            let left = shorted_elements[i];

            if right == left {
                is_desarreglo = false;
                break;
            }
        }

        // Si es un desarreglo lo mostramos si nbo simplemente decimos que no
        if is_desarreglo {
            println!("YES");
            println!("{}", elementos.len());
            println!(
                "{}",
                elementos
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        } else {
            println!("NO");
        }
    }
}
