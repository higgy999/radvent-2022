use std::fs;

fn main() {
    part_one();
}

fn part_one() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
}
