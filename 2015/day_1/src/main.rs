use std::fs;

fn main() {
    let (exit_floor, basement_index) = solution("./input/data.txt");
    println!(
        "Exit floor: {}, Basement index: {}",
        exit_floor,
        basement_index.unwrap()
    );
}

fn solution(file_path: &str) -> (i64, Option<usize>) {
    let file = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut exit_floor = 0;
    let mut basement_index: Option<usize> = None;

    for (i, char) in file.chars().enumerate() {
        match char {
            '(' => exit_floor += 1,
            ')' => exit_floor -= 1,
            _ => (),
        }
        if exit_floor == -1 && basement_index.is_none() {
            basement_index = Some(i + 1);
        }
    }

    (exit_floor, basement_index)
}
