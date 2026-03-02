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

/// BFS: returns the shortest path (list of actors) from `start` to `goal`.
/// Returns None if no path exists.
fn bfs_shortest_path<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    start: &'a str,
    goal: &'a str,
) -> Option<Vec<&'a str>> {
    if start == goal {
        return Some(vec![start]);
    }

    let mut visited: HashSet<&str> = HashSet::new();
    // Each queue entry stores the current actor and the path taken to reach them
    let mut queue: VecDeque<Vec<&str>> = VecDeque::new();

    visited.insert(start);
    queue.push_back(vec![start]);

    while let Some(path) = queue.pop_front() {
        let current = *path.last().unwrap();

        if let Some(neighbors) = graph.get(current) {
            for &neighbor in neighbors {
                if neighbor == goal {
                    let mut full_path = path.clone();
                    full_path.push(neighbor);
                    return Some(full_path);
                }
                if visited.insert(neighbor) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    queue.push_back(new_path);
                }
            }
        }
    }
    None
}

type Actor = String;
type Film = String;
const ACTOR_TARGET: &str = "Kevin Bacon";

fn main() {
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
        credits.push((film.to_string(), actors.clone()));
    }
    // dbg!(&credits);

    let graph = build_graph(&credits);

    if let Some(path) = bfs_shortest_path(&graph, &actor_name, ACTOR_TARGET) {
        println!("{}", path.len() - 1);
    } else {
        println!("No path?")
    }
}
