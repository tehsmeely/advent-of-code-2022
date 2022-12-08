use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let day = 7;
    match day {
        1 => day_1(),
        2 => day_2::run(),
        3 => day_3::run(),
        4 => day_4::run(),
        5 => day_5::run(),
        6 => day_6::run(),
        7 => day_7::run(),
        _ => panic!("Unexpected day {}", day),
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

mod day_7 {
    use crate::utils::read_all_file;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct Directory {
        name: String,
        children: Vec<Node>,
    }
    struct File {
        name: String,
        size: usize,
    }

    enum Node {
        File(File),
        Dir(Rc<RefCell<Directory>>),
    }

    impl Node {
        fn size_of(&self) -> usize {
            match self {
                Self::File(file) => file.size_of(),
                Self::Dir(dir) => dir.borrow().size_of(),
            }
        }
        fn dir_exn(&mut self) -> Rc<RefCell<Directory>> {
            match self {
                Self::File(_) => panic!("Can't fetch directory from File node"),
                Self::Dir(dir) => dir.clone(),
            }
        }
    }

    impl File {
        fn size_of(&self) -> usize {
            self.size
        }
    }

    impl Directory {
        fn size_of(&self) -> usize {
            let mut size = 0;
            for child in self.children.iter() {
                size += child.size_of()
            }
            size
        }
    }

    fn print_fs(node: &Node, indent: usize) {
        let indent_s = " ".repeat(indent * 2);
        match node {
            Node::File(file) => println!("{} - {} (file, size={})", indent_s, file.name, file.size),
            Node::Dir(dir) => {
                println!("{} - {} (dir)", indent_s, dir.borrow().name);
                for child in dir.borrow().children.iter() {
                    print_fs(child, indent + 1);
                }
            }
        }
    }

    fn build_fs(lines: Vec<String>) -> Node {
        // Lines are either a command (CD or LS) or an ls result

        let mut root_dir = Directory {
            name: "/".into(),
            children: Vec::new(),
        };
        let mut root = Node::Dir(Rc::new(RefCell::new(root_dir)));
        let mut pwd = vec![root.dir_exn()];
        for line in lines.iter().skip(1) {
            let parts: Vec<String> = line.split(' ').map(|s| s.into()).collect();
            if parts[0] == "$" {
                if parts[1] == "ls" {
                    // do nothing actually
                } else if parts[1] == "cd" {
                    if parts[2] == ".." {
                        pwd.pop();
                    }
                    if parts[2] != ".." {
                        let going_into = pwd
                            .last()
                            .unwrap()
                            .borrow()
                            .children
                            .iter()
                            .find_map(|nod| match nod {
                                Node::File(_) => None,
                                Node::Dir(dir) => {
                                    if dir.borrow().name == parts[2] {
                                        Some(dir.clone())
                                    } else {
                                        None
                                    }
                                }
                            })
                            .unwrap();
                        pwd.push(going_into);
                    }
                }
            } else {
                //must be an ls result
                assert_eq!(parts.len(), 2);
                let node = if parts[0] == "dir" {
                    let new_dir = Directory {
                        name: parts[1].clone(),
                        children: Vec::new(),
                    };
                    Node::Dir(Rc::new(RefCell::new(new_dir)))
                } else {
                    let size = parts[0].parse::<usize>().unwrap();
                    let file = File {
                        name: parts[1].clone(),
                        size,
                    };
                    Node::File(file)
                };
                pwd.last().unwrap().borrow_mut().children.push(node);
            }
        }
        root
    }

    pub fn run() {
        let lines = read_all_file("inputs/input7.example.txt");
        let fs = build_fs(lines);
        print_fs(&fs, 0);
        let lines = read_all_file("inputs/input7.txt");
        let fs = build_fs(lines);
        print_fs(&fs, 0);
    }
}

mod day_6 {
    use crate::utils::read_all_file;
    use std::collections::{HashSet, VecDeque};

    pub fn run() {
        let lines = read_all_file("inputs/input6.txt");
        let signal = &lines[0];

        let mut buf = VecDeque::new();

        let mut i = 0;
        for c in signal.chars() {
            i += 1;
            buf.push_back(c);
            if buf.len() > 4 {
                buf.pop_front();
            }
            if buf.len() == 4 {
                let window_set: HashSet<char> = buf.iter().map(|c| *c).collect();
                if window_set.len() == 4 {
                    //window is uniq chars!
                    println!("Hit a marker: {:?}, Offset: {}", buf, i);
                    break;
                }
            }
        }

        let mut i = 0;
        for c in signal.chars() {
            i += 1;
            buf.push_back(c);
            if buf.len() > 14 {
                buf.pop_front();
            }
            if buf.len() == 14 {
                let window_set: HashSet<char> = buf.iter().map(|c| *c).collect();
                if window_set.len() == 14 {
                    //window is uniq chars!
                    println!("Hit a message marker: {:?}, Offset: {}", buf, i);
                    break;
                }
            }
        }
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
