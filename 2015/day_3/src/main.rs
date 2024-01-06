use std::{collections::HashMap, fs};

fn main() {
    let houses = solution("./input/data.txt");
    println!("Houses: {}", houses);
}

fn solution(file_path: &str) -> usize {
    let file = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut position = Position::new();

    let mut map = Map::new(&position);

    for char in file.chars() {
        match char {
            'v' => position.move_down(),
            '>' => position.move_right(),
            '<' => position.move_left(),
            '^' => position.move_up(),
            _ => (),
        };
        map.deliver_present(&position);
    }

    map.grid.len()
}

struct Position {
    x: i64,
    y: i64,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn move_up(&mut self) {
        self.y += 1;
    }

    pub fn move_down(&mut self) {
        self.y -= 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }
    pub fn get(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

struct Map {
    grid: HashMap<(i64, i64), i64>,
}

impl Map {
    pub fn new(initial_position: &Position) -> Self {
        let mut grid = HashMap::new();

        grid.insert(initial_position.get(), 1);

        Self { grid }
    }

    pub fn deliver_present(&mut self, position: &Position) {
        let presents = self.grid.get(&position.get()).unwrap_or(&0) + 1;

        self.grid.insert(position.get(), presents);
    }
}
