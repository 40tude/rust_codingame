// cargo run --example ex01

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

// use std::collections::HashMap;
// const ACTOR_TARGET: &str = "Kevin Bacon";

// type Actor = String;
// type Film = String;

use graph::Graph;

fn main() {
    let mut reader = get_input_reader();
    let mut line = String::new();

    // Read actor
    reader.read_line(&mut line).unwrap();
    let actor_name: String = line.trim().parse().unwrap();
    dbg!(&actor_name);

    let mut credits: HashMap<Film, Vec<Actor>> = HashMap::new();
    let mut filmographies: HashMap<Actor, Vec<Film>> = HashMap::new();
    let mut adjacency_list: Vec<String> = Vec::new();

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
        // credits.insert(film.to_string(), actors.clone());
        // for actor in actors {
        //     filmographies
        //         .entry(actor)
        //         .or_default() // Creates Vec if key doesn't exist
        //         .push(film.to_string());
        // }
        for actor in actors {
            filmographies
                .entry(actor)
                .or_default() // Creates Vec if key doesn't exist
                .push(film.to_string());
        }
    }
    // dbg!(&credits);
    // dbg!(&filmographies);

    // Keep only actors appearing in more than one film and the actor of interest
    // filmographies.retain(|actor, films| films.len() > 1 || *actor == actor_name);

    // A l'exception de celui pour qui on fait la recherche, supprimer les
    // acteurs dont le nom n’apparaît qu'une fois dans les génériques
    // dbg!(&filmographies);
}

mod graph {
    use std::collections::{HashSet, VecDeque};

    struct GraphNode {
        name: String,
        neighbors: Vec<String>,
    }

    impl GraphNode {
        fn new(val: String, neighbors: Vec<String>) -> Self {
            Self {
                name: val,
                neighbors,
            }
        }

        fn process(&self) {
            println!("Processing node {}", self.name);
        }
    }

    pub struct Graph {
        nodes: Vec<GraphNode>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self { nodes: Vec::new() }
        }

        pub fn from_adjacency_list(adj_list: &[Vec<String>]) -> Self {
            let mut graph = Graph::new();
            for (i, neighbors) in adj_list.iter() {
                graph.nodes.push(GraphNode::new(i, neighbors.clone()));
            }
            graph
        }

        pub fn create_edge(&self, actor1: &String, actor2: &String) {
            for (i, neighbors) in adj_list.iter() {
                graph.nodes.push(GraphNode::new(i, neighbors.clone()));
            }
            graph
        }

        pub fn print(&self, node: usize) {
            let mut visited = HashSet::new();
            self.print_recursive(node, &mut visited);
        }

        fn print_recursive(&self, node: usize, visited: &mut HashSet<usize>) {
            if visited.contains(&node) {
                return;
            }
            visited.insert(node);
            println!(
                "Node {} has neighbors {:?}",
                node, &self.nodes[node].neighbors
            );
            for &neighbor in &self.nodes[node].neighbors {
                self.print_recursive(neighbor, visited);
            }
        }

        pub fn dfs(&self, start: usize) {
            let mut visited = HashSet::new();
            self.dfs_recursive(start, &mut visited);
        }

        fn dfs_recursive(&self, start: usize, visited: &mut HashSet<usize>) {
            if visited.contains(&start) {
                return;
            }
            visited.insert(start);
            self.nodes[start].process();
            for &neighbor in &self.nodes[start].neighbors {
                self.dfs_recursive(neighbor, visited);
            }
        }

        pub fn bfs(&self, start: usize) {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(start);
            while let Some(current) = queue.pop_front() {
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current);
                self.nodes[current].process();
                for &neighbor in &self.nodes[current].neighbors {
                    if !visited.contains(&neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }
}

// fn main() {

//     //     0
//     //   / | \
//     //  3  |  1
//     //   \ |
//     //     2
//     //   /
//     //  4
//     let adjacency_list = [
//         vec![1, 2, 3],
//         vec![0],
//         vec![0, 3, 4],
//         vec![0, 2],
//         vec![2],
//     ];

//     let my_graph = Graph::from_adjacency_list(&adjacency_list);
//     let start_node = 0;

//     my_graph.print(start_node);         // Node 0 has neighbors [1, 2, 3]
//                                         // ...

//     println!("\nDFS from node 0:");     // DFS from node 0:
//     my_graph.dfs(start_node);           // Processing node 0
//                                         // ...

//     println!("\nBFS from node 0:");     // BFS from node 0:
//     my_graph.bfs(start_node);           // Processing node 0
//                                         // ...
// }
