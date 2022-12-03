use std::collections::HashMap;
use std::fs;

fn main() {
    let mut hashmap: HashMap<char, i32> = HashMap::new();
    let mut i = 1;
    for letter in 'a'..='z' {
        hashmap.insert(letter, i);
        i += 1;
    }
    let mut i = 27;
    for letter in 'A'..='Z' {
        hashmap.insert(letter, i);
        i += 1;
    }

    part_one(&hashmap);
    part_two(&hashmap);
}

fn part_one(priority_hashmap: &HashMap<char, i32>) {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut added_priorities: i32 = 0;
    for line in contents.lines() {
        if line != "" {
            let mut shared_letters: String = "".to_string();
            let (sack1, sack2) = line.split_at(line.len()/2);

            for char in sack1.chars() {
                if sack2.contains(char) && !shared_letters.contains(char) {
                    shared_letters.push(char);
                }
            }

            for char in shared_letters.chars() {
                match priority_hashmap.get(&char) {
                    Some(number) => added_priorities += number,
                    None => println!("Expected letter to be in the list but got {}", char)
                };
            }
        }
    }
    println!("Sum of priorities: {}", added_priorities)
}

fn part_two(priority_hashmap: &HashMap<char, i32>) {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut added_priorities: i32 = 0;
    let mut elf_counter:i8 = 1;
    let mut the_last_line: String = "".to_string();
    let mut common_characters_1: String = "".to_string();
    let mut common_characters_final: String = "".to_string();
    for line in contents.lines() {
        if line != "" {
            if elf_counter == 1 {
                line.clone_into(&mut the_last_line);
                elf_counter += 1;
            }
            else if elf_counter == 2 {
                for char in the_last_line.chars() {
                    if line.contains(char) && !common_characters_1.contains(char) {
                        common_characters_1.push(char);
                    }
                }
                elf_counter += 1;
            }
            else if elf_counter == 3 {
                for char in common_characters_1.chars() {
                    if line.contains(char) {
                        common_characters_final.push(char);
                    }
                }
                common_characters_1.clear();
                elf_counter = 1;
            }
        }
    }

    for char in common_characters_final.chars() {
        match priority_hashmap.get(&char) {
            Some(number) => added_priorities += number,
            None => println!("Expected letter to be in the list but got {}", char)
        };
    }

    println!("Added group badge priorities: {}", added_priorities);
}
