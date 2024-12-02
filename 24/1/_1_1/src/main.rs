use std::fs::read_to_string;

fn read_input(filename: &str) -> Vec<(i32, i32)> {
    let mut res: Vec<(i32, i32)>;
    let lines : Vec<String> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    for s in lines {
        let parts = s.split(" ");
        res.push((parts[0], parts[1]));
    }
    res
}

fn main() {
    let v = read_input("input.txt");
    for p in v {
        println!("{}, {}\n", p.0, p.1);
    }
    
}
