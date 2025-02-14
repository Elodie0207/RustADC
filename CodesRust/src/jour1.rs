//first Day 1: 140.0 ms


/*
fn _p1(_input: &str) -> u64 {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in _input.lines() {
        let mut values = line.split_whitespace();
        if let Some(value) = values.next() {
            col1.push(value.parse::<i32>().unwrap());
        }
        if let Some(value) = values.next() {
            col2.push(value.parse::<i32>().unwrap());
        }
    }

    col1.sort();
    col2.sort();

    let mut distances = Vec::new();

    for (a, b) in col1.iter().zip(col2.iter()) {
        distances.push((a - b).abs());
    }



    let total_sum: u64 = distances.iter().map(|&x| x as u64).sum();


    total_sum
}

fn _p2(_input: &str) -> u64 {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in _input.lines() {
        let mut values = line.split_whitespace();
        if let Some(value) = values.next() {
            col1.push(value.parse::<i32>().unwrap());
        }
        if let Some(value) = values.next() {
            col2.push(value.parse::<i32>().unwrap());
        }
    }
    let mut similarity_score: u64 = 0;

    for &number in &col1 {
        let mut count = 0;
        for &x in &col2 {
            if x == number {
                count += 1;
            }
        }
        similarity_score += (number as u64) * count;
    }

    similarity_score
}

pub fn p1() -> u64 {
    _p1(include_str!("../Inputs/d1.txt"))
}

pub fn p2() -> u64 {
    _p2(include_str!("../Inputs/d1.txt"))
}
 */



// environ 104ms
fn _p1(_input: &str) -> u64 {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in _input.lines() {
        let mut values = line.split_whitespace().flat_map(str::parse::<i32>);
        if let Some(a) = values.next() {
            if let Some(b) = values.next() {
                col1.push(a);
                col2.push(b);
            }
        }
    }


    col1.sort_unstable();
    col2.sort_unstable();

    col1.iter().zip(col2.iter()).fold(0u64, |acc, (a, b)| acc + (a - b).unsigned_abs() as u64)
}
fn _p2(_input: &str) -> u64 {
    use std::collections::HashMap;

    let (col1, col2): (Vec<i32>, Vec<i32>) = _input
        .lines()
        .filter_map(|line| {
            let mut values = line.split_whitespace().flat_map(str::parse::<i32>);
            Some((values.next()?, values.next()?))
        })
        .unzip();

    let col2_counts: HashMap<i32, u64> = col2.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    col1.iter().fold(0u64, |acc, &number| acc + number as u64 * col2_counts.get(&number).unwrap_or(&0))
}

pub fn p1() -> u64 {
    _p1(include_str!("../Inputs/d1.txt"))
}

pub fn p2() -> u64 {
    _p2(include_str!("../Inputs/d1.txt"))
}