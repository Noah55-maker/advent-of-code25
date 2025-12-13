use std::io;
use std::collections::HashSet;

fn main() {
    // soln1();
    soln2();
}

fn soln1() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let _ = stdin.read_line(&mut buffer);

    let v: Vec<&str> = buffer.trim().split(',').collect();
    let mut ids: Vec<(i64, i64)> = [].to_vec();
    let mut longest_length = 0;

    for i in 0..v.len() {
        let p = v[i].split_once('-').unwrap();
        if p.0.len() > longest_length {
            longest_length = p.0.len();
        }

        let low = p.0.parse::<i64>().unwrap();
        let high = p.1.parse::<i64>().unwrap();
        ids.push((low, high));
    }

    let mut sum: i64 = 0;
    let mut set: HashSet<i64> = HashSet::new();

    for i in 1..1000000 {
        let id = i.to_string() + &i.to_string();
        if id.len() > longest_length {
            continue;
        }

        let val = id.parse::<i64>().unwrap();

        for p in &ids {
            if val >= p.0 && val <= p.1 {
                set.insert(val);
                break;
            }
        }
    }

    for i in &set {
        sum += i;
    }

    println!("{}", sum);
}

fn soln2() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let _ = stdin.read_line(&mut buffer);

    let v: Vec<&str> = buffer.trim().split(',').collect();
    let mut ids: Vec<(i64, i64)> = [].to_vec();
    let mut longest_length = 0;

    for i in 0..v.len() {
        let p = v[i].split_once('-').unwrap();
        if p.0.len() > longest_length {
            longest_length = p.0.len();
        }

        let low = p.0.parse::<i64>().unwrap();
        let high = p.1.parse::<i64>().unwrap();
        ids.push((low, high));
    }

    let mut sum: i64 = 0;
    let mut set: HashSet<i64> = HashSet::new();

    for i in 1..1000000 {
        let mut id = i.to_string();

        loop {
            id += &i.to_string();
            if id.len() > longest_length {
                break;
            }

            let val = id.parse::<i64>().unwrap();

            for p in &ids {
                if val >= p.0 && val <= p.1 {
                    set.insert(val);
                    break;
                }
            }
        }
    }

    for i in &set {
        sum += i;
    }

    println!("{}", sum);
}
