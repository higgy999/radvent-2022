use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    
    let mut adder:i64 = 0;
    let mut highest:i64 = 0;
    let mut elves_calories:Vec<i64> = Vec::new();

    for line in contents.lines() {
        if line != "" {
            adder += line.parse::<i64>().unwrap();
        } else {
            if adder > highest {
                highest = adder;
            }
            elves_calories.push(adder);
            adder = 0;
        }
    }
    elves_calories.sort_by(|a, b| b.cmp(a));

    println!("The highest is {}", highest);

    println!("1st is {}. 2nd is {}. 3rd is {}.",
             elves_calories.get(0).unwrap(),
             elves_calories.get(1).unwrap(),
             elves_calories.get(2).unwrap());

    println!("Together they are {}", elves_calories.get(0).unwrap()+elves_calories.get(1).unwrap()+elves_calories.get(2).unwrap());
}
