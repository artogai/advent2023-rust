use std::collections::HashMap;

use utils::read_lines;

fn main() {
    let lines = read_lines("day8/input.txt")
        .unwrap()
        .into_iter()
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    let mut cmds = parse_cmd(&lines[0]);
    let map = parse_field(&lines[2..]);

    let mut curr = "AAA";
    let end = "ZZZ";

    let mut steps = 0;
    while curr != end {
        let cmd = cmds.next().unwrap();
        let (left, right) = map.get(curr).unwrap();
        match cmd {
            'L' => curr = left,
            'R' => curr = right,
            _ => panic!("Invalid command"),
        }
        steps += 1;
    }

    println!("Steps: {}", steps);
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

struct Commands {
    cmds: Vec<char>,
    index: usize,
}

impl<'a> Iterator for Commands {
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
