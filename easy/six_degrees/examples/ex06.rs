// cargo run --example ex06
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

// Actor collaboration graph with BFS traversal
// Actors who appeared in the same movie are connected by an edge.
// We then use BFS to find the shortest path between two actors.

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

use std::collections::{HashMap, HashSet, VecDeque};

/// Builds an adjacency list from a list of movies.
/// Each movie is a tuple (title, vec_of_actors).
/// Two actors sharing a movie get an edge labeled with that movie title.
fn build_graph(movies: &Vec<(String, Vec<String>)>) -> HashMap<&str, HashSet<&str>> {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (_title, cast) in movies {
        for i in 0..cast.len() {
            for j in (i + 1)..cast.len() {
                graph.entry(&cast[i]).or_default().insert(&cast[j]);
                graph.entry(&cast[j]).or_default().insert(&cast[i]);
            }
        }
    }
    graph
}

/// BFS: returns the number of hops from `start` to `goal`, or None if unreachable.
fn bfs_len_shortest_path(
    graph: &HashMap<&str, HashSet<&str>>,
    start: &str,
    goal: &str,
) -> Option<usize> {
    if start == goal {
        return Some(0);
    }

    let mut visited: HashSet<&str> = HashSet::new();
    // Each queue entry: (current node, depth)
    let mut queue: VecDeque<(&str, usize)> = VecDeque::new();

    visited.insert(start);
    queue.push_back((start, 0));

    while let Some((current, depth)) = queue.pop_front() {
        if let Some(neighbors) = graph.get(current) {
            for &neighbor in neighbors {
                if neighbor == goal {
                    return Some(depth + 1);
                }
                if visited.insert(neighbor) {
                    queue.push_back((neighbor, depth + 1));
                }
            }
        }
    }
    None
}

fn main() {
    type Actor = String;
    type Film = String;
    const ACTOR_TARGET: &str = "Kevin Bacon";

    let mut reader = get_input_reader();
    let mut line = String::new();

    // Read actor
    reader.read_line(&mut line).unwrap();
    let actor_name: Actor = line.trim().parse().unwrap();
    // dbg!(&actor_name);

    // Read # of films
    line.clear();
    reader.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();
    // dbg!(n);

    // Read each film
    let mut credits: Vec<(Film, Vec<Actor>)> = Vec::new();
    for _ in 0..n {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let mut parts = line.splitn(2, ':');
        let film = parts.next().unwrap();
        // dbg!(film);

        let actors: Vec<String> = parts
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|name| name.trim().to_string())
            .collect();
        // dbg!(&actors);
        credits.push((film.to_string(), actors));
    }
    // dbg!(&credits);

    let graph = build_graph(&credits);

    println!(
        "{}",
        bfs_len_shortest_path(&graph, &actor_name, ACTOR_TARGET).unwrap_or(0)
    );
}
