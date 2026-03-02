// https://www.codingame.com/ide/puzzle/island-escape

// You're dropped onto an island, starting at the very middle of an N by N
// terrain. Luckily you have a raft and a map showing the elevation of each
// square plot of land. The ocean at elevation 0 surrounds the island and lies
// all along the borders of the map. You can move directly north, south, east,
// or west to an adjacent plot, provided the difference in elevation is at most
// one. Larger differences indicate steep terrain which cannot be traversed
// carrying your raft. Determine yes or no if it's possible to reach the ocean
// and get away.

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// Custom function to get the input reader (from file or stdin)
fn get_input_reader() -> Box<dyn BufRead> {
    let path = Path::new("input.txt");
    if path.exists() {
        let file = File::open(path).expect("Failed to open input.txt");
        Box::new(BufReader::new(file))
    } else {
        Box::new(BufReader::new(io::stdin()))
    }
}

fn main() {
    let mut reader = get_input_reader();
    let mut line = String::new();

    // Read matrix size
    reader.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        matrix.push(row);
    }
    // matrix is now a 2D array of i32
    // println!("{:?}", matrix);
    // dbg!(&matrix);

    // n is odd, 2<n<20
    let mid = (n - 1) / 2;

    // On se place dans la cellule au centre de la matrice (mid, mid) et si alt>0 on pousse (alt, mid, mid)
    // Tant que pop (alt, x, y)
    //      Si alt==0 break
    //      Pour cell in N, S, E, W
    //          dh = alt - cell.alt
    //          si abs(dh)<= 1 push (cell.alt, cell.x, cell.y) ET faut vérifier si elle est pas déjà dans la pile
    // Si alt==0 => yes sinon => no

    let mut my_stack: Vec<(i32, usize, usize)> = Vec::new();
    my_stack.push((matrix[mid][mid], mid, mid));

    let mut visited = HashSet::new();
    let mut escape = false;

    while let Some((alt, y, x)) = my_stack.pop() {
        if alt == 0 {
            escape = true;
            break;
        }
        // N, S, E, W
        // let cardinal_directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

        // for diff in cardinal_directions {
        for diff in [(-1isize, 0isize), (1, 0), (0, 1), (0, -1)] {
            if let Some(cell_y) = y.checked_add_signed(diff.0).filter(|&v| v < n)
                && let Some(cell_x) = x.checked_add_signed(diff.1).filter(|&v| v < n)
            {
                let cell_alt = matrix[cell_y][cell_x];
                // dbg!(cell_alt, cell_y, cell_x);
                if (alt - cell_alt).abs() <= 1 && !visited.contains(&(cell_y, cell_x)) {
                    my_stack.push((cell_alt, cell_y, cell_x));
                    visited.insert((cell_y, cell_x));
                }
            }
        }
    }
    // match escape {
    //     true => println!("yes"),
    //     false => println!("no"),
    // }
    println!("{}", if escape { "yes" } else { "no" });
}
