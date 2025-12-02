fn main() {
    let input = std::io::stdin().lines()
        .take(1)
        .filter_map(Result::ok)
        .collect::<String>();

    let t = std::time::Instant::now();
    let silver: u64 = parse(&input)
        .filter(is_invalid_silver)
        .sum();
    let took = t.elapsed();
    println!("Silver: {silver}, took: {took:?}");

    let t = std::time::Instant::now();
    let gold: u64 = parse(&input)
        .filter(is_invalid_gold)
        .sum();
    let took = t.elapsed();
    println!("Gold: {gold}, took: {took:?}");
}

fn parse(input: &str) -> impl Iterator<Item=u64> {
    input.split(',')
        .filter_map(|range| range.split_once('-'))
        .filter_map(|(l, r)| Some((
            l.parse::<u64>().ok()?,
            r.parse::<u64>().ok()?
        )))
        .flat_map(|(l, r)| (l..=r).into_iter())
}

fn is_invalid_silver(id: &u64) -> bool {
    let id_as_str = id.to_string();
    let id_char_count = id_as_str.chars().count();
    if id_char_count == 0 || !id_char_count.is_multiple_of(2) {
        return false;
    }
    let half_count = id_char_count / 2;
    let (first_half, second_half) = id_as_str.split_at(half_count);
    first_half == second_half
}

fn is_invalid_gold(id: &u64) -> bool {
    let id_as_str = id.to_string();
    let id_chars = id_as_str.chars().collect::<Vec<_>>();
    let id_char_count = id_as_str.chars().count();
    let half_count = id_char_count / 2;

    for n in 1..=half_count {
        if all_chunks_equal(&id_chars, n) {
            return true;
        }
    }
    return false
}

fn all_chunks_equal(chars: &[char], chunk_sz: usize) -> bool {
    let mut chunks = chars.chunks(chunk_sz);
    let first = chunks.next();
    let Some(first) = first else { return true };
    chunks.all(|chunk| chunk == first)
}
