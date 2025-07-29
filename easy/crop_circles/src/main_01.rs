// https://www.codingame.com/ide/puzzle/crop-circles

// The Farming-Field is 25 high and 19 wide.
// Planted : {}
// rows    : [a,y]
// columns : [a,s]
// xyd or xydd the centers
// PLANT    = add back the {} crop to the entire circle.
// PLANTMOW = within a circle if a spot is planted, then mow it; if a spot is mowed, then plant it.
// Example :
//      ft17 PLANTft9 nf17 PLANTnf9 PLANTjm5
//      PLANTgg7 or PLANTMOWjm13

// Error management :
// * ? instead of unwrap(): error propagation with Result type.
// * ok_or(...) to convert Option to Result with custom error message.
// * map_err(...) to transform parsing errors (parse()).
// * Display an error message for any instruction that does not match the pattern (Regex::captures).

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const HEIGHT: usize = 25;
const WIDTH: usize = 19;

// Custom function to get the input reader (from file or stdin)
fn get_input_reader() -> io::Result<Box<dyn BufRead>> {
    let path = Path::new("input.txt");
    if path.exists() {
        let file = File::open(path)?;
        Ok(Box::new(BufReader::new(file)))
    } else {
        Ok(Box::new(BufReader::new(io::stdin())))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut field = [[b' '; WIDTH * 2]; HEIGHT];

    for row in field.iter_mut() {
        for col in 0..WIDTH {
            row[2 * col] = b'{';
            row[2 * col + 1] = b'}';
        }
    }

    let pattern = r"^(PLANTMOW|PLANT)?([a-s])([a-y])(\d{1,2})$";
    let re = Regex::new(pattern)?; // Return error if regex is invalid

    let mut reader = get_input_reader()?;
    let mut line = String::new();
    reader.read_line(&mut line)?; // Return error if reading fails
    let instructions: Vec<&str> = line.split_whitespace().collect();

    for instruction in instructions.iter() {
        if let Some(caps) = re.captures(instruction) {
            let prefix = caps.get(1).map_or("", |m| m.as_str());

            let c_char = caps.get(2).and_then(|m| m.as_str().chars().next()).ok_or("Invalid column character")?;
            let center_col = (c_char as u8 - b'a') as isize;

            let r_char = caps.get(3).and_then(|m| m.as_str().chars().next()).ok_or("Invalid row character")?;
            let center_row = (r_char as u8 - b'a') as isize;

            let diam: isize = caps.get(4).ok_or("Missing diameter")?.as_str().parse().map_err(|_| "Diameter is not a valid number")?;

            let radius = diam as f64 / 2.0;
            let radius_sq = radius * radius;

            // Traverse all pixels in the square around the circle
            for dy in -radius as isize..=radius as isize {
                for dx in -radius as isize..=radius as isize {
                    // Is the pixel in the circle?
                    if (dx * dx + dy * dy) as f64 <= radius_sq {
                        let curr_row = center_row + dy;
                        let curr_col = center_col + dx;

                        // Bounds checking
                        if curr_row >= 0 && curr_row < HEIGHT as isize && curr_col >= 0 && curr_col < WIDTH as isize {
                            let (curr_row, curr_col) = (curr_row as usize, curr_col as usize);

                            match prefix {
                                "PLANTMOW" => {
                                    if field[curr_row][2 * curr_col] == b' ' {
                                        field[curr_row][2 * curr_col] = b'{';
                                        field[curr_row][2 * curr_col + 1] = b'}';
                                    } else {
                                        field[curr_row][2 * curr_col] = b' ';
                                        field[curr_row][2 * curr_col + 1] = b' ';
                                    }
                                }
                                "PLANT" => {
                                    field[curr_row][2 * curr_col] = b'{';
                                    field[curr_row][2 * curr_col + 1] = b'}';
                                }
                                _ => {
                                    // MOW (default behavior)
                                    field[curr_row][2 * curr_col] = b' ';
                                    field[curr_row][2 * curr_col + 1] = b' ';
                                }
                            }
                        }
                    }
                }
            }
        } else {
            eprintln!("Warning: Ignored invalid instruction '{}'", instruction);
        }
    }

    for row in field.iter() {
        for col in 0..WIDTH {
            print!("{}{}", row[2 * col] as char, row[2 * col + 1] as char);
        }
        println!();
    }

    Ok(())
}
