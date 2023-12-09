use std::collections::HashMap;

use utils::read_lines;

fn main() {
    let lines = read_lines("day8/input.txt")
        .unwrap()
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    let cmds = parse_cmd(&lines[0]);
    let map = parse_field(&lines[2..]);

    // Part 1
    let mut cmds1 = cmds.clone();
    let mut curr = "AAA";
    let end = "ZZZ";

    let mut steps = 0;
    while curr != end {
        let cmd = cmds1.next().unwrap();
        let (left, right) = map.get(curr).unwrap();
        match cmd {
            'L' => curr = left,
            'R' => curr = right,
            _ => panic!("Invalid command"),
        }
        steps += 1;
    }

    println!("Steps 1: {}", steps);

    // Part 2
    let mut curr = map.keys().filter(|k| k.ends_with('A')).collect::<Vec<_>>();
    curr.sort();

    let oscilations = curr
        .into_iter()
        .map(|c| find_oscillation(c, cmds.clone(), &map))
        .collect::<Vec<_>>();

    println!("Steps 2: {}", lcm(&oscilations));
}

fn find_oscillation(
    node: &str,
    cmds: Commands,
    field: &HashMap<&str, (&str, &str)>,
) -> u64 {
    let mut node = node;
    for (steps, cmd) in cmds.enumerate() {
        let (left, right) = field.get(node).unwrap();
        if node.ends_with('Z') {
            return steps as u64;
        }
        match cmd {
            'L' => node = left,
            'R' => node = right,
            _ => panic!("Invalid command"),
        }
    }
    panic!("No oscillation found");
}

fn gcd_2(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd_2(b, a % b)
    }
}

fn lcm_2(a: u64, b: u64) -> u64 {
    a / gcd_2(a, b) * b
}

fn lcm(numbers: &[u64]) -> u64 {
    numbers.iter().copied().reduce(lcm_2).unwrap()
}

fn parse_field(lines: &[String]) -> HashMap<&str, (&str, &str)> {
    let mut field = HashMap::new();
    for line in lines {
        let (from, (left, right)) = parse_field_row(line);
        field.insert(from, (left, right));
    }
    field
}

fn parse_field_row(s: &str) -> (&str, (&str, &str)) {
    let (from, lr) = s.split_once(" = ").unwrap();
    let (left, right) = lr.split_once(", ").unwrap();
    (from, (&left[1..], &right[..right.len() - 1]))
}

fn parse_cmd(s: &str) -> Commands {
    Commands {
        cmds: s.chars().collect::<Vec<_>>(),
        index: 0,
    }
}

#[derive(Clone)]
struct Commands {
    cmds: Vec<char>,
    index: usize,
}

impl Iterator for Commands {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.cmds.len() {
            let res = self.cmds[self.index];
            self.index += 1;
            Some(res)
        } else {
            let res = self.cmds[0];
            self.index = 1;
            Some(res)
        }
    }
}
