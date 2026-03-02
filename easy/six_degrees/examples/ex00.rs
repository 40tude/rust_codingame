// cargo run --example ex00
// https://www.codingame.com/ide/puzzle/six-degrees-of-kevin-bacon

// Six Degrees of Kevin Bacon is a pop-culture game in which an arbitrarily
// chosen actor is repeatedly connected to another actor via a movie that both
// actors have appeared in together, repeating this process to try to find the
// shortest path that ultimately leads to the prolific American actor Kevin
// Bacon. Given an actor_name, an integer n and then that many movie_casts
// determine the Bacon number of actor_name, i.e. the minimum number of movies
// needed to link actor_name to Kevin Bacon.

// Example:
// Elvis Presley
// 3
// Change of Habit: Elvis Presley, Mary Tyler Moore, Barbara McNair, Jane Elliot, Ed Asner
// JFK: Kevin Costner, Kevin Bacon, Tommy Lee Jones, Laurie Metcalf, Gary Oldman, Ed Asner
// Sleepers: Kevin Bacon, Jason Patric, Brad Pitt, Robert De Niro, Dustin Hoffman

// The answer is 2 because Elvis Presley → Ed Asner → Kevin Bacon, using Change
// of Habit to connect Presley and Asner and then JFK to connect Asner to Bacon
// = 2 degrees of separation.
// Custom function to get the input reader (from file or stdin)

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

use std::collections::HashMap;
const ACTOR_TARGET: &str = "Kevin Bacon";

type Actor = String;
type Film = String;

fn main() {
    let mut reader = get_input_reader();
    let mut line = String::new();

    // Read actor
    reader.read_line(&mut line).unwrap();
    let actor_name: String = line.trim().parse().unwrap();
    dbg!(&actor_name);

    let mut credits: HashMap<Film, Vec<Actor>> = HashMap::new();
    let mut filmographies: HashMap<Actor, Vec<Film>> = HashMap::new();

    // Read # of films
    line.clear();
    reader.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();
    dbg!(n);

    // Read each film
    for _ in 0..n {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let parts = line.split(':');
        let collection: Vec<&str> = parts.collect();
        let film = collection[0].trim();
        // dbg!(film);
        let actors: Vec<String> = collection[1]
            .trim()
            .split(',')
            .map(|name| name.trim().to_string())
            .collect();
        // dbg!(&actors);
        credits.insert(film.to_string(), actors.clone());
        for actor in actors {
            filmographies
                .entry(actor)
                .or_default() // Creates Vec if key doesn't exist
                .push(film.to_string());
        }
    }
    dbg!(&credits);
    dbg!(&filmographies);

    // Keep only actors appearing in more than one film and the actor of interest
    filmographies.retain(|actor, films| films.len() > 1 || *actor == actor_name);

    // A l'exception de celui pour qui on fait la recherche, supprimer les
    // acteurs dont le nom n’apparaît qu'une fois dans les génériques
    dbg!(&filmographies);
}
