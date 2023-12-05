use rayon::prelude::*;
use std::{collections::HashMap, fs};

type Category = String;

#[derive(Debug)]
struct Almanac {
    pub seeds: Vec<Seed>,
    pub categories: HashMap<Category, CategoryMapping>,
}

#[derive(Debug)]
struct Seed {
    pub from: u64,
    pub range_length: u64,
}

#[derive(Debug)]
struct CategoryMapping {
    pub to: Category,
    pub ranges: Vec<RangeMapping>,
}

#[derive(Debug)]
struct RangeMapping {
    pub destination_range_start: u64,
    pub source_range_start: u64,
    pub range_length: u64,
}

fn main() {
    let s = fs::read_to_string("day5/input.txt").unwrap();
    let almanac = parse_almanac(s.strip_suffix('\n').unwrap(), false);

    let min_location = almanac
        .seeds
        .into_par_iter()
        .map(|seed| {
            (seed.from..seed.from + seed.range_length)
                .into_par_iter()
                .map(|i| get_location(i, &almanac.categories))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    println!("Min location: {}", min_location);
}

fn get_location(id: u64, mappings: &HashMap<Category, CategoryMapping>) -> u64 {
    let mut id = id;
    let mut curr = mappings.get("seed");

    while let Some(m) = curr {
        id = match_range(id, &m.ranges);
        curr = mappings.get(&m.to);
    }

    id
}

fn match_range(id: u64, ranges: &[RangeMapping]) -> u64 {
    for range in ranges {
        if range.source_range_start <= id
            && id < range.source_range_start + range.range_length
        {
            return range.destination_range_start
                + (id - range.source_range_start);
        }
    }
    id
}

fn parse_almanac(s: &str, is_single: bool) -> Almanac {
    let blocks = s.split("\n\n").collect::<Vec<_>>();
    let seeds = parse_seeds(blocks.first().unwrap(), is_single);
    let categories = blocks[1..]
        .iter()
        .map(|block| parse_category(block))
        .collect::<Vec<_>>();

    Almanac {
        seeds,
        categories: categories.into_iter().collect::<HashMap<_, _>>(),
    }
}

fn parse_seeds(s: &str, is_single: bool) -> Vec<Seed> {
    let nums = s
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|id| id.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    if is_single {
        nums.into_iter()
            .map(|id| Seed {
                from: id,
                range_length: 1,
            })
            .collect::<Vec<_>>()
    } else {
        let mut res = Vec::new();
        for i in (0..nums.len()).step_by(2) {
            res.push(Seed {
                from: nums[i],
                range_length: nums[i + 1],
            });
        }
        res
    }
}

fn parse_category(s: &str) -> (Category, CategoryMapping) {
    let lines = s.split('\n').collect::<Vec<_>>();
    let (from, to) = parse_header(lines.first().unwrap());
    let ranges = lines[1..]
        .iter()
        .map(|line| parse_range(line))
        .collect::<Vec<_>>();

    (
        from.to_owned(),
        CategoryMapping {
            to: to.to_owned(),
            ranges,
        },
    )
}

fn parse_header(s: &str) -> (&str, &str) {
    s.strip_suffix(" map:").unwrap().split_once("-to-").unwrap()
}

fn parse_range(s: &str) -> RangeMapping {
    let nums = s
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    RangeMapping {
        destination_range_start: nums[0],
        source_range_start: nums[1],
        range_length: nums[2],
    }
}
