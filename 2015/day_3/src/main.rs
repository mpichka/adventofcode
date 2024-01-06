use std::{collections::HashMap, fs};

fn main() {
    let first_solution = first_solution("./input/data.txt");
    println!("First solution: {}", first_solution);

    let second_solution = second_solution("./input/data.txt");
    println!("Second solution: {}", second_solution);
}

fn first_solution(file_path: &str) -> usize {
    let file = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut position = Position::new();

    let mut map = Map::new();

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

fn second_solution(file_path: &str) -> usize {
    let file = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut santa_position = Position::new();
    let mut robo_santa_position = Position::new();

    let mut map = Map::new();

    let mut delivery_turn = DeliveryGuy::Santa;

    for char in file.chars() {
        let position = match delivery_turn {
            DeliveryGuy::Santa => &mut santa_position,
            DeliveryGuy::RoboSanta => &mut robo_santa_position,
        };

        match char {
            'v' => position.move_down(),
            '>' => position.move_right(),
            '<' => position.move_left(),
            '^' => position.move_up(),
            _ => (),
        };
        map.deliver_present(&position);

        delivery_turn = match delivery_turn {
            DeliveryGuy::Santa => DeliveryGuy::RoboSanta,
            DeliveryGuy::RoboSanta => DeliveryGuy::Santa,
        };
    }

    map.grid.len()
}

enum DeliveryGuy {
    Santa,
    RoboSanta,
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
    pub fn new() -> Self {
        let mut grid = HashMap::new();

        grid.insert((0, 0), 1);

        Self { grid }
    }

    pub fn deliver_present(&mut self, position: &Position) {
        let presents = self.grid.get(&position.get()).unwrap_or(&0) + 1;

        self.grid.insert(position.get(), presents);
    }
}
