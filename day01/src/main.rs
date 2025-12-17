use std::io;

fn main() {
    // soln1();
    soln2();
}

fn soln1() {
    let stdin = io::stdin();

    let mut lock: i32 = 50;
    let mut count0: i32 = 0;

    loop {
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer);
        let chars = buffer.chars();

        if buffer.starts_with('R') {
            let substr: String = chars.skip(1).take(buffer.len()-2).collect();
            let my_int: i32 = substr.parse().unwrap();

            lock += my_int;
        }
        else if buffer.starts_with('L') {
            let substr: String = chars.skip(1).take(buffer.len()-2).collect();
            let my_int: i32 = substr.parse().unwrap();

            lock -= my_int;
        } else {
            break;
        }

        lock = (lock % 100 + 100) % 100;
        if lock == 0 {
            count0 += 1;
        }
    }

    println!("{}", count0);
}

fn soln2() {
    let stdin = io::stdin();

    let mut lock: i32 = 50;
    let mut count0: i32 = 0;

    loop {
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer);
        let chars = buffer.chars();

        if buffer.starts_with('R') {
            let substr: String = chars.skip(1).take(buffer.len()-2).collect();
            let clicks: i32 = substr.parse().unwrap();

            count0 += clicks / 100;
            lock += clicks % 100;

            if lock >= 100 {
                lock -= 100;
                count0 += 1;
            }
        }
        else if buffer.starts_with('L') {
            let substr: String = chars.skip(1).take(buffer.len()-2).collect();
            let clicks: i32 = substr.parse().unwrap();

            count0 += clicks / 100;
            let was0 = lock == 0;
            lock -= clicks % 100;

            if lock == 0 || (lock < 0 && !was0) {
                count0 += 1;
            }
            if lock < 0 {
                lock += 100;
            }
        } else {
            break;
        }
    }

    println!("{}", count0);
}
