use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let day = 5;
    match day {
        1 => day_1(),
        2 => day_2::run(),
        3 => day_3::run(),
        4 => day_4::run(),
        5 => day_5::run(),
        _ => (),
    }
}

mod utils {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn read_all_file(filename: &str) -> Vec<String> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        reader.lines().filter_map(Result::ok).collect()
    }
}

mod day_5 {
    use crate::utils::read_all_file;
    use std::iter::FromIterator;

    //[M]                     [N] [Z]
    //[F]             [R] [Z] [C] [C]
    //[C]     [V]     [L] [N] [G] [V]
    //[W]     [L]     [T] [H] [V] [F] [H]
    //[T]     [T] [W] [F] [B] [P] [J] [L]
    //[D] [L] [H] [J] [C] [G] [S] [R] [M]
    //[L] [B] [C] [P] [S] [D] [M] [Q] [P]
    //[B] [N] [J] [S] [Z] [W] [F] [W] [R]
    // 1   2   3   4   5   6   7   8   9
    pub fn run() {
        let mut stacks: Vec<Vec<char>> = [
            "BLDTWCFM", "NBL", "JCHTLV", "SPJW", "ZSCFTLR", "WDGBHNZ", "FMSPVGCN", "WQRJFVCZ",
            "RPMLH",
        ]
        .into_iter()
        .map(|string| Vec::from_iter(string.chars()))
        .collect();

        let stacks2 = stacks.clone();

        let inputs = read_all_file("inputs/input5.txt");

        // Part 1, Single Moves:
        for input in inputs.iter() {
            let elems: Vec<&str> = input.split(' ').collect();
            assert_eq!(elems.len(), 6);
            let num_moves = elems[1].parse::<usize>().unwrap();
            let from_pos = elems[3].parse::<usize>().unwrap();
            let to_pos = elems[5].parse::<usize>().unwrap();

            for _i in 0..num_moves {
                let crate_ = stacks[from_pos - 1].pop().unwrap();
                stacks[to_pos - 1].push(crate_);
            }
        }

        for (i, stack) in stacks.iter().enumerate() {
            println!("{}: {:?}", i + 1, stack);
        }

        let mut tops = Vec::new();
        for stack in stacks.iter() {
            if stack.len() > 0 {
                tops.push(stack[stack.len() - 1]);
            }
        }
        let s: String = tops.iter().collect();
        println!("Tops: {}", s);

        let mut stacks = stacks2;
        // Part 2, Combo Moves:
        for input in inputs {
            let elems: Vec<&str> = input.split(' ').collect();
            assert_eq!(elems.len(), 6);
            let num_moves = elems[1].parse::<usize>().unwrap();
            let from_pos = elems[3].parse::<usize>().unwrap();
            let to_pos = elems[5].parse::<usize>().unwrap();

            let mut picked_up_crates = Vec::new();
            for _i in 0..num_moves {
                picked_up_crates.push(stacks[from_pos - 1].pop().unwrap());
            }
            picked_up_crates.reverse();
            for crate_ in picked_up_crates {
                stacks[to_pos - 1].push(crate_);
            }
        }

        for (i, stack) in stacks.iter().enumerate() {
            println!("{}: {:?}", i + 1, stack);
        }

        let mut tops = Vec::new();
        for stack in stacks.iter() {
            if stack.len() > 0 {
                tops.push(stack[stack.len() - 1]);
            }
        }
        let s: String = tops.iter().collect();
        println!("Tops: {}", s);
    }
}

mod day_4 {
    use crate::utils::read_all_file;

    // Inclusive
    #[derive(Debug)]
    struct Range {
        start: usize,
        end: usize,
    }

    impl Range {
        fn of_str(input: &str) -> Self {
            let items: Vec<&str> = input.split('-').collect();
            assert_eq!(items.len(), 2);
            Self {
                start: items[0].parse::<usize>().unwrap(),
                end: items[1].parse::<usize>().unwrap(),
            }
        }

        fn contains(&self, other: &Self) -> bool {
            other.start >= self.start && other.end <= self.end
        }

        fn overlap(&self, other: &Self) -> bool {
            // [    ]
            //     [      ]
            //
            //          [        ]
            //     [      ]
            let left_overlap = other.end >= self.start && other.end <= self.end;
            let right_overlap = other.start >= self.start && other.start <= self.end;
            left_overlap || right_overlap
        }
    }

    pub fn run() {
        let lines = read_all_file("inputs/input4.txt");
        let _lines = vec![
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ];

        let mut contain_count = 0;
        let mut overlap_count = 0;
        for line in lines {
            let items: Vec<&str> = line.split(',').collect();
            assert_eq!(items.len(), 2);
            let r1 = Range::of_str(items[0]);
            let r2 = Range::of_str(items[1]);

            if r1.contains(&r2) || r2.contains(&r1) {
                println!("{:?} and {:?}. One contains the other", r1, r2);
                contain_count += 1;
            } else {
                println!("{:?} and {:?}. do not contain eachother", r1, r2);
            }

            if r1.overlap(&r2) || r2.overlap(&r1) {
                overlap_count += 1;
            }
        }

        println!("Fully contained ranges: {}", contain_count);
        println!("Overlapping ranges: {}", overlap_count);
    }
}

mod day_3 {
    use crate::utils::read_all_file;
    use std::collections::HashSet;

    fn to_val(c: &char) -> u32 {
        let v: u32 = (*c).into();

        if c.is_ascii_uppercase() {
            v - 38
        } else {
            v - 96
        }
    }

    pub fn run() {
        let lines = read_all_file("inputs/input3.txt");

        let mut priorities = 0;

        for line in lines.iter() {
            let compartment_size = line.len() / 2;
            let left = &line[0..compartment_size];
            let right = &line[compartment_size..line.len()];
            let left_charset = HashSet::<char>::from_iter(left.chars());
            let right_charset = HashSet::<char>::from_iter(right.chars());
            assert_eq!(left.len(), right.len());
            let intersection = left_charset
                .intersection(&right_charset)
                .collect::<Vec<&char>>();
            assert_eq!(1, intersection.len());

            priorities += to_val(intersection[0]);
        }

        println!("Priority Sum: {}", priorities);

        priorities = 0;
        let mut team = Vec::new();

        for line in lines {
            let set = HashSet::<char>::from_iter(line.chars());
            team.push(set);

            if team.len() == 3 {
                let intersect1: HashSet<char> =
                    team[0].intersection(&team[1]).map(|c| *c).collect();
                let intersect2 = intersect1.intersection(&team[2]).collect::<Vec<&char>>();
                assert_eq!(1, intersect2.len());
                priorities += to_val(intersect2[0]);
                team.clear();
            }
        }
        assert_eq!(team.len(), 0);

        println!("Team Priority Sum: {}", priorities);
    }
}

mod day_2 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn run() {
        let file = File::open("inputs/input2.txt").unwrap();
        let reader = BufReader::new(file);

        let mut score = 0usize;

        let lines: Vec<Vec<String>> = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let parts: Vec<String> = line.split(' ').map(String::from).collect();
                assert_eq!(2, parts.len());
                parts
            })
            .collect();

        for parts in lines.iter() {
            let opponent_plays = Hand::of_char(&parts[0]);
            let you_play = Hand::of_char(&parts[1]);

            score += you_play.score();
            score += you_play.outcome(&opponent_plays).score()
        }
        println!("p1: Total Score: {}", score);

        score = 0;

        for parts in lines.iter() {
            let opponent_plays = Hand::of_char(&parts[0]);
            let expected_outcome = HandResult::of_char(&parts[1]);

            score += expected_outcome.score();
            let hand_should_play = match expected_outcome {
                HandResult::Win => opponent_plays.loses_to(),
                HandResult::Draw => opponent_plays.draws_with(),
                HandResult::Loss => opponent_plays.beats(),
            };
            score += hand_should_play.score();
        }
        println!("p2: Total Score: {}", score);
    }

    enum Hand {
        Rock,
        Paper,
        Scissors,
    }
    enum HandResult {
        Win,
        Draw,
        Loss,
    }

    impl HandResult {
        fn score(&self) -> usize {
            match self {
                Self::Loss => 0,
                Self::Draw => 3,
                Self::Win => 6,
            }
        }

        fn of_char(c: &str) -> Self {
            match c {
                "X" => Self::Loss,
                "Y" => Self::Draw,
                "Z" => Self::Win,
                wrong => panic!("Invalid char to result: {}", wrong),
            }
        }
    }

    impl Hand {
        fn beats(&self) -> Self {
            match self {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            }
        }
        fn draws_with(&self) -> Self {
            match self {
                Self::Rock => Self::Rock,
                Self::Paper => Self::Paper,
                Self::Scissors => Self::Scissors,
            }
        }
        fn loses_to(&self) -> Self {
            match self {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            }
        }

        fn of_char(c: &str) -> Self {
            match c {
                "A" | "X" => Self::Rock,
                "B" | "Y" => Self::Paper,
                "C" | "Z" => Self::Scissors,
                wrong => panic!("Invalid char to hand: {}", wrong),
            }
        }

        fn score(&self) -> usize {
            match self {
                Self::Rock => 1,
                Self::Paper => 2,
                Self::Scissors => 3,
            }
        }

        /// Returns the result for playing self against [other]
        fn outcome(&self, other: &Self) -> HandResult {
            match (self, other) {
                (Self::Rock, Self::Rock)
                | (Self::Paper, Self::Paper)
                | (Self::Scissors, Self::Scissors) => HandResult::Draw,
                (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper) => HandResult::Win,
                _ => HandResult::Loss,
            }
        }
    }
}

fn day_1() {
    let file = File::open("inputs/input1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut _elf_num = 1;
    let mut elf_carry_sum: usize = 0;
    let mut elves = Vec::new();
    for line in reader.lines() {
        match line.unwrap().as_str() {
            "" => {
                //finalise elf
                elves.push(elf_carry_sum);
                elf_carry_sum = 0;
            }
            num => {
                let n = num.parse::<usize>().unwrap();
                elf_carry_sum += n;
            }
        }
    }
    if elf_carry_sum != 0 {
        elves.push(elf_carry_sum);
    }

    println!("Max size: {}", elves.iter().max().unwrap());

    elves.sort();
    let l = elves.len();
    let top_3 = &elves[l - 3..l];
    println!("Top 3 Sum: {:?} = {}", top_3, top_3.iter().sum::<usize>());
}
