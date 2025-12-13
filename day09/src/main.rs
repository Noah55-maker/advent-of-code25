fn main() {
    soln1();
}

fn soln1() {
    use std::io;
    let stdin = io::stdin();

    let mut pts: Vec<(i64, i64)> = Vec::new();
    loop {
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer);

        if buffer.len() < 3 {
            break;
        }

        let c = buffer.clone();
        let p = c.trim().split_once(',').unwrap();
        pts.push((p.0.parse().unwrap(), p.1.parse().unwrap()));
    }

    let mut area = 0;
    for i in 0..pts.len() {
        for j in 0..i {
            let x = pts[i].0.max(pts[j].0) - pts[i].0.min(pts[j].0) + 1;
            let y = pts[i].1.max(pts[j].1) - pts[i].1.min(pts[j].1) + 1;
            area = area.max(x*y);
        }
    }

    println!("{}", area);
}
