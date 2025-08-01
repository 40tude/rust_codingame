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

// Enum for PLANT, MOW, PLANTMOW

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const HEIGHT: usize = 25;
const WIDTH: usize = 19;

// Enum to represent the action to perform
enum Action {
    Plant,
    Mow,
    PlantMow,
}

impl Action {
    // Parse the prefix into an Action
    fn from_str(prefix: &str) -> Self {
        match prefix {
            "PLANT" => Action::Plant,
            "PLANTMOW" => Action::PlantMow,
            _ => Action::Mow, // Default to MOW if no prefix or unknown
        }
    }
}

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

// Apply one instruction to the field
fn apply_instruction(field: &mut [[u8; WIDTH * 2]; HEIGHT], action: Action, center_row: isize, center_col: isize, diam: isize) {
    let radius = diam as f64 / 2.0;
    let radius_sq = radius * radius;

    // Traverse all pixels in the square around the circle
    for dy in -radius as isize..=radius as isize {
        for dx in -radius as isize..=radius as isize {
            // Is the pixel in the circle?
            if (dx * dx + dy * dy) as f64 <= radius_sq {
                let row = center_row + dy;
                let col = center_col + dx;

                // Bounds checking
                if row >= 0 && row < HEIGHT as isize && col >= 0 && col < WIDTH as isize {
                    let (r, c) = (row as usize, col as usize);

                    match action {
                        Action::PlantMow => {
                            if field[r][2 * c] == b' ' {
                                field[r][2 * c] = b'{';
                                field[r][2 * c + 1] = b'}';
                            } else {
                                field[r][2 * c] = b' ';
                                field[r][2 * c + 1] = b' ';
                            }
                        }
                        Action::Plant => {
                            field[r][2 * c] = b'{';
                            field[r][2 * c + 1] = b'}';
                        }
                        Action::Mow => {
                            field[r][2 * c] = b' ';
                            field[r][2 * c + 1] = b' ';
                        }
                    }
                }
            }
        }
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

    let mut reader = get_input_reader()?; // Return error if reading fails
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let instructions: Vec<&str> = line.split_whitespace().collect();

    for instruction in instructions.iter() {
        if let Some(caps) = re.captures(instruction) {
            let action = Action::from_str(caps.get(1).map_or("", |m| m.as_str()));

            let c_char = caps.get(2).and_then(|m| m.as_str().chars().next()).ok_or("Invalid column character")?;
            let center_col = (c_char as u8 - b'a') as isize;

            let r_char = caps.get(3).and_then(|m| m.as_str().chars().next()).ok_or("Invalid row character")?;
            let center_row = (r_char as u8 - b'a') as isize;

            let diam: isize = caps.get(4).ok_or("Missing diameter")?.as_str().parse().map_err(|_| "Diameter is not a valid number")?;

            // Apply the parsed instruction to the field
            apply_instruction(&mut field, action, center_row, center_col, diam);
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
