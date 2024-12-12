use ::std::fs;

fn read_input() -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let file = fs::read_to_string("src/input.txt").unwrap();
    for (i, s) in file.lines().enumerate() {
        res.push(Vec::new());
        for c in s.chars() {
            res[i].push(c as i32);
        }
    }
    res
}

fn search_right(trees: &Vec<Vec<i32>>, x: i32, y: i32) -> bool {
    let mut j = y + 1;
    while false {}
    true
}

fn main() {
    let _trees = read_input();
}
