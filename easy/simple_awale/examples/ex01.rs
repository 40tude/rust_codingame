// cargo run --example ex01
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
struct Bowls {
    // me = [0..6] opponent = [7..13]
    bowls: [i32; 14],
}

impl FromIterator<i32> for Bowls {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut bowls = [0i32; 14];
        for (i, v) in iter.into_iter().enumerate().take(7) {
            bowls[i] = v;
        }
        Bowls { bowls }
    }
}

impl fmt::Display for Bowls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts = self.bowls[7..13]
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let _ = writeln!(f, "{} [{}]", parts, self.bowls[13]);

        let parts = self.bowls[0..6]
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "{} [{}]", parts, self.bowls[6])
    }
}

impl fmt::Debug for Bowls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

fn main() {
    let mut reader = get_input_reader();
    let mut line = String::new();

    let mut g_bowls: Bowls = Bowls { bowls: [-1; 14] };

    // opponent
    reader.read_line(&mut line).unwrap();
    let temp: Vec<i32> = line.trim().split(' ').map(|i| i.parse().unwrap()).collect();
    g_bowls.bowls[7..=13].copy_from_slice(&temp);
    // dbg!(&g_bowls);

    // me
    line.clear();
    reader.read_line(&mut line).unwrap();
    let temp: Vec<i32> = line.trim().split(' ').map(|i| i.parse().unwrap()).collect();
    g_bowls.bowls[0..=6].copy_from_slice(&temp[0..=6]);
    // dbg!(&g_bowls);

    line.clear();
    reader.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();
    // dbg!(n);

    let mut seeds = g_bowls.bowls[n];
    g_bowls.bowls[n] = 0;
    let mut index = n + 1;
    while seeds > 0 {
        g_bowls.bowls[index % 13] += 1;
        index += 1;
        seeds -= 1;
    }
    // dbg!(&g_bowls);

    println!("{}", g_bowls);
    if index == 7 {
        println!("REPLAY");
    }
}
