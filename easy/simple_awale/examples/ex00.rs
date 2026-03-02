// cargo run --example ex00
// https://www.codingame.com/ide/puzzle/simple-awale

// Awalé is an African 2 players game consisting of moving grains in some bowls.
// Each player has 7 bowls indexed from 0 to 6. The last bowl is the reserve.

// At each turn a player chooses one of his own bowls except the reserve, picks
// up all grains there and redistributes them one by one to bowls beginning just
// after the chosen one. If the number of grains in hand is sufficient, after
// adding one to his reserve the player continues the distribution in the
// opponent's bowls excluding his reserve and then back in his own bowls, until
// his hand is empty.

// If the final grain is distributed to the player's reserve, he is allowed to
// play again.

// Examples

// bowls num : 0 1 2 3 4 5  6
// --------------------------
// opp bowls : 5 1 0 6 2 2 [3]
//  my bowls : 3 4 0 3 3 2 [2]

// I play bowl 0: distribute 3 grains in bowl 1, 2 and 3
// bowls num : 0 1 2 3 4 5  6
// --------------------------
// opp bowls : 5 1 0 6 2 2 [3]
//  my bowls : 0 5 1 4 3 2 [2]

// I play bowl 5: distribute 2 grains (1 in my reserve and 1 in the first
// opponent bowl)

// bowls num : 0 1 2 3 4 5  6
// --------------------------
// opp bowls : 6 1 0 6 2 2 [3]
//  my bowls : 3 4 0 3 3 0 [3]

// If I end in my reserve I can replay:
// I play bowl 3:
// bowls num : 0 1 2 3 4 5  6
// --------------------------
// opp bowls : 5 1 0 6 2 2 [3]
//  my bowls : 3 4 0 0 4 3 [3]
// REPLAY

// Your goal is to simulate your turn of game. Given the numbers of grains in
// each bowl and the num of the chosen bowl your program has to display the new
// situation and the string REPLAY if the player has a chance to play again.
// Print the numbers of grains of opponent bowls separated by space, then yours.
// Put reserve counts between brackets. Remember that the player always skips
// the opponent's reserve when distributing!

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
// #[derive(Debug)] // trop moche à l'affichage
struct Bowls {
    bowls: [i32; 7],
}

impl FromIterator<i32> for Bowls {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut bowls = [0i32; 7];
        for (i, v) in iter.into_iter().enumerate().take(7) {
            bowls[i] = v;
        }
        Bowls { bowls }
    }
}

impl fmt::Display for Bowls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

    reader.read_line(&mut line).unwrap();
    let mut op_bowls: Bowls = line.trim().split(' ').map(|i| i.parse().unwrap()).collect();
    // dbg!(&op_bowls);

    line.clear();
    reader.read_line(&mut line).unwrap();
    let mut my_bowls: Bowls = line.trim().split(' ').map(|i| i.parse().unwrap()).collect();
    // dbg!(&my_bowls);

    line.clear();
    reader.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();
    // dbg!(n);

    // On ajoute 1 à partir du bol n+1 modulo 13
    // Si on met le dernier dans son bol 6 => REPLAY
    // Exemple affichage
    // 4 4 4 4 4 4 [0]
    // 4 4 0 5 5 5 [1]
    // REPLAY

    // let parts = my_bowls[0..6]
    //     .iter()
    //     .map(|i| i.to_string())
    //     .collect::<Vec<_>>()
    //     .join(" ");
    // println!("{} [{}]", parts, my_bowls[6]);

    let mut g_bowls = [0; 13];
    g_bowls[0..=6].copy_from_slice(&my_bowls.bowls);
    g_bowls[7..=12].copy_from_slice(&op_bowls.bowls[0..=5]);
    // dbg!(g_bowls);

    let mut seeds = g_bowls[n];
    g_bowls[n] = 0;
    let mut index = n + 1;
    while seeds > 0 {
        g_bowls[index % 13] += 1;
        index += 1;
        seeds -= 1;
    }
    // dbg!(&g_bowls);

    my_bowls.bowls.copy_from_slice(&g_bowls[0..=6]);
    op_bowls.bowls[0..=5].copy_from_slice(&g_bowls[7..=12]);
    println!("{}", op_bowls);
    println!("{}", my_bowls);
    if index == 7 {
        println!("REPLAY");
    }
}
