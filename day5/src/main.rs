use std::{collections::BTreeMap, io, ops::Bound};

fn main() {
    let _ = part2faster();
}

fn part1() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer;

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    loop {
        buffer = String::new();
        stdin.read_line(&mut buffer)?;
        if buffer.len() < 3 {
            break;
        }
        let both = buffer.trim().split_once('-').unwrap();
        ranges.push((both.0.parse::<i64>().unwrap(), both.1.parse::<i64>().unwrap()));
    }

    let mut fresh = 0;
    loop {
        buffer = String::new();
        stdin.read_line(&mut buffer)?;

        if buffer.trim().len() == 0 {
            break;
        }
        let ingredient = buffer.trim().parse::<i64>().unwrap();

        for r in &ranges {
            if ingredient >= r.0 && ingredient <= r.1 {
                fresh += 1;
                break;
            }
        }
    }

    println!("{}", fresh);
    Ok(())
}

// O(n^2)
fn part2() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer;

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    loop {
        buffer = String::new();
        stdin.read_line(&mut buffer)?;
        if buffer.len() < 3 {
            break;
        }
        let both = buffer.trim().split_once('-').unwrap();
        ranges.push((both.0.parse::<i64>().unwrap(), both.1.parse::<i64>().unwrap()));
    }

    let mut i = 0;
    while i+1 < ranges.len() {
        let mut j = i+1;
        let mut changed = false;
        while j < ranges.len() {
            if ranges[i].1 < ranges[j].0 || ranges[i].0 > ranges[j].1 {
                j += 1;
                continue;
            }
            ranges[i].0 = std::cmp::min(ranges[i].0, ranges[j].0);
            ranges[i].1 = std::cmp::max(ranges[i].1, ranges[j].1);

            let last = ranges.len()-1;
            ranges.swap(j, last);
            ranges.pop();
            changed = true;
        }

        if !changed {
            i += 1;
        }
    }

    let mut fresh = 0;
    for r in &ranges {
        println!("{},{}", r.0, r.1);
        fresh += r.1 - r.0 + 1;
    }

    println!();
    println!("{}", fresh);
    Ok(())
}

// O(n*log(n)) with ordered map
fn part2fast() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer;

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    loop {
        buffer = String::new();
        stdin.read_line(&mut buffer)?;
        if buffer.len() < 2 {
            break;
        }
        let both = buffer.trim().split_once('-').unwrap();
        ranges.push((both.0.parse::<i64>().unwrap(), both.1.parse::<i64>().unwrap()));
    }

    // store non-overlapping ranges in an ordered map (range_start -> range_end)
    let mut map: BTreeMap<i64, i64> = BTreeMap::new();
    for r in &ranges {
        let c = map.clone();
        let ran = c.range((Bound::Included(r.0), Bound::Included(r.1)));
        // v includes all ranges overlapping r that start >= r.0
        let mut v: Vec<(&i64, &i64)> = ran.collect();

        // check for a range that begins before r.0 and overlaps r
        let prev = c.range((Bound::Unbounded, Bound::Excluded(r.0))).next_back();
        if prev != None && *prev.unwrap().1 >= r.0 {
            v.push(prev.unwrap());
        }

        let mut min = r.0;
        let mut max = r.1;
        for p in &v {
            min = min.min(*p.0);
            max = max.max(*p.1);
            map.remove(p.0);
        }

        map.insert(min, max);
    }

    let mut fresh = 0;
    for r in map {
        fresh += r.1 - r.0 + 1;
    }

    println!("{}", fresh);
    Ok(())
}

// O(n*log(n)) with sorting
fn part2faster() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer;

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    loop {
        buffer = String::new();
        stdin.read_line(&mut buffer)?;
        if buffer.len() < 2 {
            break;
        }
        let both = buffer.trim().split_once('-').unwrap();
        ranges.push((both.0.parse::<i64>().unwrap(), both.1.parse::<i64>().unwrap()));
    }

    // sort by starting times
    ranges.sort_unstable();
    let mut non_overlapping = Vec::<(i64, i64)>::new();
    for r in &ranges {
        let mut low = r.0;
        let mut high = r.1;

        // if there's an overlapping interval, merge it with the current one
        while !non_overlapping.is_empty() && non_overlapping.last().unwrap().1 >= low {
            let back = non_overlapping.last().unwrap();
            low = back.0;
            high = high.max(back.1);
            non_overlapping.pop();
        }

        non_overlapping.push((low, high));
    }

    let mut fresh = 0;
    for r in non_overlapping {
        fresh += r.1 - r.0 + 1;
    }

    println!("{}", fresh);
    Ok(())
}
