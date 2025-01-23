use std::{fs, str::Split};

struct Line {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

fn process_part(part: &Split<&str>, a: usize, b: usize) -> i32 {
    //what a beautiful language
    part.clone()
        .nth(a)
        .expect("there is no a-th part")
        .split(',')
        .nth(b)
        .expect("there is no b-th part")
        .trim()
        .parse()
        .unwrap()
}

//fn filter_input()

fn read_input() -> Vec<Line> {
    let file = fs::read_to_string("src/input").unwrap();
    let mut res: Vec<Line> = Vec::new();
    for l in file.lines() {
        let parts = l.split(" <- ");
        let l = Line {
            x1: process_part(&parts, 0, 0),
            x2: process_part(&parts, 0, 1),
            y1: process_part(&parts, 1, 0),
            y2: process_part(&parts, 1, 1),
        };
        res.push(l);
    }
    res
}

fn main() {
    let input = read_input();
    for part in input {
        println!("{}; {} -> {};{}", part.x1, part.x2, part.y1, part.y2);
    }
}
