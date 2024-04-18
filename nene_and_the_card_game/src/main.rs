// https://codeforces.com/problemset/problem/1956/B
use std::io;

fn main() {
    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("Failed to read line");
    let tests: u32 = tests.trim().parse().expect("Please type a number");

    let mut result = String::from("");

    for _ in 0..tests {
        let mut num_cards = String::new();
        io::stdin()
            .read_line(&mut num_cards)
            .expect("Failed to read line");
        
        let mut points: u32 = 0;

        let mut cards = String::new();
        io::stdin()
            .read_line(&mut cards)
            .expect("Failed to read line");

        let cards: Vec<&str> = cards
            .trim()
            .split_whitespace()
            .collect();

        if cards.len() > 1 {
            let mut i = 0;
            let mut j = 1;
            while i < cards.len() - 1 {
                while j < cards.len() {
                    if cards[i] == cards[j] {
                        points += 1;
                    }
                    j += 1;
                }
                i += 1;
                j = i + 1;
            }
        }

        result += &points.to_string();
        result += "\n";
    }

    println!("{}", result);
}
