use std::{io, time};

fn main() {
    let lines = io::stdin().lines()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let t = time::Instant::now();
    let silver = largest_n_stable(&lines, 2);
    let time = t.elapsed();
    println!("Silver: {silver}, took: {time:?}");

    let t = time::Instant::now();
    let gold = largest_n_stable(&lines, 12);
    let time = t.elapsed();
    println!("Gold: {gold}, took: {time:?}");
}

fn largest_n_stable(lines: &[String], n: usize) -> i64 {
    lines.iter()
        .map(|line| {
            let bytes = line.as_bytes();
            let bytes_len = bytes.len();
            let mut largest = String::with_capacity(n);
            let mut start = 0;

            for n in (0..n).rev() {
                let mut best_idx = start;

                for i in start..(bytes_len-n) {
                    if bytes[i] > bytes[best_idx] {
                        best_idx = i;
                    }
                }
                largest.push(bytes[best_idx] as char);
                start = best_idx + 1;
            }

            largest.parse::<i64>().unwrap()
        })
        .sum()
}

