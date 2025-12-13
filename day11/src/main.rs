use std::collections::{HashMap, VecDeque};

fn main() {
    soln1();
}

fn soln1() {
    use std::io;
    let stdin = io::stdin();

    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer);

        if buffer.len() < 3 {
            break;
        }

        let c = buffer.clone();
        let (from, to) = c.trim().split_once(": ").unwrap();
        adj.insert(from.to_string(), to.split(' ').map(|s| s.to_string()).collect());
    }

    let mut q: VecDeque<String> = VecDeque::new();
    q.push_back(String::from("you"));

    let mut paths = 0;

    while !q.is_empty() {
        let curr = q.pop_front().unwrap();
        if curr == "out" {
            paths += 1;
            continue;
        }

        if !adj.contains_key(&curr) {
            continue;
        }
        for s in &adj[&curr] {
            q.push_back(s.clone());
        }
    }

    println!("{}", paths);
}
