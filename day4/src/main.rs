use std::fs;

fn main() {
    part_one();
}

fn part_one() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        if line != "" {
            let split = line.split(",");
            let vec: Vec<&str> = split.collect();
            for string in vec {
                println!("{}", string);
            }
        }
    }
}
