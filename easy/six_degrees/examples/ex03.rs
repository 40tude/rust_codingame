// Actor collaboration graph with BFS traversal
// Actors who appeared in the same movie are connected by an edge.
// We then use BFS to find the shortest path between two actors.

use std::collections::{HashMap, HashSet, VecDeque};

/// Builds an adjacency list from a list of movies.
/// Each movie is a tuple (title, vec_of_actors).
/// Two actors sharing a movie get an edge labeled with that movie title.
fn build_graph<'a>(movies: &'a Vec<(String, Vec<String>)>) -> HashMap<&'a str, HashSet<&'a str>> {
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

/// Finds the movie that connects two actors (for display purposes).
fn find_shared_movie<'a>(
    movies: &'a Vec<(String, Vec<String>)>,
    actor_a: &str,
    actor_b: &str,
) -> Option<&'a str> {
    movies.iter().find_map(|(title, cast)| {
        if cast.iter().any(|s| s == actor_a) && cast.iter().any(|s| s == actor_b) {
            Some(title.as_str())
        } else {
            None
        }
    })
}

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

type Actor = String;
type Film = String;
const ACTOR_TARGET: &str = "Kevin Bacon";

fn main() {
    let mut reader = get_input_reader();
    let mut line = String::new();

    // Read actor
    reader.read_line(&mut line).unwrap();
    let actor_name: String = line.trim().parse().unwrap();
    // dbg!(&actor_name);

    // let mut credits: HashMap<Film, Vec<Actor>> = HashMap::new();
    let mut credits: Vec<(Film, Vec<Actor>)> = Vec::new();
    // let mut filmographies: HashMap<Actor, Vec<Film>> = HashMap::new();

    // Read # of films
    line.clear();
    reader.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();
    // dbg!(n);

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
        credits.push((film.to_string(), actors.clone()));
        // for actor in actors {
        //     filmographies
        //         .entry(actor)
        //         .or_default() // Creates Vec if key doesn't exist
        //         .push(film.to_string());
        // }
    }
    // dbg!(&credits);
    // dbg!(&filmographies);

    // --- Define some movies and their casts ---
    // let movies: Vec<(&str, Vec<&str>)> = vec![
    //     (
    //         "Inception",
    //         vec!["DiCaprio", "Hardy", "Gordon-Levitt", "Page"],
    //     ),
    //     (
    //         "The Dark Knight",
    //         vec!["Bale", "Ledger", "Hardy", "Oldman", "Freeman"],
    //     ),
    //     (
    //         "Interstellar",
    //         vec!["McConaughey", "Hathaway", "Caine", "Damon"],
    //     ),
    //     (
    //         "The Departed",
    //         vec!["DiCaprio", "Nicholson", "Damon", "Wahlberg"],
    //     ),
    //     ("Good Will Hunting", vec!["Damon", "Williams", "Affleck"]),
    //     (
    //         "Batman Begins",
    //         vec!["Bale", "Caine", "Oldman", "Freeman", "Neeson"],
    //     ),
    //     (
    //         "The Prestige",
    //         vec!["Bale", "Jackman", "Johansson", "Caine"],
    //     ),
    //     ("Titanic", vec!["DiCaprio", "Winslet", "Zane"]),
    //     ("Les Misérables", vec!["Jackman", "Hathaway", "Crowe"]),
    // ];

    // --- Build the graph ---
    let graph = build_graph(&credits);

    // --- Display the graph ---
    // println!("=== Actor Collaboration Graph ===\n");
    // let mut actors: Vec<&&str> = graph.keys().collect();
    // actors.sort();
    // for actor in &actors {
    //     let mut neighbors: Vec<&&str> = graph[**actor].iter().collect();
    //     neighbors.sort();
    //     let neighbor_list: Vec<String> = neighbors.iter().map(|n: &&&str| n.to_string()).collect();
    //     println!("  {:<15} -> [{}]", actor, neighbor_list.join(", "));
    // }

    // --- BFS shortest path queries ---
    // println!("\n=== BFS Shortest Path Queries ===\n");

    // let queries = vec![
    //     ("Kevin Bacon", "Shane Carruth"),
    //     // ("Williams", "Crowe"),
    //     // ("Nicholson", "Caine"),
    //     // ("Zane", "Neeson"),
    // ];

    // for (start, goal) in queries {
    // print!("Path from {} to {}: ", start, goal);
    match bfs_shortest_path(&graph, &actor_name, ACTOR_TARGET) {
        Some(path) => {
            // Print each hop with the connecting movie
            let _hops: Vec<String> = path
                .windows(2)
                .map(|pair| {
                    // let movie = find_shared_movie(&movies, pair[0], pair[1]).unwrap_or("?");
                    let movie = find_shared_movie(&credits, pair[0], pair[1]).unwrap_or("?");
                    format!("{} --[{}]--> {}", pair[0], movie, pair[1])
                })
                .collect();
            // println!("{}", hops.join("  ·  "));
            // println!("  (distance: {} hop(s))\n", path.len() - 1);
            println!("{}", path.len() - 1);
        }
        None => println!("0"),
    }
    // }
}
