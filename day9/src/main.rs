use utils::read_lines;

fn main() {
    let mut sum = 0;
    for line in read_lines("day9/input.txt").unwrap() {
        sum += extrapolate(parse(&line.unwrap()));
    }
    println!("Sum: {}", sum);
}

fn parse(s: &str) -> Vec<i64> {
    s.split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn extrapolate(nums: Vec<i64>) -> i64 {
    let mut rows = generate_rows(nums);
    rows.pop();
    rows.reverse();

    let mut sum = 0;
    for row in rows {
        sum = row.first().unwrap() - sum
    }
    sum
}

fn generate_rows(nums: Vec<i64>) -> Vec<Vec<i64>> {
    let mut rows = Vec::new();
    rows.push(nums);

    loop {
        let curr_row = rows.last().unwrap();
        if curr_row.iter().all(|c| *c == 0) {
            break;
        }
        let mut next_row = Vec::with_capacity(curr_row.len() - 1);
        for i in 0..curr_row.len() - 1 {
            next_row.push(curr_row[i + 1] - curr_row[i]);
        }
        rows.push(next_row);
    }

    rows
}
