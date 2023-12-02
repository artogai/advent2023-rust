use lazy_static::lazy_static;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

use anyhow::Result;

lazy_static! {
    static ref SPELLED_DIGITS: Vec<&'static str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
}

fn main() {
    let input = "day1/input.txt";
    let sum1 = sum_c_values_file(input, parse_c_value).unwrap();
    let sum2 =
        sum_c_values_file(input, |l| parse_c_value(&replace_spelled_digits(l)))
            .unwrap();

    println!("Sum 1: {}", sum1);
    println!("Sum 2: {}", sum2);
}

fn sum_c_values_file(
    path: &str,
    parse_c_value: impl Fn(&str) -> Result<u32>,
) -> Result<u32> {
    sum_c_values(read_lines(path)?, parse_c_value)
}

fn sum_c_values(
    lines: Lines<BufReader<File>>,
    parse_c_value: impl Fn(&str) -> Result<u32>,
) -> Result<u32> {
    lines.into_iter().try_fold(0, |acc, line| {
        parse_c_value(&line?).map(|c_value| acc + c_value)
    })
}

fn parse_c_value(line: &str) -> Result<u32> {
    let digits = line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
    let c = make_c_value(&digits)?;
    Ok(c)
}

fn replace_spelled_digits(line: &str) -> String {
    let mut acc = String::new();
    for c in line.chars() {
        acc.push(c);
        for (i, digit) in SPELLED_DIGITS.iter().enumerate() {
            if acc.ends_with(digit) {
                acc = acc.replace(
                    digit,
                    &format!("{}{}", i + 1, digit.chars().last().unwrap()),
                );
            }
        }
    }
    acc
}

fn make_c_value(digits: &[char]) -> Result<u32> {
    let (first, tail) = digits.split_first().expect("No digits found");
    let second = match tail.last() {
        Some(last) => last,
        None => first,
    };

    let c = [first, second]
        .into_iter()
        .collect::<String>()
        .parse::<u32>()?;

    Ok(c)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
