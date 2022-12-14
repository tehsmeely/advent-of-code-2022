use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let day = 16;
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
        12 => day_12::run(),
        13 => day_13::run(),
        14 => day_14::run(),
        15 => day_15::run(),
        16 => day_16::run(),
        _ => panic!("Unexpected day {}", day),
    }
}

mod utils {
    use num_traits::{Num, PrimInt, Signed, Unsigned};
    use std::fmt::{Display, Formatter};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::ops::{Add, Sub};

    #[derive(Debug)]
    pub struct V2<I> {
        pub x: I,
        pub y: I,
    }

    impl<I: Display> Display for V2<I> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({},{})", self.x, self.y)
        }
    }

    impl<I> V2<I> {
        pub fn new(x: I, y: I) -> V2<I> {
            Self { x, y }
        }
    }

    pub trait GridDist {
        type Out;
        fn grid_dist(&self, other: &Self) -> Self::Out;
    }

    impl<I> GridDist for V2<I>
    where
        I: PrimInt,
    {
        type Out = I;

        fn grid_dist(&self, other: &Self) -> Self::Out {
            let dx = if self.x > other.x {
                self.x - other.x
            } else {
                other.x - self.x
            };
            let dy = if self.y > other.y {
                self.y - other.y
            } else {
                other.y - self.y
            };
            dx + dy
        }
    }

    pub fn read_all_file(filename: &str) -> Vec<String> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        reader.lines().filter_map(Result::ok).collect()
    }
}

mod day_16 {
    pub fn run() {}
}

mod day_15 {
    use crate::utils::{read_all_file, GridDist, V2};
    use itertools::Itertools;
    use regex::Regex;
    use std::collections::HashSet;
    use std::ops::ControlFlow;

    fn parse_line(line: &str) -> (V2<i32>, V2<i32>) {
        let reg = r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)";
        let re = Regex::new(reg).unwrap();
        println!("{}", line);
        let caps = re.captures(line).unwrap();

        let sens_x: i32 = caps[1].parse().unwrap();
        let sens_y: i32 = caps[2].parse().unwrap();
        let beac_x: i32 = caps[3].parse().unwrap();
        let beac_y: i32 = caps[4].parse().unwrap();

        (V2::new(sens_x, sens_y), V2::new(beac_x, beac_y))
    }

    pub fn run() {
        let lines = read_all_file("inputs/input15.txt");

        let sensors_and_beacons: Vec<(V2<i32>, V2<i32>)> =
            lines.iter().map(|s| parse_line(&s)).collect();

        let part_1 = false;
        let part_2 = true;
        if part_1 {
            // For each Sensor, we know there's no other beacon within N radius of it
            // where N is the quoted distance

            let target_row = 2000000;
            let mut set_x_on_target_row: HashSet<i32> = HashSet::new();
            for (sensor, beacon) in sensors_and_beacons.iter() {
                //println!("{:?} -> {:?}", sensor, beacon);
                let sensor_distance = sensor.grid_dist(&beacon);
                //println!("dist: {:?}", sensor_distance);

                /*
                y = 1, distance = 9. Variable target row:
                07 |....#######..  -> (y + 9) - 7 = (10) - 8 = 3
                08 |.....#####...  -> (y + 9) - 8 = (10) - 8 = 2
                09 |......###....  -> (y + 9) - 9 = (10) - 9 = 1
                10 |.......#.....  -> (y + 9) - 10 = (10) - 10 = 0
                */
                /*
                y = 15, distance = 8. Variable target row:
                07 |.......#.....  -> (y - 8) - 7 = 7 - (15 - 8) = 7 - 7 = 0
                08 |......###....  -> (y + 8) - 8 = 8 - (15 - 8) = 8 - 7 = -1
                09 |.....#####...  -> (y + 8) - 9 = 9 - (7) = 2
                10 |....#######..  -> (y + 8) - 10 = 10 - (7) = 3
                */
                let half_width = if sensor.y < target_row {
                    sensor.y + sensor_distance - target_row
                } else {
                    target_row - (sensor.y - sensor_distance)
                };
                if half_width >= 0 {
                    println!(
                        "Sensor at {} (dist {}) impacts target row. Overlap width = {}",
                        sensor,
                        sensor_distance,
                        (half_width * 2) + 1
                    );
                    // set for x and x(+-) half_width
                    for x in sensor.x - half_width..=sensor.x + half_width {
                        set_x_on_target_row.insert(x);
                    }
                }
            }
            // Prune known beacons:
            for (_sensor, beacon) in sensors_and_beacons.iter() {
                if beacon.y == target_row {
                    set_x_on_target_row.remove(&beacon.x);
                }
            }

            let x_min = set_x_on_target_row.iter().min().unwrap();
            let x_max = set_x_on_target_row.iter().max().unwrap();
            let render = false;
            if render {
                println!();
                for j in 0..2 {
                    for i in x_min - 1..=x_max + 1 {
                        if j == 0 {
                            if i % 5 == 0 {
                                print!("{}", i)
                            } else if ((i - 1) % 5 == 0) && i > 10 {
                                ()
                            } else {
                                print!(" ")
                            }
                        } else {
                            if set_x_on_target_row.contains(&i) {
                                print!("#");
                            } else {
                                print!(".");
                            }
                        }
                    }
                    println!();
                }
            }
            println!("Set positions:{}", set_x_on_target_row.len());
        } else if part_2 {
            let _result = find(&sensors_and_beacons);
        } else {
            'outer_loop: for y in 0..=4_000_000 {
                for x in 0..=4_000_000 {
                    let pos = V2::new(x, y);
                    let mut is_undetected = true;

                    'beacon_loop: for (sensor, beacon) in sensors_and_beacons.iter() {
                        let sensor_distance = sensor.grid_dist(&beacon);
                        let test_pos_distance = pos.grid_dist(sensor);

                        if test_pos_distance <= sensor_distance {
                            is_undetected = false;
                            break 'beacon_loop;
                        }
                    }

                    if is_undetected {
                        println!("Undetected position is: {}", pos);
                        println!("Tuning freq : {}", (4000000 * x) + y);
                        break 'outer_loop;
                    }
                }
                if y % 10000 == 0 {
                    println!(". {}", y)
                }
            }
        }
    }

    fn find(sensors_and_beacons: &Vec<(V2<i32>, V2<i32>)>) -> V2<i32> {
        use rayon::prelude::*;
        let result = (0..=4_000_000u64).into_par_iter().try_for_each(|y| {
            (0..=4_000_000u64).into_par_iter().try_for_each(|x| {
                let pos = V2::new(x as i32, y as i32);
                let mut is_undetected = true;

                'beacon_loop: for (sensor, beacon) in sensors_and_beacons.iter() {
                    let sensor_distance = sensor.grid_dist(beacon);
                    let test_pos_distance = pos.grid_dist(sensor);

                    if test_pos_distance <= sensor_distance {
                        is_undetected = false;
                        break 'beacon_loop;
                    }
                }

                if is_undetected {
                    println!("Undetected position is: {}", pos);
                    println!("Tuning freq : {}", (4000000 * x) + y);
                    ControlFlow::Break(pos)
                } else {
                    ControlFlow::Continue(())
                }
            })
        });
        match result {
            ControlFlow::Break(pos) => pos,
            ControlFlow::Continue(()) => panic!("Loop ended with no pos found!"),
        }
    }
}

mod day_14 {
    use crate::utils::read_all_file;
    use array2d::Array2D;

    #[derive(Clone, Debug, PartialEq)]
    enum Cell {
        Rock,
        Air,
        Sand,
    }

    type Grid = Array2D<Cell>;

    struct Limits {
        x_min: Option<i32>,
        x_max: Option<i32>,
        y_min: Option<i32>,
        y_max: Option<i32>,
    }
    impl Limits {
        fn new() -> Self {
            Self {
                x_min: None,
                x_max: None,
                y_min: None,
                y_max: None,
            }
        }

        fn maybe_set(&mut self, (x, y): &(i32, i32)) {
            self.maybe_set_x(*x);
            self.maybe_set_y(*y);
        }
        fn maybe_set_x(&mut self, x: i32) {
            let x_min = match self.x_min {
                Some(old_x) => {
                    if old_x < x {
                        old_x
                    } else {
                        x
                    }
                }
                None => x,
            };
            let x_max = match self.x_max {
                Some(old_x) => {
                    if old_x > x {
                        old_x
                    } else {
                        x
                    }
                }
                None => x,
            };
            self.x_min = Some(x_min);
            self.x_max = Some(x_max)
        }
        fn maybe_set_y(&mut self, y: i32) {
            let y_min = match self.y_min {
                Some(old_y) => {
                    if old_y < y {
                        old_y
                    } else {
                        y
                    }
                }
                None => y,
            };
            let y_max = match self.y_max {
                Some(old_y) => {
                    if old_y > y {
                        old_y
                    } else {
                        y
                    }
                }
                None => y,
            };
            self.y_min = Some(y_min);
            self.y_max = Some(y_max)
        }
    }

    fn interpolate_points(from: &(i32, i32), to: &(i32, i32)) -> Vec<(i32, i32)> {
        let step = ((to.0 - from.0).signum(), (to.1 - from.1).signum());

        println!("From {:?} to {:?}. Step {:?}", from, to, step);
        let mut pos = from.clone();
        let mut result = vec![pos.clone()];
        let mut count = 0;
        while pos != *to {
            pos.0 += step.0;
            pos.1 += step.1;
            println!("pos:{:?}", pos);
            result.push(pos.clone());
            count += 1;
            if count > 100 {
                panic!("Iteration limit");
            }
        }
        result
    }

    fn populate_grid(line: &str, grid: &mut Grid, limits: &mut Limits) {
        let points: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|s| {
                let elems: Vec<&str> = s.split(',').collect();
                assert_eq!(elems.len(), 2);
                (elems[0].parse().unwrap(), elems[1].parse().unwrap())
            })
            .collect();

        let mut prev = points.first().unwrap();
        limits.maybe_set(prev);
        for end_point in points.iter().skip(1) {
            for point in interpolate_points(prev, end_point) {
                grid[au(point)] = Cell::Rock
            }
            prev = end_point;
            limits.maybe_set(prev);
        }
    }

    fn au((i, j): (i32, i32)) -> (usize, usize) {
        // Note this flips as array2d is indexed as [(row, column)] which is basically [(y, x)]
        (j as usize, i as usize)
    }

    fn spawn_sand(grid: &mut Grid, spawn_point: &(i32, i32)) {
        grid[au(*spawn_point)] = Cell::Sand;
    }

    fn draw_grid(grid: &Grid, limits: &Limits, sand_spawn: &(i32, i32)) {
        for row in limits.y_min.unwrap()..=limits.y_max.unwrap() {
            for col in limits.x_min.unwrap()..=limits.x_max.unwrap() {
                if (col, row) == *sand_spawn {
                    print!("+");
                } else {
                    print!(
                        "{}",
                        match grid.get(row as usize, col as usize).unwrap() {
                            Cell::Rock => "#",
                            Cell::Air => ".",
                            Cell::Sand => "o",
                        }
                    );
                }
            }
            println!()
        }
    }

    fn is_sand(grid: &Grid, col: i32, row: i32) -> bool {
        match grid.get(row as usize, col as usize).unwrap() {
            Cell::Sand => true,
            _ => false,
        }
    }
    fn is_free(grid: &Grid, col: i32, row: i32) -> bool {
        match grid.get(row as usize, col as usize).unwrap() {
            Cell::Air => true,
            _ => false,
        }
    }

    fn try_move(from: (i32, i32), grid: &Grid) -> Option<(i32, i32)> {
        //try straight down
        let col = from.0;
        let row = from.1;
        if is_free(grid, col, row + 1) {
            return Some((col, row + 1));
        }

        //then down and left
        if is_free(grid, col - 1, row + 1) {
            return Some((col - 1, row + 1));
        }

        // then down and right
        if is_free(grid, col + 1, row + 1) {
            return Some((col + 1, row + 1));
        }

        None
    }

    enum UpdateResult {
        StillUpdating((i32, i32)),
        AtRest,
        OutOfBottom,
    }

    fn update_grid(sand_pos: (i32, i32), grid: &mut Grid, limits: &mut Limits) -> UpdateResult {
        // iterate from bottom up

        let row = sand_pos.1;
        let col = sand_pos.0;
        if is_sand(grid, col, row) {
            if let Some((col2, row2)) = try_move((col, row), grid) {
                grid[au((col, row))] = Cell::Air;
                grid[au((col2, row2))] = Cell::Sand;

                // Expand limits if sand is pushed out to the edges
                limits.maybe_set_x(col2);

                if row2 > limits.y_max.unwrap() {
                    return UpdateResult::OutOfBottom;
                } else {
                    return UpdateResult::StillUpdating((col2, row2));
                }
            } else {
                UpdateResult::AtRest
            }
        } else {
            panic!("Sand wasn't found at location claimed to be sand pos");
        }
    }

    fn update_until_at_rest(sand_start: &(i32, i32), grid: &mut Grid, limits: &mut Limits) -> bool {
        let mut sand_pos = *sand_start;
        loop {
            match update_grid(sand_pos, grid, limits) {
                UpdateResult::StillUpdating(new_sand_pos) => {
                    sand_pos = new_sand_pos;
                }
                UpdateResult::AtRest => return false,
                UpdateResult::OutOfBottom => return true,
            }
        }
    }

    pub fn run2() {
        // Make an array
        // Being really inefficient with the width here, soz
        let mut grid = Array2D::filled_with(Cell::Air, 1000, 2000);

        let mut limits = Limits::new();

        // find and set sand spawn
        let sand_spawn = (500, 0);
        limits.maybe_set(&sand_spawn);

        // Populate rocks by tracing paths from input
        let lines = read_all_file("inputs/input14.txt");
        for line in lines {
            populate_grid(&line, &mut grid, &mut limits);
        }

        // add infinite bottom plane
        let plane_y = limits.y_max.unwrap() + 2;
        for x in 0..2000 {
            grid[au((x, plane_y))] = Cell::Rock;
        }
        limits.maybe_set_y(plane_y);

        draw_grid(&grid, &limits, &sand_spawn);

        // Spawn sand and update rows from bottom up
        // if sand, apply move rules
        let mut spawn_count = 0;

        while spawn_count < 100000 {
            spawn_sand(&mut grid, &sand_spawn);
            let sand_fell_out_of_my_bottom =
                update_until_at_rest(&sand_spawn, &mut grid, &mut limits);
            print!(".");
            if spawn_count % 50 == 0 {
                println!();
            }

            if sand_fell_out_of_my_bottom {
                panic!("Sand should not be falling out the bottom of an infinite plane");
            }
            spawn_count += 1;

            // we stop this time if we came to rest and sand occupies the spawn
            if grid[au(sand_spawn)] == Cell::Sand {
                break;
            }
        }
        println!();
        // Don't decr spawn count because the last sand that spawned stuck around as per our end detection rules

        draw_grid(&grid, &limits, &sand_spawn);

        println!("Finished, spawn count: {}", spawn_count);
    }

    pub fn _run1() {
        // Make an array
        let mut grid = Array2D::filled_with(Cell::Air, 1000, 1000);

        let mut limits = Limits::new();

        // find and set sand spawn
        let sand_spawn = (500, 0);
        limits.maybe_set(&sand_spawn);

        // Populate rocks by tracing paths from input
        let lines = read_all_file("inputs/input14.txt");
        for line in lines {
            populate_grid(&line, &mut grid, &mut limits);
        }

        draw_grid(&grid, &limits, &sand_spawn);

        // Spawn sand and update rows from bottom up
        // if sand, apply move rules
        let mut sand_fell_out_of_my_bottom = false;

        let mut spawn_count = 0;

        while !sand_fell_out_of_my_bottom && spawn_count < 1000 {
            spawn_sand(&mut grid, &sand_spawn);
            sand_fell_out_of_my_bottom = update_until_at_rest(&sand_spawn, &mut grid, &mut limits);
            //draw_grid(&grid, &limits, &sand_spawn);
            spawn_count += 1;
        }

        // decr since the last spawned sand fell out of the world as per the rules
        spawn_count -= 1;

        println!("Finished, spawn count: {}", spawn_count);
    }

    pub fn run() {
        run2();
    }
}

mod day_13 {
    use crate::utils::read_all_file;
    use itertools::Itertools;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::cmp::Ordering;
    use std::fmt::{Display, Formatter};

    #[derive(Deserialize, Serialize, Debug, Clone)]
    enum ListItem {
        List(Vec<ListItem>),
        Integer(i32),
    }

    impl Display for ListItem {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Integer(i) => write!(f, "{}", i),
                Self::List(l) => {
                    let lv = l.iter().map(|v| format!("{}", v)).join(",");
                    write!(f, "[{}]", lv)
                }
            }
        }
    }

    impl ListItem {
        fn from_value(v: Value) -> Self {
            match v {
                Value::Array(vals) => Self::List(vals.into_iter().map(Self::from_value).collect()),
                Value::Number(num) => {
                    let i = num.as_i64().unwrap() as i32;
                    Self::Integer(i)
                }
                other => {
                    panic!("Unexpected value: {}", other)
                }
            }
        }

        fn cmp(&self, other: &Self) -> Ordering {
            match (self, other) {
                (Self::List(one), Self::List(two)) => {
                    // Record if one is longer
                    let (one_longer, two_longer) = (one.len() > two.len(), one.len() < two.len());

                    for (a, b) in one.iter().zip(two.iter()) {
                        match a.cmp(b) {
                            Ordering::Equal => (), // continue
                            other => return other,
                        }
                    }

                    // we hit the end of the equal matched list, the one with data left is bigger
                    match (one_longer, two_longer) {
                        (true, false) => Ordering::Greater,
                        (false, true) => Ordering::Less,
                        _ => Ordering::Equal,
                    }
                }
                (Self::Integer(one), Self::Integer(two)) => one.cmp(two),
                (Self::Integer(one), Self::List(two)) => {
                    Self::List(vec![Self::Integer(one.clone())]).cmp(&Self::List(two.clone()))
                }
                (Self::List(one), Self::Integer(two)) => {
                    Self::List(one.clone()).cmp(&Self::List(vec![Self::Integer(two.clone())]))
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    struct Packet(ListItem);

    fn parse_line(line: &str) -> Packet {
        let de = serde_json::de::from_str(line).unwrap();

        match de {
            Value::Array(array) => Packet(ListItem::List(
                array.into_iter().map(ListItem::from_value).collect(),
            )),
            other => {
                panic!("Toplevel should always be Array: {}", other);
            }
        }
    }

    pub fn run() {
        let part_1 = false;

        match part_1 {
            true => run_part1(),
            false => run_part2(),
        }
    }
    pub fn run_part2() {
        let lines = read_all_file("inputs/input13.txt");

        let divider1 = ListItem::List(vec![ListItem::List(vec![ListItem::Integer(6)])]);
        let divider2 = ListItem::List(vec![ListItem::List(vec![ListItem::Integer(2)])]);

        let mut packets = Vec::new();
        for line in lines {
            if line == "" {
            } else {
                packets.push(parse_line(&line));
            }
        }

        packets.push(Packet(divider1.clone()));
        packets.push(Packet(divider2.clone()));

        packets.sort_by(|a, b| a.0.cmp(&b.0));

        let mut div1 = None;
        let mut div2 = None;

        for (i, packet) in packets.iter().enumerate() {
            if let Ordering::Equal = packet.0.cmp(&divider1) {
                div1 = Some(i + 1);
            }
            if let Ordering::Equal = packet.0.cmp(&divider2) {
                div2 = Some(i + 1);
            }
        }

        println!("Dividier index multiple: {}", div1.unwrap() * div2.unwrap());
    }
    pub fn run_part1() {
        let lines = read_all_file("inputs/input13.txt");

        let mut packet_a = None;
        let mut packet_b = None;

        let mut packet_pairs = Vec::new();

        for line in lines {
            if line == "" {
                match (packet_a, packet_b) {
                    (Some(pa), Some(pb)) => packet_pairs.push((pa, pb)),
                    (pa, pb) => panic!(
                        "Hit empty line but did not have two packets! pa: {:?}, pb: {:?}",
                        pa, pb
                    ),
                };
                packet_a = None;
                packet_b = None;
            } else {
                let packet = parse_line(&line);

                println!("Packet: {:?}", packet);

                if packet_a.is_none() {
                    packet_a = Some(packet);
                } else {
                    packet_b = Some(packet);
                }
            }
        }

        let mut right_ordered_sum = 0;

        for (i, (p1, p2)) in packet_pairs.into_iter().enumerate() {
            match p1.0.cmp(&p2.0) {
                Ordering::Less => right_ordered_sum += i + 1,
                Ordering::Equal => println!("Packets were Equal!:  {:?}=={:?}", p1, p2),
                Ordering::Greater => (),
            }
        }

        println!("Sum: {}", right_ordered_sum);
    }
}

mod day_12 {
    use crate::utils::read_all_file;
    use array2d::Array2D;
    use num_integer::Roots;

    #[derive(Clone, Debug)]
    struct MapCell {
        height: i32,
        is_start: bool,
        is_end: bool,
    }
    impl MapCell {
        fn of_char(c: char) -> Self {
            let is_start = c == 'S';
            let is_end = c == 'E';
            let height = match c {
                'S' => 'a' as i32,
                'E' => 'z' as i32,
                c => c as i32,
            } - 97;
            Self {
                height,
                is_start,
                is_end,
            }
        }

        fn can_move_to(&self, other: &Self) -> bool {
            other.height - 1 <= self.height
        }
    }

    fn build_grid(lines: Vec<String>) -> Array2D<MapCell> {
        let rows: Vec<Vec<MapCell>> = lines
            .iter()
            .map(|line| line.chars().map(MapCell::of_char).collect())
            .collect();

        Array2D::from_rows(&rows).unwrap()
    }

    fn find_beginning_and_end(grid: &Array2D<MapCell>) -> ((i32, i32), (i32, i32)) {
        let mut start = None;
        let mut end = None;
        for (j, row) in grid.rows_iter().enumerate() {
            for (i, cell) in row.enumerate() {
                if cell.is_start {
                    start = Some((i as i32, j as i32));
                }
                if cell.is_end {
                    end = Some((i as i32, j as i32));
                }
            }
        }
        (start.unwrap(), end.unwrap())
    }
    const ORTHOG_NEIGHBOURS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    fn calc_path(grid: Array2D<MapCell>, start_override: Option<(i32, i32)>) -> Option<usize> {
        let (start, end) = find_beginning_and_end(&grid);

        let start = match start_override {
            Some(s) => s,
            None => start,
        };

        fn au((i, j): (i32, i32)) -> (usize, usize) {
            // Note this flips as array2d is indexed as [(row, column)] which is basically [(y, x)]
            (j as usize, i as usize)
        }

        fn get_neighbours(
            (x, y): (i32, i32),
            grid: &Array2D<MapCell>,
            x_max: i32,
            y_max: i32,
        ) -> Vec<((i32, i32), i32)> {
            ORTHOG_NEIGHBOURS
                .into_iter()
                .map(|(i, j)| (x + i, y + j))
                .filter(|(new_x, new_y)| {
                    *new_x >= 0i32 && *new_x < x_max && *new_y >= 0i32 && *new_y < y_max
                })
                .filter(|n| grid[au((x, y))].can_move_to(&grid[au(*n)]))
                .map(|pos| (pos, 1))
                .collect()
        }

        fn calc_distance(from: (i32, i32), to: (i32, i32)) -> i32 {
            ((from.0 - to.0).pow(2) + (from.1 + to.1).pow(2)).sqrt()
        }

        let x_max = grid.num_columns() as i32;
        let y_max = grid.num_rows() as i32;
        println!("Size of grid: cols: {}, rows: {}", x_max, y_max);

        if let Some((path, _cost)) = pathfinding::prelude::astar(
            &start,
            |pos| get_neighbours(*pos, &grid, x_max, y_max),
            |p| calc_distance(*p, end),
            |p| *p == end,
        ) {
            //let path: HashMap<(i32, i32), i32> = path.iter().collect();

            println!("Path: {:?}", path);
            println!("Len: {}", path.len());
            for y in 0..y_max {
                for x in 0..x_max {
                    print!("{}", if path.contains(&(x, y)) { "X" } else { " " })
                }
                println!();
            }

            Some(path.len() - 1)
        } else {
            None
        }
    }

    pub fn run() {
        let lines = read_all_file("inputs/input12.txt");
        let grid = build_grid(lines);

        for row in grid.rows_iter() {
            for cell in row {
                print!("[{:02}]", cell.height);
            }
            println!();
        }

        println!(
            "Using actual start. Steps: {}",
            calc_path(grid.clone(), None).unwrap()
        );

        let mut possible_starts: Vec<(i32, i32)> = Vec::new();

        let x_max = grid.num_columns() as i32;
        let y_max = grid.num_rows() as i32;
        for y in 0..y_max {
            for x in 0..x_max {
                if grid.get(y as usize, x as usize).unwrap().height == 0 {
                    possible_starts.push((x, y));
                }
            }
            println!();
        }

        println!(
            "From choosing from {} a height starts. {}",
            possible_starts.len(),
            possible_starts
                .into_iter()
                .filter_map(|start| calc_path(grid.clone(), Some(start)))
                .min()
                .unwrap()
        )
    }
}

mod day_11 {
    use core::convert::From;
    use itertools::Itertools;
    use num_bigint::BigInt;
    use num_integer::Integer;
    use num_traits::identities::Zero;
    use num_traits::ToPrimitive;
    use std::ops::{AddAssign, MulAssign};

    enum Operation {
        Add(i32),
        Square,
        Multiply(i32),
    }
    impl Operation {
        fn apply_mut(&self, in_: &mut BigInt) {
            match self {
                Self::Add(a) => in_.add_assign(*a),
                Self::Square => *in_ = in_.pow(2),
                Self::Multiply(m) => in_.mul_assign(*m),
            }
        }
    }
    struct Monkey {
        inventory: Vec<BigInt>,
        operation: Operation,
        test_divisible_by: BigInt,
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
            test_divisible_by: i32,
            true_target: i32,
            false_target: i32,
        ) -> Self {
            let inventory = inventory.into_iter().map(BigInt::from).collect();
            Self {
                inventory,
                operation,
                test_divisible_by: BigInt::from(test_divisible_by),
                true_target,
                false_target,
                inspect_count: 0,
            }
        }

        fn process_items(&mut self, worry_attenuation: bool, lcm: i32) -> Vec<MonkeyResult> {
            let mut results = Vec::new();
            self.inspect_count += self.inventory.len();

            for mut item in self.inventory.drain(..) {
                // Inspect, and apply operation
                self.operation.apply_mut(&mut item);
                // Get bored and reduce

                if worry_attenuation {
                    item = item / 3
                } else {
                    item = item % lcm;
                };

                let test_result = item.mod_floor(&self.test_divisible_by);

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

        let lcm = monkeys
            .iter()
            .map(|m| m.test_divisible_by.to_i32().unwrap())
            .reduce(|a, b| a * b)
            .unwrap();

        let max_rounds = if worry_attenuation { 20 } else { 10000 };
        while round_num < max_rounds {
            for i in 0..monkeys.len() {
                let results = monkeys[i].process_items(worry_attenuation, lcm);
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
                let modifier: i32 = parts[1].parse().unwrap();
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
