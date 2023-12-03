use std::collections::{HashMap, HashSet};

use utils::read_lines;

type NumberId = usize;
type SymbolId = usize;

fn main() {
    let mut schema = Vec::new();
    let mut numbers = HashMap::new();
    let mut symbols = HashMap::new();
    let mut numbers_conn = HashMap::new();
    let mut symbols_conn = HashMap::new();

    let mut width = 0;

    for line in read_lines("day3/input.txt").unwrap() {
        let mut line = line.unwrap().chars().collect::<Vec<char>>();
        width = line.len();
        schema.append(&mut line);
    }

    let mut curr = 0;
    while curr < schema.len() {
        if schema[curr].is_numeric() {
            let number_id = curr;
            let number_row = curr / width;
            let mut buff = Vec::new();
            while curr < schema.len()
                && curr / width == number_row // check that we are still in the same row
                && schema[curr].is_numeric()
            {
                let i = curr / width;
                let j = curr % width;

                check_conn(
                    number_id,
                    i,
                    j,
                    &mut numbers_conn,
                    &mut symbols_conn,
                    &mut symbols,
                    &schema,
                    width,
                );
                buff.push(schema[curr]);
                curr += 1;
            }
            let number =
                buff.iter().collect::<String>().parse::<usize>().unwrap();
            numbers.insert(number_id, number);
        } else {
            curr += 1
        }
    }

    let mut part_sum = 0;
    for number_id in numbers_conn.keys() {
        part_sum += numbers[number_id];
    }
    println!("Part sum: {}", part_sum);

    let mut gear_sum = 0;
    for symbol_id in symbols.keys() {
        let symbol = symbols[symbol_id];
        if symbol == '*' && symbols_conn[symbol_id].len() == 2 {
            let conn = symbols_conn[symbol_id].iter().collect::<Vec<_>>();
            let number_id_1 = conn[0];
            let number_id_2 = conn[1];
            gear_sum += numbers[number_id_1] * numbers[number_id_2];
        }
    }
    println!("Gear sum: {}", gear_sum);
}

#[allow(clippy::too_many_arguments)]
fn check_conn(
    number_id: NumberId,
    i: usize,
    j: usize,
    numbres_conn: &mut HashMap<NumberId, HashSet<SymbolId>>,
    symbols_conn: &mut HashMap<SymbolId, HashSet<NumberId>>,
    symbols: &mut HashMap<SymbolId, char>,
    schema: &[char],
    width: usize,
) {
    let i = i as isize;
    let j = j as isize;

    [
        (i - 1, j),
        (i + 1, j),
        (i, j - 1),
        (i, j + 1),
        (i - 1, j - 1),
        (i - 1, j + 1),
        (i + 1, j - 1),
        (i + 1, j + 1),
    ]
    .into_iter()
    .for_each(|(i, j)| {
        add_conn_if_exists(
            number_id,
            i,
            j,
            numbres_conn,
            symbols_conn,
            symbols,
            schema,
            width,
        )
    });
}

#[allow(clippy::too_many_arguments)]
fn add_conn_if_exists(
    number_id: NumberId,
    i: isize,
    j: isize,
    numbres_conn: &mut HashMap<NumberId, HashSet<SymbolId>>,
    symbols_conn: &mut HashMap<SymbolId, HashSet<NumberId>>,
    symbols: &mut HashMap<SymbolId, char>,
    schema: &[char],
    width: usize,
) {
    if i < 0 || j < 0 {
        return;
    }

    let i = i as usize;
    let j = j as usize;

    if i >= schema.len() / width || j >= width {
        return;
    }

    let symbol_id = i * width + j;
    let symbol = schema[symbol_id];

    if !symbol.is_numeric() && symbol != '.' {
        symbols.insert(symbol_id, symbol);
        numbres_conn
            .entry(number_id)
            .or_default()
            .insert(symbol_id);
        symbols_conn
            .entry(symbol_id)
            .or_default()
            .insert(number_id);
    }
}
