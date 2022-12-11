use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let day = 11;
    match day {
        1 => day_1(),
        2 => day_2::run(),
        3 => day_3::run(),
        4 => day_4::run(),
        5 => day_5::run(),
        6 => day_6::run(),
        7 => day_7::run(),
        8 => day_8::run(),
        9 => day_9::run(),
        10 => day_10::run(),
        11 => day_11::run(),
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

mod day_11 {
    use itertools::Itertools;
    use num_bigint::BigInt;
    use num_traits::identities::Zero;
    use std::collections::VecDeque;

    enum Operation {
        Add(i32),
        Square,
        Multiply(i32),
    }
    impl Operation {
        fn apply(&self, in_: BigInt) -> BigInt {
            match self {
                Self::Add(a) => in_ + *a,
                Self::Square => in_.pow(2),
                Self::Multiply(m) => in_ * *m,
            }
        }
    }
    struct Monkey {
        inventory: Vec<BigInt>,
        operation: Operation,
        test__divisible_by: i32,
        true_target: i32,
        false_target: i32,
        inspect_count: usize,
    }

    struct MonkeyResult {
        item: BigInt,
        target: i32,
    }

    impl Monkey {
        fn new(
            inventory: Vec<i32>,
            operation: Operation,
            test__divisible_by: i32,
            true_target: i32,
            false_target: i32,
        ) -> Self {
            let inventory = inventory.into_iter().map(BigInt::from).collect();
            Self {
                inventory,
                operation,
                test__divisible_by,
                true_target,
                false_target,
                inspect_count: 0,
            }
        }

        fn process_items(&mut self, worry_attenuation: bool) -> Vec<MonkeyResult> {
            let mut results = Vec::new();
            self.inspect_count += self.inventory.len();

            for item in self.inventory.drain(..) {
                // Inspect, and apply operation
                let item = self.operation.apply(item);
                // Get bored and reduce

                let item = if worry_attenuation { item / 3 } else { item };

                let test_result = item.clone() % self.test__divisible_by;

                let target = if test_result.is_zero() {
                    self.true_target
                } else {
                    self.false_target
                };

                results.push(MonkeyResult { item, target });
            }
            results
        }
    }

    fn setup_monkeys() -> Vec<Monkey> {
        vec![
            Monkey::new(vec![54, 89, 94], Operation::Multiply(7), 17, 5, 3),
            Monkey::new(vec![66, 71], Operation::Add(4), 3, 0, 3),
            Monkey::new(vec![76, 55, 80, 55, 55, 96, 78], Operation::Add(2), 5, 7, 4),
            Monkey::new(
                vec![93, 69, 76, 66, 89, 54, 59, 94],
                Operation::Add(7),
                7,
                5,
                2,
            ),
            Monkey::new(vec![80, 54, 58, 75, 99], Operation::Multiply(17), 11, 1, 6),
            Monkey::new(vec![69, 70, 85, 83], Operation::Add(8), 19, 2, 7),
            Monkey::new(vec![89], Operation::Add(6), 2, 0, 1),
            Monkey::new(vec![62, 80, 58, 57, 93, 56], Operation::Square, 13, 6, 4),
        ]
    }

    fn run_worry_optional(worry_attenuation: bool) {
        let mut round_num = 0;
        let mut monkeys = setup_monkeys();

        let max_rounds = if worry_attenuation { 20 } else { 10000 };
        while round_num < max_rounds {
            for i in 0..monkeys.len() {
                let results = monkeys[i].process_items(worry_attenuation);
                for MonkeyResult { item, target } in results.iter() {
                    monkeys[*target as usize].inventory.push(item.clone());
                }
            }
            print!(".");
            if round_num % 20 == 0 {
                println!(" {}", round_num)
            }
            round_num += 1;
        }

        let monkey_business = monkeys
            .iter()
            .map(|monkey| monkey.inspect_count)
            .sorted()
            .rev()
            .take(2)
            .reduce(|a, b| a * b)
            .unwrap();
        println!("Monkey Business: {}", monkey_business);
    }
    pub fn run() {
        println!("With worry attentuation");
        run_worry_optional(true);
        println!("Without worry attentuation");
        run_worry_optional(false);
    }
}

mod day_10 {
    use crate::utils::read_all_file;

    struct Cpu {
        register: i32,
        cycle_num: usize,
        stored_values_at_times: Vec<i32>,
        rendered_rows: Vec<Vec<bool>>,
    }

    impl Cpu {
        fn new() -> Self {
            Self {
                register: 1,
                cycle_num: 0,
                stored_values_at_times: Vec::new(),
                rendered_rows: Vec::new(),
            }
        }

        fn step(&mut self) {
            self.cycle_num += 1;
            println!("Start of cycle: {}", self.cycle_num);
            let should_record = {
                if self.cycle_num >= 20 {
                    (self.cycle_num - 20) % 40 == 0
                } else {
                    false
                }
            };
            if should_record {
                self.stored_values_at_times
                    .push(self.register * (self.cycle_num as i32));
            }

            let row_number = (self.cycle_num - 1) / 40;
            let column_number = ((self.cycle_num - 1) % 40) as i32;
            if self.rendered_rows.len() <= row_number {
                self.rendered_rows.push(Vec::new());
            }

            let should_draw_pixel =
                column_number >= self.register - 1 && column_number <= self.register + 1;
            println!(
                "Drawing pixel: {} - (Sprite at {}, I am column {})",
                if should_draw_pixel { "#" } else { "." },
                self.register,
                column_number
            );
            self.rendered_rows[row_number].push(should_draw_pixel);
            self.render()
        }

        fn render(&self) {
            for row in self.rendered_rows.iter() {
                for pixel in row {
                    if *pixel {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
    }

    pub fn run() {
        let lines = read_all_file("inputs/input10.txt");

        let mut cpu = Cpu::new();

        for command in lines.iter() {
            // start cycle
            if command == "noop" {
                //increase_cycle
                cpu.step();
            } else {
                let parts: Vec<&str> = command.split(' ').collect();
                assert_eq!(parts.len(), 2);
                let modifier = parts[1].parse::<i32>().unwrap();
                //increase_cycle_by_two
                cpu.step();
                if false && cpu.cycle_num >= 40 {
                    break;
                }
                println!();
                cpu.step();
                cpu.register += modifier;
                println!(
                    "End of cycle, finishing execute (Register is now {})",
                    cpu.register
                );
            }
            if false && cpu.cycle_num >= 40 {
                break;
            }
            println!()
        }

        for snapshot in cpu.stored_values_at_times.iter() {
            println!("{}", snapshot);
        }

        println!(
            "Sum of snapshots: {}",
            cpu.stored_values_at_times.iter().sum::<i32>()
        );

        cpu.render();
    }
}

mod day_9 {
    use crate::utils::read_all_file;
    use itertools::Itertools;
    use std::fmt::Formatter;

    #[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct Pos {
        x: i32,
        y: i32,
    }

    enum Direction {
        Up,
        Down,
        Right,
        Left,
    }
    impl Direction {
        fn of_str(c: &str) -> Self {
            match c {
                "U" => Self::Up,
                "D" => Self::Down,
                "R" => Self::Right,
                "L" => Self::Left,
                _ => panic!("Unexpected char to direction ({})", c),
            }
        }
    }

    impl Pos {
        fn new() -> Self {
            Self { x: 0, y: 0 }
        }
        fn out_of_range_of(&self, other: &Self) -> bool {
            let dx = (self.x - other.x).abs();
            let dy = (self.y - other.y).abs();
            dx > 1 || dy > 1
        }

        fn move_(&mut self, direction: &Direction) {
            match direction {
                Direction::Up => self.y += 1,
                Direction::Down => self.y -= 1,
                Direction::Right => self.x += 1,
                Direction::Left => self.x -= 1,
            }
        }

        fn set_to(&mut self, other: &Self) {
            self.x = other.x;
            self.y = other.y;
        }
        fn to_move_towards(&self, other: &Self) -> Self {
            let mut new = self.clone();
            new.move_towards(other);
            new
        }
        fn move_towards(&mut self, other: &Self) {
            if self.out_of_range_of(other) {
                if self.x == other.x {
                    //move in y
                    if other.y > self.y {
                        self.y += 1;
                    } else {
                        self.y -= 1;
                    }
                } else if self.y == other.y {
                    //move in x
                    if other.x > self.x {
                        self.x += 1;
                    } else {
                        self.x -= 1;
                    }
                } else {
                    //move diagonally
                    if other.x > self.x {
                        self.x += 1;
                    } else {
                        self.x -= 1;
                    }
                    if other.y > self.y {
                        self.y += 1;
                    } else {
                        self.y -= 1;
                    }
                }
            }
        }
    }

    impl std::fmt::Display for Pos {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({},{})", self.x, self.y)
        }
    }

    struct Rope {
        knots: Vec<Pos>,
    }

    impl Rope {
        fn new(num_knots: usize) -> Self {
            Self {
                knots: vec![Pos::new(); num_knots],
            }
        }
        fn move_head(&mut self, direction: &Direction) {
            for i in 0..self.knots.len() {
                match i {
                    0 => self.knots[i].move_(direction),
                    _ => {
                        let new_knot = self.knots[i].to_move_towards(&self.knots[i - 1]);
                        self.knots[i].set_to(&new_knot);
                    }
                }
            }
        }

        fn tail_pos(&self) -> Pos {
            self.knots.last().unwrap().clone()
        }
    }

    fn run_two() {
        let lines = read_all_file("inputs/input9.txt");

        let mut head = Pos::new();
        let mut tail = Pos::new();
        let mut tail_positions = vec![tail.clone()];
        for line in lines.iter() {
            let parts: Vec<&str> = line.split(' ').collect();
            assert_eq!(parts.len(), 2);

            let direction = Direction::of_str(parts[0]);
            let steps = parts[1].parse::<i32>().unwrap();
            println!("Moving {}, {} times", parts[0], parts[1]);
            for _i in 0..steps {
                head.move_(&direction);
                tail.move_towards(&head);
                println!("H:{}, t:{}", head, tail);
                tail_positions.push(tail.clone());
            }
        }

        let count = tail_positions.into_iter().sorted().dedup().count();
        println!("Number of unique tail locations: {}", count);
    }

    fn run_many() {
        let lines = read_all_file("inputs/input9.txt");

        let mut rope = Rope::new(10);
        let mut tail_positions = vec![rope.tail_pos()];
        for line in lines.iter() {
            let parts: Vec<&str> = line.split(' ').collect();
            assert_eq!(parts.len(), 2);
            let direction = Direction::of_str(parts[0]);
            let steps = parts[1].parse::<i32>().unwrap();
            println!("Moving {}, {} times", parts[0], parts[1]);
            for _i in 0..steps {
                rope.move_head(&direction);
                tail_positions.push(rope.tail_pos());
            }
        }

        let count = tail_positions.into_iter().sorted().dedup().count();
        println!("Number of unique tail locations: {}", count);
    }

    pub fn run() {
        run_two();
        run_many();
    }
}

mod day_8 {
    use crate::utils::read_all_file;
    use array2d::Array2D;

    #[derive(Clone, Debug)]
    struct Tree {
        height: usize,
        visible: bool,
        scenic_score: usize,
    }

    impl Tree {
        fn of_char(c: char) -> Self {
            Tree {
                height: c.to_digit(10).unwrap() as usize,
                visible: false,
                scenic_score: 0,
            }
        }
    }

    fn build(lines: Vec<String>) -> Array2D<Tree> {
        let rows: Vec<Vec<Tree>> = lines
            .iter()
            .map(|line_string| line_string.chars().map(Tree::of_char).collect())
            .collect();
        Array2D::from_rows(&rows).unwrap()
    }

    fn find_visibility(
        trees: &Array2D<Tree>,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> bool {
        //walk in each direction from (x,y) and if we encounter a tree >= our size, not visible
        let tree_height = trees[(x, y)].height;

        let moves = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        moves
            .iter()
            .map(|(move_x, move_y)| {
                let mut position = (x as i32, y as i32);
                position.0 += move_x;
                position.1 += move_y;
                while position.0 >= 0
                    && position.0 < (width as i32)
                    && position.1 >= 0
                    && position.1 < (height as i32)
                {
                    if trees[(position.0 as usize, position.1 as usize)].height >= tree_height {
                        return false;
                    }
                    position.0 += move_x;
                    position.1 += move_y;
                }
                true
            })
            .any(|a| a)
    }

    fn find_scenic_score(
        trees: &Array2D<Tree>,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> usize {
        // TODO: Oof indeed this is very repeaty with [find_visible]
        //walk in each direction from (x,y) and if we encounter a tree >= our size, not visible
        let tree_height = trees[(x, y)].height;

        let moves = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let result = moves
            .iter()
            .map(|(move_x, move_y)| {
                let mut position = (x as i32, y as i32);
                let mut num_visible = 0;
                position.0 += move_x;
                position.1 += move_y;
                while position.0 >= 0
                    && position.0 < (width as i32)
                    && position.1 >= 0
                    && position.1 < (height as i32)
                {
                    num_visible += 1;
                    if trees[(position.0 as usize, position.1 as usize)].height >= tree_height {
                        return num_visible;
                    }
                    position.0 += move_x;
                    position.1 += move_y;
                }
                num_visible
            })
            .reduce(|a, b| a * b)
            .unwrap();
        result as usize
    }

    fn analyse_trees(mut trees: Array2D<Tree>) -> Array2D<Tree> {
        let width = trees.num_columns();
        let height = trees.num_rows();
        for i in 0..width {
            for j in 0..height {
                let is_visible = find_visibility(&trees, i, j, width, height);
                let scenic_score = find_scenic_score(&trees, i, j, width, height);
                trees[(i, j)].visible = is_visible;
                trees[(i, j)].scenic_score = scenic_score;
            }
        }
        trees
    }

    fn count_of_visible(trees: &Array2D<Tree>) -> usize {
        let mut count = 0;
        let width = trees.num_columns();
        let height = trees.num_rows();
        for i in 0..width {
            for j in 0..height {
                if trees[(i, j)].visible {
                    count += 1;
                }
            }
        }
        count
    }

    fn max_scenic_score(trees: &Array2D<Tree>) -> usize {
        let mut max = 0;
        let width = trees.num_columns();
        let height = trees.num_rows();
        for i in 0..width {
            for j in 0..height {
                if trees[(i, j)].scenic_score > max {
                    max = trees[(i, j)].scenic_score;
                }
            }
        }
        max
    }

    pub fn run() {
        let lines = read_all_file("inputs/input8.txt");
        let trees = analyse_trees(build(lines));
        println!("Num visible: {}", count_of_visible(&trees));
        println!("Highest scenic score: {}", max_scenic_score(&trees));
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

    fn sum_of_sizes_lt(node: &Node, threshold: usize, acc: usize) -> usize {
        match node {
            Node::File(_file) => acc,
            Node::Dir(dir) => {
                let mut acc = acc;
                let size = dir.borrow().size_of();
                if size <= threshold {
                    acc += size
                }
                for child in dir.borrow().children.iter() {
                    acc = sum_of_sizes_lt(child, threshold, acc);
                }
                acc
            }
        }
    }

    fn dir_sizes(node: &Node, mut acc: Vec<usize>) -> Vec<usize> {
        match node {
            Node::File(_file) => acc,
            Node::Dir(dir) => {
                acc.push(dir.borrow().size_of());
                for child in dir.borrow().children.iter() {
                    acc = dir_sizes(child, acc);
                }
                acc
            }
        }
    }

    fn build_fs(lines: Vec<String>) -> Node {
        // Lines are either a command (CD or LS) or an ls result

        let root_dir = Directory {
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

    fn smallest_freeable_dir(dir_sizes: Vec<usize>, fs: &Node) -> usize {
        let total = 70000000usize;
        let needed = 30000000usize;

        let currently_available = total - fs.size_of();
        let min_to_free = needed - currently_available;
        *dir_sizes
            .iter()
            .filter(|size| **size > min_to_free)
            .min()
            .unwrap()
    }

    pub fn run() {
        let lines = read_all_file("inputs/input7.example.txt");
        let fs = build_fs(lines);
        print_fs(&fs, 0);
        println!(
            "Sum of dirs under 100,000: {}",
            sum_of_sizes_lt(&fs, 100000, 0)
        );

        let dir_sizes_ = dir_sizes(&fs, vec![]);
        println!(
            "Smallest freeable dir size: {}",
            smallest_freeable_dir(dir_sizes_, &fs)
        );

        let do_full = true;
        if do_full {
            let lines = read_all_file("inputs/input7.txt");
            let fs = build_fs(lines);
            print_fs(&fs, 0);
            println!(
                "Sum of dirs under 100,000: {}",
                sum_of_sizes_lt(&fs, 100000, 0)
            );

            let dir_sizes_ = dir_sizes(&fs, vec![]);
            println!(
                "Smallest freeable dir size: {}",
                smallest_freeable_dir(dir_sizes_, &fs)
            );
        }
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
