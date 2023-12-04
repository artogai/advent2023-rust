use utils::read_lines;

#[derive(Debug)]
struct Card {
    pub _id: u8,
    pub winning_numbers: Vec<u8>,
    pub numbers: Vec<u8>,
}

impl Card {
    fn wins(&self) -> Vec<u8> {
        let mut wins = Vec::new();
        for num in &self.numbers {
            if self.winning_numbers.contains(num) {
                wins.push(*num);
            }
        }
        wins
    }
}

fn main() {
    let mut cards = Vec::new();
    for line in read_lines("day4/input.txt").unwrap() {
        let card = parse_card(&line.unwrap()).unwrap();
        cards.push(card);
    }

    let mut total_score = 0;
    for card in &cards {
        let score = calc_card_score(card);
        total_score += score;
    }
    println!("Total score: {}", total_score);

    let total_copies = calc_total_copies(&cards);
    println!("Total copies: {}", total_copies);
}

fn calc_total_copies(cards: &[Card]) -> usize {
    fn calc_copies(cards: &[Card], total: &mut usize) {
        if let Some((head, tail)) = cards.split_first() {
            *total += 1;
            let copies = head.wins().len();
            for i in 0..copies {
                calc_copies(&tail[i..], total);
            }
        }
    }

    let mut total = 0;
    for i in 0..cards.len() {
        calc_copies(&cards[i..], &mut total);
    }
    total
}

fn calc_card_score(card: &Card) -> u32 {
    card.wins().split_first().map_or(0, |(_, tail)| {
        let mut score = 1;
        for _ in tail {
            score *= 2;
        }
        score
    })
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
            _id: id,
            winning_numbers: win_nums,
            numbers: nums,
        })
    })
}

fn parse_numbers(s: &str) -> Option<Vec<u8>> {
    s.split_whitespace()
        .map(|num_str| num_str.parse::<u8>().ok())
        .collect()
}
