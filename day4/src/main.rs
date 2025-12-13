use std::io;

fn main() {
    soln2();
}

fn adj(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut count = 0;
    if x > 0 {
        if grid[y][x-1] == '@' {
            count += 1;
        }
        if y > 0 && grid[y-1][x-1] == '@' {
            count += 1;
        }
        if y+1 < grid.len() && grid[y+1][x-1] == '@' {
            count += 1;
        }
    }
    if x+1 < grid[0].len() {
        if grid[y][x+1] == '@' {
            count += 1;
        }
        if y > 0 && grid[y-1][x+1] == '@' {
            count += 1;
        }
        if y+1 < grid.len() && grid[y+1][x+1] == '@' {
            count += 1;
        }
    }

    if y > 0 && grid[y-1][x] == '@' {
        count += 1;
    }
    if y+1 < grid.len() && grid[y+1][x] == '@' {
        count += 1;
    }

    count
}

fn soln1() {
    let stdin = io::stdin();
    let mut grid: Vec<Vec<char>> = Vec::new();

    loop {
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer);

        if buffer.len() < 5 {
            break;
        }

        let mut line: Vec<char> = Vec::new();

        for c in buffer.chars() {
            line.push(c);
        }

        grid.push(line);
    }

    let mut count: i32 = 0;
    for i in 0..grid.len() {
        let k = &grid[i].len();
        for j in 0..(*k) {
            if grid[i][j] == '@' && adj(&grid, j, i) < 4 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn soln2() {
    let stdin = io::stdin();

    let mut grid: Vec<Vec<char>> = Vec::new();

    loop {
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer);

        if buffer.len() < 5 {
            break;
        }

        let mut line: Vec<char> = Vec::new();

        for c in buffer.chars() {
            line.push(c);
        }

        grid.push(line);
    }

    let mut total: i32 = 0;
    loop {
        let mut count: i32 = 0;
        for i in 0..grid.len() {
            let k = &grid[i].len();
            for j in 0..(*k) {
                if grid[i][j] == '@' && adj(&grid, j, i) < 4 {
                    count += 1;
                    grid[i][j] = '.';
                }
            }
        }

        if count == 0 {
            break;
        }
        total += count;
    }

    println!("{}", total);
}
