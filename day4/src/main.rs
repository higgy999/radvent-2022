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
            let vec: Vec<&str> = line.split(",").collect();

            let vec_of_first_range: Vec<&str> = vec.get(0).unwrap().split("-").collect();
            let vec_of_second_range: Vec<&str> = vec.get(1).unwrap().split("-").collect();

            let first_in_first = vec_of_first_range.get(0).unwrap().parse::<i32>().unwrap();
            let second_in_first = vec_of_first_range.get(1).unwrap().parse::<i32>().unwrap();
            let first_in_second = vec_of_second_range.get(0).unwrap().parse::<i32>().unwrap();
            let second_second = vec_of_second_range.get(1).unwrap().parse::<i32>().unwrap();
            let mut is_contained: bool = true;

            for number in first_in_first..=second_in_first {
                if !(first_in_second..=second_second).contains(&number) {
                    is_contained = false;
                    break;
                } else {
                    continue;
                }
            }
            if is_contained {
                increment += 1;
                //println!("{} -> {}", &line, true);
                continue;
            } else {
                is_contained = true;
                for number in first_in_second..=second_second {
                    if !(first_in_first..=second_in_first).contains(&number) {
                        is_contained = false;
                        break;
                    } else {
                        continue;
                    }
                }
            }
            if is_contained {
                increment += 1;
                //println!("{} -> {}", &line, true);
                continue;
            } else {
                //println!("{} -> {}", &line, false);
            }
        }
    }
    println!("In how many assignment pairs does one range fully contain the other? {}", increment);
}

fn part_two() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut increment: i32 = 0;
    for line in contents.lines() {
        if line != "" {
            let vec: Vec<&str> = line.split(",").collect();

            let vec_of_first_range: Vec<&str> = vec.get(0).unwrap().split("-").collect();
            let vec_of_second_range: Vec<&str> = vec.get(1).unwrap().split("-").collect();

            let first_in_first = vec_of_first_range.get(0).unwrap().parse::<i32>().unwrap();
            let second_in_first = vec_of_first_range.get(1).unwrap().parse::<i32>().unwrap();
            let first_in_second = vec_of_second_range.get(0).unwrap().parse::<i32>().unwrap();
            let second_second = vec_of_second_range.get(1).unwrap().parse::<i32>().unwrap();

            let mut found_overlap: bool = false;
            for number in first_in_first..=second_in_first {
                if (first_in_second..=second_second).contains(&number) {
                    increment += 1;
                    found_overlap = true;
                    //println!("{} -> {}", &line, true);
                    break;
                }
            }
            if !found_overlap {
                for number in first_in_second..=second_second {
                    if (first_in_first..=second_in_first).contains(&number) {
                        increment += 1;
                        //found_overlap = true;
                        //println!("{} -> {}", &line, true);
                        break;
                    }
                }
                //println!("{} -> {}", &line, false);
            }
        }
    }
    println!("In how many assignment pairs do the ranges overlap, at all? {}", increment);
}
