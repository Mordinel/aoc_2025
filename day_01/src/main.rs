use std::{str, iter, fmt};

fn main() {
    let lines = std::io::stdin().lines()
        .flat_map(Result::ok)
        .collect::<Vec<_>>();

    let silver = silver(&lines);
    println!("Silver: {silver}");

    let gold = gold(&lines);
    println!("Gold: {gold}");
}

fn silver(lines: &[String]) -> usize {
    let dial = Dial::<100>::starting_at(50);

    let mut rotations = lines.iter()
        .filter_map(|line| line.parse().ok());

    let dial_succ = |d: &Dial<_>| match rotations.next() {
        Some(Rotation:: Left(amount)) => return Some(d.clone() .left_by(amount)),
        Some(Rotation::Right(amount)) => return Some(d.clone().right_by(amount)),
        _ => return None,
    };

    let dials = iter::successors(Some(dial), dial_succ);

    dials
        .filter_map(|dial| (dial.at() == 0).then_some(dial))
        .count()
}

fn gold(lines: &[String]) -> usize {
    let dial = Dial::<100>::starting_at(50);

    let mut rotations = lines.iter()
        .filter_map(|line| line.parse().ok())
        .flat_map(|rot: Rotation| iter::successors(Some(rot), |r| r.decrement()));

    let dial_succ = |d: &Dial<_>| {
        let mut d = d.clone();
        let rot = rotations.next();
        match rot {
            Some(Rotation:: Left(_)) => d.left(),
            Some(Rotation::Right(_)) => d.right(),
            _ => return None,
        }
        Some(d)
    };

    let dials = iter::successors(Some(dial), dial_succ);

    dials
        .filter_map(|dial| (dial.at() == 0).then_some(dial))
        .count()
}

enum Rotation {
    Left(usize),
    Right(usize),
}
impl Rotation {
    fn decrement(&self) -> Option<Self> {
        match self {
            Self::Left(amount) | Self::Right(amount) if *amount == 1 => None,
            Self::Right(amount) => Some(Self::Right(amount - 1)),
            Self:: Left(amount) => Some(Self:: Left(amount - 1)),
        }
    }
}
impl str::FromStr for Rotation {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let first = chars.next().ok_or("no chars")?;
        let amount = chars.as_str().parse::<usize>().or(Err("not a `usize`"))?;
        match first {
            'R' => Ok(Rotation::Right(amount)),
            'L' => Ok(Rotation::Left(amount)),
            _ => Err("Invalid format (not R | L)"),
        }
    }
}
impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Rotation:: Left(amount) => format!("L{amount}"),
            Rotation::Right(amount) => format!("R{amount}"),
        })
    }
}

#[derive(Clone)]
struct Dial<const LIMIT: usize> {
    at: usize,
}

impl<const LIMIT: usize> Dial<LIMIT> {
    fn starting_at(at: usize) -> Self {
        Dial { at }
    }
    fn at(&self) -> usize {
        self.at
    }
    fn right(&mut self) {
        self.at = if self.at == (LIMIT-1) {
            0
        } else {
            self.at + 1
        };
    }
    fn right_by(mut self, amount: usize) -> Self {
        self.at += amount;
        self.at %= LIMIT;
        self
    }
    fn left(&mut self) {
        self.at = if self.at == 0 {
            LIMIT - 1
        } else {
            self.at - 1
        };
    }
    fn left_by(mut self, mut amount: usize) -> Self {
        if amount > self.at {
            while amount != 0 {
                self.left();
                amount -= 1;
            }
        } else {
            self.at -= amount;
        }
        self
    }
}
