// https://codeforces.com/problemset/problem/231/A
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: i16 = n.trim().parse::<i16>().unwrap();

    let mut result: i16 = 0;

    for _ in 0..n {
        let mut team = String::new();
        io::stdin()
            .read_line(&mut team)
            .expect("Failed to read line");
        let team: Vec<i8> = team.trim().split(" ").map(|x| x.parse::<i8>().unwrap()).collect();

        let sum: i8 = team.iter().sum();
        
        if sum > 1 {
            result += 1;
        }
    }

    println!("{}", result);
}
