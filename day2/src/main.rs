use utils::read_lines;

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.sets.iter().all(|set| set.is_valid())
    }
}

#[derive(Debug)]
struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Set {
    fn is_valid(&self) -> bool {
        self.red <= BAG.red && self.green <= BAG.green && self.blue <= BAG.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

const BAG: Set = Set {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    let games = read_lines("day2/input.txt")
        .unwrap()
        .map(|line| parse_game(&line.unwrap()))
        .collect::<Vec<_>>();

    let sum_1 = valid_games_ids_sum(&games);
    let sum_2 = possible_bags_power(&games);

    println!("Sum 1: {}", sum_1);
    println!("Sum 2: {}", sum_2);
}

fn valid_games_ids_sum(games: &[Game]) -> u32 {
    let mut sum = 0;
    for game in games {
        if game.is_valid() {
            sum += game.id;
        }
    }
    sum
}

fn possible_bags_power(games: &[Game]) -> u32 {
    let mut sum = 0;
    for game in games {
        let min_bag = calc_min_bag(game);
        sum += min_bag.power();
    }
    sum
}

fn calc_min_bag(game: &Game) -> Set {
    let mut min_bag = Set {
        red: 0,
        green: 0,
        blue: 0,
    };

    for set in &game.sets {
        if set.red > min_bag.red {
            min_bag.red = set.red;
        }
        if set.green > min_bag.green {
            min_bag.green = set.green;
        }
        if set.blue > min_bag.blue {
            min_bag.blue = set.blue;
        }
    }

    min_bag
}

fn parse_game(line: &str) -> Game {
    let mut parts = line.split(':');
    let id = parts
        .next()
        .unwrap()
        .strip_prefix("Game ")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let sets = parts
        .next()
        .unwrap()
        .split(';')
        .map(|s| s.trim())
        .map(parse_set)
        .collect::<Vec<_>>();

    Game { id, sets }
}

fn parse_set(s: &str) -> Set {
    let mut set = Set {
        red: 0,
        green: 0,
        blue: 0,
    };
    s.split(',')
        .map(|c| c.trim())
        .map(|c| c.split(' ').collect::<Vec<_>>())
        .map(|c| (c[0].parse::<u32>().unwrap(), c[1]))
        .for_each(|(n, color)| match color {
            "red" => set.red = n,
            "green" => set.green = n,
            "blue" => set.blue = n,
            _ => panic!("Unknown color: {}", color),
        });

    set
}
