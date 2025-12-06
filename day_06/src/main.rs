use std::{io::{self, Read}, iter::{Product, Sum}, str::FromStr, time::Instant};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let t = Instant::now();
    let silver = silver(&input);
    let time = t.elapsed();
    println!("Silver: {silver}, took: {time:?}");

    let t = Instant::now();
    let gold = gold(&input);
    let time = t.elapsed();
    println!("Gold: {gold}, took: {time:?}");
}

enum Operator {
    Sum,
    Product,
}

impl TryFrom<char> for Operator {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Operator::Sum),
            '*' => Ok(Operator::Product),
            _ => Err("Is not a valid operator")
        }
    }
}

impl FromStr for Operator {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Sum),
            "*" => Ok(Operator::Product),
            _ => Err("Is not a valid operator")
        }
    }
}

impl Operator {
    fn apply_to<T: Sum + Product>(&self, ring_elems: impl Iterator<Item=T>) -> T {
        match self {
            Operator::Sum     => ring_elems.sum(),
            Operator::Product => ring_elems.product(),
        }
    }
}

fn silver(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<_>>();
    let (&last, rest) =  lines.split_last().unwrap();
    let operators = last
        .split_whitespace()
        .flat_map(|s| Operator::from_str(s).ok())
        .collect::<Vec<_>>();

    let mut rows = rest.into_iter()
        .map(|&s| s.split_whitespace()
            .flat_map(|s| s.parse::<i64>().ok()))
        .collect::<Vec<_>>();


    let mut results = Vec::with_capacity(operators.len());

    for op in operators {
        let mut column_elems = Vec::with_capacity(rows.len());
        for row in &mut rows {
            let value = row.next().unwrap();
            column_elems.push(value);
        }
        let result = op.apply_to(column_elems.into_iter());
        results.push(result);
    }

    results.into_iter()
        .sum()
}

fn gold(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<_>>();
    let (&last, rest) = lines.split_last().unwrap();
    let mut last = last;

    let mut rows = rest.into_iter()
        .map(|row| row.chars())
        .collect::<Vec<_>>();

    let mut operators = Vec::new();
    while let Some(((op, ws_count), remaining)) = op_then_whitespace_with_count(last) {
        last = remaining;
        let op = Operator::try_from(op).unwrap();
        operators.push((op, ws_count));
    }
    operators.last_mut().map(|(_, w)| *w = *w+1);

    let mut results: Vec<i64> = Vec::with_capacity(operators.len());

    for (op, ws) in operators {
        let mut nums: Vec<i64> = Vec::with_capacity(ws);
        for _ in 0..=ws {
            let mut num_chars = String::with_capacity(rows.len());
            for row in &mut rows {
                match row.next() {
                    Some(' ') => (),
                    Some(c)   => num_chars.push(c),
                    _         => (),
                }
            }
            if !num_chars.is_empty() {
                let num = num_chars.parse::<i64>().unwrap();
                nums.push(num);
            }
        }
        let result = op.apply_to(nums.into_iter());
        results.push(result);
    }

    results.into_iter()
        .sum()
}

fn op_then_whitespace_with_count(input: &str) -> Option<((char, usize), &str)> {
    if input.is_empty() {
        return None;
    }
    let mut chars = input.chars();

    let op = chars.next()?;
    let input = &input[1..];

    let whitespace = chars.take_while(|c| c.is_whitespace());
    let whitespace_count = whitespace.count();
    let input = &input[whitespace_count..];

    Some(((op, whitespace_count), input))
}
