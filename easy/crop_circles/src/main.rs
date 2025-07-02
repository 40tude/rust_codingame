// use std::io;

// macro_rules! parse_input {
//     ($x:expr, $t:ident) => {
//         $x.trim().parse::<$t>().unwrap()
//     };
// }

// fn main() {
//     let mut input_line = String::new();
//     io::stdin().read_line(&mut input_line).unwrap();
//     let instructions = input_line.trim_matches('\n').to_string();

//     // Write an answer using println!("message...");
//     // To debug: eprintln!("Debug message...");

//     println!("Farming-Field with Crop-Circles");
// }

// use std::fs::File;
// use std::io::{self, BufRead, BufReader};

// macro_rules! parse_input {
//     ($x:expr, $t:ident) => {
//         $x.trim().parse::<$t>().unwrap()
//     };
// }

// fn main() {
//     // Try to open input.txt, fallback to stdin if not found
//     let reader: Box<dyn BufRead> = match File::open("input.txt") {
//         Ok(file) => Box::new(BufReader::new(file)),
//         Err(_) => Box::new(BufReader::new(io::stdin())),
//     };

//     let mut lines = reader.lines();

//     // Read the first line
//     if let Some(Ok(input_line)) = lines.next() {
//         let instructions = input_line.trim().to_string();

//         // Write an answer using println!("message...");
//         // To debug: eprintln!("Debug message...");

//         println!("Farming-Field with Crop-Circles");
//         eprintln!("Instructions: {}", instructions); // Debug output
//     } else {
//         eprintln!("No input provided.");
//     }
// }

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

    // Read first line
    reader.read_line(&mut line).unwrap();
    let (w, h): (usize, usize) = {
        let mut parts = line.trim().split_whitespace();
        let w = parts.next().unwrap().parse().unwrap();
        let h = parts.next().unwrap().parse().unwrap();
        (w, h)
    };

    line.clear();
    reader.read_line(&mut line).unwrap();
    let count: usize = line.trim().parse().unwrap();

    // Read h lines into a 2D map
    let mut map = Vec::new();
    for _ in 0..h {
        line.clear();
        reader.read_line(&mut line).unwrap();
        map.push(line.trim().chars().collect::<Vec<char>>());
    }

    // Debug output
    eprintln!("w = {}, h = {}", w, h);
    eprintln!("count = {}", count);
    eprintln!("map = {:?}", map);
}
