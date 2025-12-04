use std::io::{self, Read};
use std::iter;
use std::time::Instant;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let t = Instant::now();
    let grid = parse(&input);
    let silver = count_accessible(&grid);
    let time = t.elapsed();
    println!("Silver: {silver}, took: {time:?}");

    let t = Instant::now();
    let grid = parse(&input);
    let gold = count_removable(&grid);
    let time = t.elapsed();
    println!("Gold: {gold}, took: {time:?}");
}

fn parse(input: &str) -> (Vec<u8>, usize, usize) {
    let mut width = None;
    let mut height = 0;

    let grid = input.lines()
        .flat_map(|line| {
            let row = line.bytes().map(|b| (b == b'@') as u8).collect::<Vec<_>>();
            if width.is_none() {
                width = Some(row.len());
            }
            height += 1;
            row
        })
        .collect::<Vec<_>>();
    (grid, width.unwrap_or(0), height)
}

const OFFSETS: [(isize, isize); 8] = [
    (-1,-1), (-1,0), (-1,1),
    ( 0,-1),         ( 0,1),
    ( 1,-1), ( 1,0), ( 1,1),
];

fn neighbor_count(grid: &[u8], width: usize, r: usize, c: usize) -> u32 {
    OFFSETS.iter()
        .filter_map(|&(dr, dc)| {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nc >= 0 {
                let nr = nr as usize;
                let nc = nc as usize;
                if nr < grid.len() / width && nc < width {
                    Some(grid[nr * width + nc] as u32)
                } else { None }
            } else { None }
        })
    .sum()
}

fn positions(width: usize, height: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..height).flat_map(move |r| (0..width).map(move |c| (r, c)))
}

fn count_accessible((grid, width, height): &(Vec<u8>, usize, usize)) -> usize {
    positions(*width, *height)
        .filter(|&(r, c)| (grid[r * width + c] == 1) && (neighbor_count(grid, *width, r, c) < 4))
        .count()
}

fn count_removable((grid, width, height): &(Vec<u8>, usize, usize)) -> usize {
    let w = *width;
    let h = *height;

    let next_state = |g: Vec<u8>| -> Option<(Vec<u8>, usize)> {
        let to_remove: Vec<usize> = positions(w, h)
            .filter(|&(r, c)| (g[r * w + c] == 1) && (neighbor_count(&g, w, r, c) < 4))
            .map(|(r, c)| r * w + c)
            .collect();

        if to_remove.is_empty() {
            None
        } else {
            let mut next = g;
            for &i in &to_remove { next[i] = 0; }
            Some((next, to_remove.len()))
        }
    };

    iter::successors(Some((grid.clone(), 0)), move |(current, _)| next_state(current.clone()))
        .skip(1)
        .map_while(|(_, removed)| (removed > 0).then_some(removed))
        .sum()
}
