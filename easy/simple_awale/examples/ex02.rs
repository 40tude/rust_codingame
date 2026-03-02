// cargo run --example ex02
// https://www.codingame.com/ide/puzzle/simple-awale

/// Read input either form input.txt of io::stdin
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
fn get_input_reader() -> Box<dyn BufRead> {
    let path = Path::new("input.txt");
    if path.exists() {
        let file = File::open(path).expect("Failed to open input.txt");
        Box::new(BufReader::new(file))
    } else {
        Box::new(BufReader::new(io::stdin()))
    }
}

use std::fmt;
use std::mem;
struct Awale {
    // me = [0..6] opponent = [7..13]
    bowls: [i32; 14],
}

impl Awale {
    fn format_row(&self, bowls_range: std::ops::Range<usize>, reserve: usize) -> String {
        let parts = self.bowls[bowls_range]
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        format!("{} [{}]", parts, self.bowls[reserve])
    }
}

impl fmt::Display for Awale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.format_row(7..13, 13))?;
        write!(f, "{}", self.format_row(0..6, 6))
    }
}

fn main() {
    // opponent
    let mut reader = get_input_reader();
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let temp: Vec<i32> = line.trim().split(' ').map(|i| i.parse().unwrap()).collect();
    let mut game: Awale = Awale { bowls: [-1; 14] };
    game.bowls[7..=13].copy_from_slice(&temp);

    // me
    line.clear();
    reader.read_line(&mut line).unwrap();
    let temp: Vec<i32> = line.trim().split(' ').map(|i| i.parse().unwrap()).collect();
    game.bowls[0..=6].copy_from_slice(&temp[0..=6]);

    line.clear();
    reader.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    let mut seeds = mem::take(&mut game.bowls[n]);
    let mut index = n + 1;
    while seeds > 0 {
        game.bowls[index % 13] += 1;
        index += 1;
        seeds -= 1;
    }

    println!("{}", game);
    if index == 7 {
        println!("REPLAY");
    }
}
