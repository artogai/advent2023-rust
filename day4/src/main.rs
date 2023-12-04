use utils::read_lines;

#[derive(Debug)]
struct Card {
    pub id: u8,
    pub winning_numbers: Vec<u8>,
    pub numbers: Vec<u8>,
}

fn main() {
    let mut total_score = 0;
    for line in read_lines("day4/input.txt").unwrap() {
        let card = parse_card(&line.unwrap()).unwrap();
        let score = calc_card_score(&card);
        total_score += score;
    }

    println!("Total score: {}", total_score);
}

fn calc_card_score(card: &Card) -> u32 {
    let mut score = 0;
    let mut first = true;
    for num in &card.numbers {
        if card.winning_numbers.contains(num) {
            if first {
                score += 1;
                first = false;
            } else {
                score *= 2;
            }
        }
    }
    score
}

fn parse_card(s: &str) -> Option<Card> {
    s.split_once(':').and_then(|(id_part, nums_part)| {
        let id = id_part
            .split_once(' ')
            .and_then(|(_, id)| id.trim().parse::<u8>().ok());
        let nums = nums_part.split_once('|').and_then(|(win_nums, nums)| {
            parse_numbers(win_nums).zip(parse_numbers(nums))
        });

        id.zip(nums).map(|(id, (win_nums, nums))| Card {
            id,
            winning_numbers: win_nums,
            numbers: nums,
        })
    })
}

fn parse_numbers(s: &str) -> Option<Vec<u8>> {
    s
        .split_whitespace()
        .map(|num_str| num_str.parse::<u8>().ok())
        .collect()
}
