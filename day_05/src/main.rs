use std::{io::{self, Read}, ops::RangeInclusive, time::Instant};

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

fn silver(input: &str) -> i64 {
    let (range_part, id_part) = input.split_once("\n\n").unwrap();
    let fresh_ranges = range_part.lines()
        .flat_map(|line| line.split_once('-'))
        .flat_map(|(lo, hi)| Some((lo.parse::<i64>().ok()?, hi.parse::<i64>().ok()?)))
        .map(|(lo, hi)| lo..=hi)
        .collect::<Vec<_>>();
    let ids = id_part.lines()
        .flat_map(|line| line.parse::<i64>().ok());
    ids.filter(|id| is_fresh(id, &fresh_ranges))
        .count() as i64
}

fn gold(input: &str) -> i64 {
    let (range_part, _) = input.split_once("\n\n").unwrap();
    let fresh_ranges = range_part.lines()
        .flat_map(|line| line.split_once('-'))
        .flat_map(|(lo, hi)| Some((lo.parse::<i64>().ok()?, hi.parse::<i64>().ok()?)))
        .map(|(lo, hi)| lo..=hi)
        .collect::<Vec<_>>();
    unique_id_count(&fresh_ranges) as i64
}

fn unique_id_count(fresh_ranges: &[RangeInclusive<i64>]) -> usize {
    let mut intervals = fresh_ranges.to_vec();

    if intervals.is_empty() {
        return 0;
    }

    intervals.sort_unstable_by_key(|r| *r.start());

    let mut merged = Vec::with_capacity(intervals.len());
    let mut cur = intervals[0].clone();

    for next in intervals.into_iter().skip(1) {
        if *cur.end() + 1 >= *next.start() {
            let new_end = (*cur.end()).max(*next.end());
            cur = *cur.start()..=new_end;
        } else {
            merged.push(cur);
            cur = next;
        }
    }
    merged.push(cur);

    merged.iter()
        .flat_map(|range| range.clone().into_iter())
        .count()
}

fn is_fresh(id: &i64, fresh_ranges: &[RangeInclusive<i64>]) -> bool {
    for range in fresh_ranges {
        if range.contains(id) {
            return true;
        }
    }
    return false;
}
