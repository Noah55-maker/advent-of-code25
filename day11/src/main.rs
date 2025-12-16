use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    soln2();
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
        adj.insert(
            from.to_string(),
            to.split(' ').map(|s| s.to_string()).collect(),
        );
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

fn soln2() {
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
        adj.insert(
            from.to_string(),
            to.split(' ').map(|s| s.to_string()).collect(),
        );
    }

    // <curr, has_dac, has_fft, parent>
    let mut q: VecDeque<(String, bool, bool, String)> = VecDeque::new();
    q.push_back((String::from("svr"), false, false, String::from("")));

    // <curr, has_dac, has_fft> -> <parents>
    let mut visited: HashMap<(String, bool, bool), HashSet<String>> = HashMap::new();

    // bfs path search
    while !q.is_empty() {
        let curr = q.pop_front().unwrap();

        let look = (curr.0.clone(), curr.1, curr.2);
        if visited.contains_key(&look) {
            visited.entry(look).or_default().insert(curr.3);
            continue;
        }
        visited.insert(look, HashSet::from([curr.3]));

        if !adj.contains_key(&curr.0) {
            continue;
        }
        for s in &adj[&curr.0] {
            q.push_back((
                s.clone(),
                curr.1 || s == "dac",
                curr.2 || s == "fft",
                curr.0.clone(),
            ));
        }
    }

    // dfs reverse search
    fn count_paths(
        parents: &HashMap<(String, bool, bool), HashSet<String>>,
        cache: &mut HashMap<(String, bool, bool), i64>,
        s: &String,
        dac: bool,
        fft: bool,
    ) -> i64 {
        let key = (s.clone(), dac, fft);
        if cache.contains_key(&key) {
            return *cache.get(&key).unwrap();
        }

        if s == "svr" {
            if !dac && !fft {
                return 1;
            }
            return 0;
        }

        let mut sum = 0i64;
        for p in parents.get(&key).unwrap() {
            sum += count_paths(parents, cache, p, dac && s != "dac", fft && s != "fft");
        }

        cache.insert(key, sum);
        sum
    }

    println!(
        "{}",
        count_paths(&visited, &mut HashMap::new(), &String::from("out"), true, true)
    );
}
