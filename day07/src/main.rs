fn main() {
    soln2();
}

fn soln1() {
    use std::io;
    let stdin = io::stdin();

    let mut grid: Vec<Vec<char>> = Vec::new();
    loop {
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer);

        if buffer.len() < 2 {
            break;
        }

        let cl = buffer.clone();
        grid.push(cl.trim().as_bytes().iter().map(|b| *b as char).collect());
    }

    for i in 0..grid[0].len() {
        if grid[0][i] == 'S' {
            grid[1][i] = '|';
            break;
        }
    }

    let mut splits = 0;
    for i in 1..grid.len()-1 {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' && grid[i-1][j] == '|' {
                splits += 1;
                if j != 0 {
                    grid[i+1][j-1] = '|';
                }
                if j+1 != grid[i].len() {
                    grid[i+1][j+1] = '|';
                }
            }
            else if grid[i][j] == '|' && grid[i+1][j] == '.' {
                grid[i+1][j] = '|';
            }
        }
    }

    println!("{}", splits);
}

fn soln2() {
    use std::io;
    let stdin = io::stdin();

    let mut grid: Vec<Vec<(char, i64)>> = Vec::new();
    loop {
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer);

        if buffer.len() < 2 {
            break;
        }

        let cl = buffer.clone();
        grid.push(cl.trim().as_bytes().iter().map(|b| (*b as char, 0)).collect());
    }

    for i in 0..grid[0].len() {
        if grid[0][i].0 == 'S' {
            grid[1][i] = ('|', 1);
            break;
        }
    }

    for i in 1..grid.len()-1 {
        for j in 0..grid[i].len() {
            if grid[i][j].0 == '^' && grid[i-1][j].0 == '|' {
                if j != 0 {
                    grid[i+1][j-1].0 = '|';
                    grid[i+1][j-1].1 += grid[i-1][j].1;
                }
                if j+1 != grid[i].len() {
                    grid[i+1][j+1].0 = '|';
                    grid[i+1][j+1].1 += grid[i-1][j].1;
                }
            }
            else if grid[i][j].0 == '|' && grid[i+1][j].0 != '^' {
                grid[i+1][j].0 = '|';
                grid[i+1][j].1 += grid[i][j].1;
            }
        }
    }

    let mut timelines = 0;
    let last = grid.len()-1;
    for i in 0..grid[0].len() {
        if grid[last][i].0 == '|' {
            timelines += grid[last][i].1;
        }
    }

    println!("{}", timelines);
}
