use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut increment: i32 = 0;
    for line in contents.lines() {
        if line != "" {
            let split = line.split(",");
            let vec: Vec<&str> = split.collect();

            let vec_of_first_range: Vec<&str> = vec.get(0).unwrap().split("-").collect();
            let vec_of_second_range: Vec<&str> = vec.get(1).unwrap().split("-").collect();

            let first_in_first = vec_of_first_range.get(0).unwrap().parse::<i32>().unwrap();
            let second_in_first = vec_of_first_range.get(1).unwrap().parse::<i32>().unwrap();
            let first_in_second = vec_of_second_range.get(0).unwrap().parse::<i32>().unwrap();
            let second_second = vec_of_second_range.get(1).unwrap().parse::<i32>().unwrap();

            if first_in_first >= first_in_second && first_in_first <= second_second {
                if second_in_first >= first_in_second && second_in_first <= second_second {
                    increment += 1;
                    println!("{} -> {}", &line, true);
                    continue;
                }
            }
            if first_in_second >= first_in_first && second_second <= second_in_first {
                if first_in_second >= first_in_first && second_second <= second_in_first {
                    increment += 1;
                    println!("{} -> {}", &line, true);
                    continue;
                }
            }
            println!("{} -> {}", &line, false);
        }
    }
    println!("In how many assignment pairs does one range fully contain the other? {}", increment);
}

fn part_two() {
    
}
