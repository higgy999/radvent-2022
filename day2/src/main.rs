use std::fs;

enum RoundOutcome {
    Win,
    Stalemate,
    Loss,
}

#[derive(PartialEq)]
enum Opponent {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum You {
    Rock,
    Paper,
    Scissors,
}

impl RoundOutcome {
    fn as_int(&self) -> i32 {
        match self {
            RoundOutcome::Win => 6,
            RoundOutcome::Stalemate => 3,
            RoundOutcome::Loss => 0,
        }
    }
    fn parse(string: &str) -> Option<RoundOutcome> {
        match string {
            "X" => Some(RoundOutcome::Loss),
            "Y" => Some(RoundOutcome::Stalemate),
            "Z" => Some(RoundOutcome::Win),
            _ => None,
        }
    }
}

impl Opponent {
    fn parse(string: &str) -> Option<Opponent> {
        match string {
            "A" => Some(Opponent::Rock),
            "B" => Some(Opponent::Paper),
            "C" => Some(Opponent::Scissors),
            _ => None,
        }
    }
}

impl You {
    fn parse(string: &str) -> Option<You> {
        match string {
            "X" => Some(You::Rock),
            "Y" => Some(You::Paper),
            "Z" => Some(You::Scissors),
            _ => None,
        }
    }
    fn as_int(&self) -> i32 {
        match self {
            You::Rock => 1,
            You::Paper => 2,
            You::Scissors => 3,
        }
    }
}

fn main() {
    let p1: i32 = part_one();
    println!("Score after believed outcome: {}", p1);

    let p2: i32 = part_two();
    println!("Score after real outcome: {}", p2);
}

fn play_rps(opponent: &Opponent, you: &You) -> Result<RoundOutcome, i32> {
    if *opponent == Opponent::Rock {
        return match you {
            You::Rock => Ok(RoundOutcome::Stalemate),
            You::Paper => Ok(RoundOutcome::Win),
            You::Scissors => Ok(RoundOutcome::Loss),
        }
    }
    if *opponent == Opponent::Paper {
        return match you {
            You::Rock => Ok(RoundOutcome::Loss),
            You::Paper => Ok(RoundOutcome::Stalemate),
            You::Scissors => Ok(RoundOutcome::Win),
        }
    }
    if *opponent == Opponent::Scissors {
        return match you {
            You::Rock => Ok(RoundOutcome::Win),
            You::Paper => Ok(RoundOutcome::Loss),
            You::Scissors => Ok(RoundOutcome::Stalemate),
        }
    }
    return Err(0);
}

fn part_one() -> i32 {
    let mut score: i32 = 0;
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        if line != "" {
            let opponent: Opponent = Opponent::parse(&line[0..1]).unwrap();
            let you: You = You::parse(&line[2..3]).unwrap();

            let outcome: RoundOutcome = play_rps(&opponent, &you).unwrap();

            score = score + outcome.as_int() + you.as_int();
        }
    }
    return score;
}

fn what_should_you_play(outcome_needed: &RoundOutcome, opponent: &Opponent) -> Result<You, i32> {
    if *opponent == Opponent::Rock {
        return match outcome_needed {
            RoundOutcome::Win => Ok(You::Paper),
            RoundOutcome::Stalemate => Ok(You::Rock),
            RoundOutcome::Loss => Ok(You::Scissors),
        };
    }
    if *opponent == Opponent::Paper {
        return match outcome_needed {
            RoundOutcome::Win => Ok(You::Scissors),
            RoundOutcome::Stalemate => Ok(You::Paper),
            RoundOutcome::Loss => Ok(You::Rock),
        };
    }
    if *opponent == Opponent::Scissors {
        return match outcome_needed {
            RoundOutcome::Win => Ok(You::Rock),
            RoundOutcome::Stalemate => Ok(You::Scissors),
            RoundOutcome::Loss => Ok(You::Paper),
        };
    }
    return Err(0);
}

fn part_two() -> i32 {
    let mut score: i32 = 0;
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        if line != "" {
            let opponent: Opponent = Opponent::parse(&line[0..1]).unwrap();
            let outcome: RoundOutcome = RoundOutcome::parse(&line[2..3]).unwrap();
            let you: You = what_should_you_play(&outcome, &opponent).unwrap();

            score = score + outcome.as_int() + you.as_int();
        }
    }
    return score;
}
