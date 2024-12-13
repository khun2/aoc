use ::std::fs;

fn read_input() -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let file = fs::read_to_string("src/input").unwrap();
    for (i, s) in file.lines().enumerate() {
        res.push(Vec::new());
        for c in s.chars() {
            res[i].push(c as i32 - 48);
        }
    }
    res
}

fn search_right(trees: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    let mut j = trees[row].len() - 1;
    while j > col && trees[row][j] < trees[row][col] {
        j -= 1;
    }
    j == col
}

fn search_left(trees: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    let mut j = 0;
    while j < col && trees[row][j] < trees[row][col] {
        j += 1;
    }
    j == col
}

fn search_down(trees: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    let mut i = trees.len() - 1;
    while i > row && trees[i][col] < trees[row][col] {
        i -= 1;
    }
    i == row
}

fn search_up(trees: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    let mut i = 0;
    while i < row && trees[i][col] < trees[row][col] {
        i += 1;
    }
    i == row
}

fn main() {
    let trees = read_input();
    let mut res = (trees.len() + trees[0].len()) * 2 - 4;
    println!("{}, {}", trees.len(), trees[0].len());
    for v in &trees {
        for n in v {
            print!("{} ", n);
        }
        print!("\n");
    }
    for row in 1..trees.len() - 1 {
        for col in 1..trees[row].len() - 1 {
            if search_up(&trees, row, col)
                || search_down(&trees, row, col)
                || search_left(&trees, row, col)
                || search_right(&trees, row, col)
            {
                println!("{} visible at: {}, {}", trees[row][col], row, col);
                res += 1;
            }
        }
    }
    println!("{}", res);
}
