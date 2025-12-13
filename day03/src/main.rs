use std::io;

fn main() {
    soln2();
}

fn soln1() {
    let stdin = io::stdin();

    let mut sum: u32 = 0;
    loop {
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer).unwrap();
        buffer = (&buffer.trim()).to_string();
        if buffer.len() == 0 {
            break;
        }
        let chars = buffer.chars();

        let mut max_digit: u32 = 0;
        let mut max_joltage: u32 = 0;
        for c in chars {
            let d = c.to_digit(10).unwrap();
            let joltage: u32 = max_digit * 10 + d;
            if joltage > max_joltage {
                max_joltage = joltage;
            }
            if d > max_digit {
                max_digit = d;
            }
        }

        sum += max_joltage;
    }

    println!("{}", sum);
}

fn soln2() {
    let stdin = io::stdin();
    let mut sum: u64 = 0;

    loop {
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer).unwrap();
        buffer = (&buffer.trim()).to_string();
        if buffer.len() == 0 {
            break;
        }

        let mut stk: Vec<u32> = Vec::new();
        let slen = buffer.chars().count();

        for (i, c) in buffer.chars().enumerate() {
            let d = c.to_digit(10).unwrap();

            // pop digits from stk if we can increase the joltage
            while !stk.is_empty() && &d > stk.last().unwrap() && (i+12-stk.len() < slen) {
                stk.pop();
            }

            if stk.len() < 12 {
                stk.push(d);
            }
        }

        let mut joltage: u64 = 0;
        for i in 0..12 {
            joltage *= 10;
            joltage += u64::from(stk[i]);
        }
        sum += joltage;
    }

    println!("{}", sum);
}
