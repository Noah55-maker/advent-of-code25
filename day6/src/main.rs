use std::io;

fn main() -> io::Result<()> {
    part2()?;
    Ok(())
}

fn part1() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer;

    let mut ranges: Vec<Vec<_>> = Vec::new();
    let ops: Vec<char>;
    loop {
        buffer = String::new();
        println!("read stuff");
        stdin.read_line(&mut buffer)?;
        let trimmed = buffer.clone();
        let first = buffer.chars().nth(0).unwrap();
        if first == '+' || first == '*' {
            ops = trimmed.trim().split_ascii_whitespace().map(|s| s.chars().nth(0).unwrap()).collect();
            break;
        }
        let line: Vec<_> = trimmed.trim().split_ascii_whitespace().map(|s| s.parse::<i64>()).collect();
        ranges.push(line);
    }

    let mut total = 0i64;
    for eq in 0..ranges[0].len() {
        if ops[eq] == '+' {
            let mut sum: i64 = 0;
            for o in 0..ranges.len() {
                sum += ranges[o][eq].as_ref().unwrap();
            }
            total += sum;
            println!("+ {}", sum);
        }

        if ops[eq] == '*' {
            let mut prod: i64 = 1;
            for o in 0..ranges.len() {
                prod *= ranges[o][eq].as_ref().unwrap();
            }
            total += prod;
            println!("* {}", prod);
        }
    }

    println!("{}", total);
    Ok(())
}

fn part2() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer;

    let mut lines: Vec<_> = Vec::new();
    let mut ops: Vec<(i64, char)> = Vec::new();
    loop {
        buffer = String::new();
        stdin.read_line(&mut buffer)?;

        if buffer.len() < 4 {
            break;
        }
        lines.push(buffer.as_bytes().iter().map(|b| *b as char).collect());
    }

    let last: Vec<char> = lines.pop().unwrap();
    let mut idx = 0i64;
    for c in last {
        if c == '+' || c == '*' {
            ops.push((idx, c));
        }
        idx += 1;
    }

    let mut total = 0i64;
    for i in 0..ops.len() {
        let curr = ops[i].0;
        let mut next = idx+1;
        if i+1 < ops.len() {
            next = ops[i+1].0;
        }

        let mut ans = 0i64;
        if ops[i].1 == '*' {
            ans = 1;
        }

        for col in curr..next-1 {
            let mut num = 0i64;
            for row in 0..lines.len() {
                if lines[row][col as usize] != ' ' {
                    num *= 10;
                    num += lines[row][col as usize].to_digit(10).unwrap() as i64;
                }
            }

            if ops[i].1 == '+' {
                ans += num;
            } else {
                ans *= num;
            }
        }
        total += ans;
    }

    println!("{}", total);
    Ok(())
}
