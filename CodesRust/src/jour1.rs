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

    //println!("Distancias absolutas: {:?}", distances);

    let total_sum: u64 = distances.iter().map(|&x| x as u64).sum();
    //println!("Suma total de distancias: {}", total_sum);

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

    //println!("Similarity score: {}", similarity_score);
    similarity_score
}

pub fn p1() -> u64 {
    _p1(include_str!("../Inputs/d1.txt"))
}

pub fn p2() -> u64 {
    _p2(include_str!("../Inputs/d1.txt"))
}

#[cfg(test)]
mod tests {
    use crate::jour1::*;

    #[test]
    fn test_p1() {
        assert_eq!(11, _p1(include_str!("../Inputs/d1_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(31, _p2(include_str!("../Inputs/d1_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(1258579, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(23981443, p2());
    }
}