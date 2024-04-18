// https://codeforces.com/problemset/problem/520/A
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    //let n: i32 = n.trim().parse().expect("Failed to parse");

    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    let s: Vec<&str> = s.trim().split("").collect();

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut e = 0;
    let mut f = 0;
    let mut g = 0;
    let mut h = 0;
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut l = 0;
    let mut m = 0;
    let mut n = 0;
    let mut o = 0;
    let mut p = 0;
    let mut q = 0;
    let mut r = 0;
    let mut s = 0;
    let mut t = 0;
    let mut u = 0;
    let mut v = 0;
    let mut w = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0; 
    
    for i in n..s.len() {
        if s[i as usize] == "a" || i == "A" && s[i as usize] == 0 {
            a += 1;
        } else if s[i as usize] == "b" || i == "B" && b[i as usize] == 0 {
            b += 1;
        } else if i == "c" || i == "C" && c == 0 {
            c += 1;
        } else if i == "d" || i == "D" && d == 0 {
            d += 1;
        } else if i == "e" || i == "E" && e == 0 {
            e += 1;
        } else if i == "f" || i == "F" && f == 0 {
            f += 1;
        } else if i == "g" || i == "G" && g == 0 {
            g += 1;
        } else if i == "h" || i == "H" && h == 0 {
            h += 1;
        } else if i == "i" || i == "I" && i == 0 {
            i += 1;
        } else if i == "j" || i == "J" && j == 0 {
            j += 1;
        } else if i == "k" || i == "K" && k == 0 {
            k += 1;
        } else if i == "l" || i == "L" && l == 0 {
            l += 1;
        } else if i == "m" || i == "M" && m == 0 {
            m += 1;
        } else if i == "n" || i == "N" && n == 0 {
            n += 1;
        } else if i == "o" || i == "O" && o == 0 {
            o += 1;
        } else if i == "p" || i == "P" && p == 0 {
            p += 1;
        } else if i == "q" || i == "Q" && q == 0 {
            q += 1;
        } else if i == "r" || i == "R" && r == 0 {
            r += 1;
        } else if i == "s" || i == "S" && s == 0 {
            r += 1;
        } else if i == "t" || i == "T" && t == 0 {
            t += 1;
        } else if i == "u" || i == "U" && u == 0 {
            u += 1;
        } else if i == "v" || i == "V" && v == 0 {
            v += 1;
        } else if i == "w" || i == "W" && w == 0 {
            w += 1;
        } else if i == "x" || i == "X" && x == 0 {
            x += 1;
        } else if i == "y" || i == "Y" && y == 0 {
            y += 1;
        } else if i == "z" || i == "Z" && z == 0 {
            z += 1;
        }
    }

    let sum = a + b + c + d + e + f + g + h + i + j + k + l + m + n + o + p + q + r + s + t + u + v + w + x + y + z;

    if sum == 26 {
        println!("YES");
    } else {
        println!("NO");
    }
}
