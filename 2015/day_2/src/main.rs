use std::fs;

fn main() {
    let (wrapping, ribbon) = solution("./input/data.txt");
    println!("Wrapping: {}, Ribbon: {}", wrapping, ribbon);
}

fn solution(file_path: &str) -> (i64, i64) {
    let file = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let mut total_wrapping: i64 = 0;
    let mut total_ribbon: i64 = 0;

    for line in file.lines() {
        let present = get_present(line);

        let surface_area = present.area_of_smallest_side()
            + 2 * present.length * present.width
            + 2 * present.width * present.height
            + 2 * present.height * present.length;

        total_wrapping += surface_area;

        let ribbon_length = present.perimeter_of_smallest_size() + present.volume();

        total_ribbon += ribbon_length;
    }

    (total_wrapping, total_ribbon)
}

fn get_present(line: &str) -> Present {
    const DELIMITER: &str = "x";
    let sizes = line
        .split(DELIMITER)
        .map(|number| number.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    Present::new(sizes)
}

struct Present {
    length: i64,
    width: i64,
    height: i64,
}

impl Present {
    pub fn new(sizes: Vec<i64>) -> Self {
        Self {
            length: sizes[0],
            width: sizes[1],
            height: sizes[2],
        }
    }

    pub fn area_of_smallest_side(&self) -> i64 {
        let (width, height) = self.smallest_sides();
        width * height
    }

    pub fn perimeter_of_smallest_size(&self) -> i64 {
        let (width, height) = self.smallest_sides();
        width * 2 + height * 2
    }

    pub fn volume(&self) -> i64 {
        self.length * self.width * self.height
    }

    fn smallest_sides(&self) -> (i64, i64) {
        let mut sizes = vec![self.length, self.width, self.height];
        sizes.sort();

        (sizes[0], sizes[1])
    }
}
