// https://codeforces.com/problemset/problem/71/A
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n = n.trim().parse::<i32>().unwrap();

    let mut result = String::from("");

    for _ in 0..n {
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");
        let word: Vec<&str> = word.trim().split("").collect();

        if word.len() < 13 {
            result += &format!("{}\n", word.join(""));
        } else {
            result += &format!("{}{}{}\n", word[1], word.len() - 4, word[word.len() - 2]);
        }
    }

    println!("{}", result);

}
