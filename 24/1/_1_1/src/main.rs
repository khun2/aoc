use std::fs::read_to_string;

read_to_string(filename: &str) -> Vec<(i32, i32)> {
    let mut res: Vec<(i32, i32)>;
    let lines = read_to_string(filename)
        .unwrap()
        .lines()
        .map(Sring::from)
        .collect();
    for mut s in lines {
        let parts = s.split(' ');
        res.push((parts[0], parts[1]));
        
    }
    res
}

fn main() {
}
