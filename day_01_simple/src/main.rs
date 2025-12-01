use std::time::Instant;

fn main() {
    let lines = std::io::stdin()
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    const MODULUS: u64 = 100;
    let mut dial: u64 = 50;
    let mut zero_count = 0;

    let t = Instant::now();

    for line in &lines {
        let mut line_chars = line.chars();
        let direction_char = line_chars.next();
        match direction_char {
            Some('R') => {
                let amount: u64 = line_chars.as_str().parse().unwrap();
                dial += amount;
                dial %= MODULUS;
            },
            Some('L') => {
                let mut amount: u64 = line_chars.as_str().parse().unwrap();
                if amount > dial {
                    while amount != 0 {
                        if dial == 0 {
                            dial = MODULUS - 1;
                        } else {
                            dial -= 1;
                        }
                        amount -= 1;
                    }
                } else {
                    dial -= amount;
                }
            },
            _ => panic!("input not (R | L)"),
        }
        if dial == 0 {
            zero_count += 1;
        }
    }

    let took = t.elapsed();
    println!("Part 1: {zero_count}. Took: {took:?}");

    let t = Instant::now();
    let mut dial: u64 = 50;
    let mut zero_count = 0;

    fn increment_if_zero(x: u64, zero_count: &mut u64) {
        if x == 0 {
            *zero_count += 1;
        }
    }

    fn rotate_right(dial: &mut u64, modulus: u64) {
        if *dial == (modulus - 1) {
            *dial = 0;
        } else {
            *dial += 1;
        }
    }

    fn rotate_left(dial: &mut u64, modulus: u64) {
        if *dial == 0 {
            *dial = modulus - 1;
        } else {
            *dial -= 1;
        }
    }

    for line in &lines {
        let mut line_chars = line.chars();
        let direction_char = line_chars.next();
        match direction_char {
            Some('R') => {
                let mut amount: u64 = line_chars.as_str().parse().unwrap();
                while amount > 0 {
                    rotate_right(&mut dial, MODULUS);
                    amount -= 1;
                    increment_if_zero(dial, &mut zero_count);
                }
            },
            Some('L') => {
                let mut amount: u64 = line_chars.as_str().parse().unwrap();
                while amount > 0 {
                    rotate_left(&mut dial, MODULUS);
                    amount -= 1;
                    increment_if_zero(dial, &mut zero_count);
                }
            },
            _ => panic!("input not (R | L)"),
        }
    }

    let took = t.elapsed();
    println!("Part 2: {zero_count}. Took: {took:?}");
}
