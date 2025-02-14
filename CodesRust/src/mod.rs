fn parse_disk_map(disk_map: &str) -> Vec<Option<usize>> {
    let mut blocks = Vec::new();
    let mut file_id = 0;
    let nums: Vec<usize> = disk_map.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    for chunk in nums.chunks(2) {
        let file_len = chunk[0];
        for _ in 0..file_len {
            blocks.push(Some(file_id));
        }
        file_id += 1;

        if chunk.len() > 1 {
            let space_len = chunk[1];
            for _ in 0..space_len {
                blocks.push(None);
            }
        }
    }
    blocks
}

fn compact(blocks: &mut Vec<Option<usize>>) {
    let len = blocks.len();
    let mut left = 0;
    let mut right = len - 1;

    while left < right {
        //trouver le none prochain (from left to right)
        while left < right && blocks[left].is_some() {
            left += 1;
        }

        // trouver le some prochain (from right to left)
        while left < right && blocks[right].is_none() {
            right -= 1;
        }

        //swap si on trouve une paire
        if left < right {
            blocks.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

fn _p1(_input: &str) -> i64 {
    let mut blocks = parse_disk_map(_input);
    compact(&mut blocks);
    let mut sum = 0;
    for (pos, block) in blocks.iter().enumerate() {
        if let Some(file_id) = block {
            sum += pos * file_id;
        }
    }
    sum as i64
}

fn compact_p2(blocks: &mut Vec<Option<usize>>) {
    let ranges = get_file_ranges(blocks);
    let mut sorted_ranges: Vec<_> = ranges.iter().copied().collect();
    sorted_ranges.sort_by_key(|&(id, _, _)| std::cmp::Reverse(id));

    for &(id, start, end) in &sorted_ranges {
        let size = end - start;
        if let Some(new_pos) = find_free_space(blocks, size, start) {
            //Clear ancienne position
            for i in start..end {
                blocks[i] = None;
            }
            // Place in new position
            for i in new_pos..(new_pos + size) {
                blocks[i] = Some(id);
            }
        }
    }
}


fn get_file_ranges(blocks: &[Option<usize>]) -> Vec<(usize, usize, usize)> {
    let mut ranges = Vec::new();
    let mut start = None;
    let mut current_id = None;

    for (i, &block) in blocks.iter().enumerate() {
        match (current_id, block) {
            //on commence une range (on a pas de current_id
            (None, Some(id)) => {
                start = Some(i);
                current_id = Some(id);
            }
            //on trouve une nouvelle id (!= current_id)
            (Some(id), Some(new_id)) if id != new_id => {
                ranges.push((id, start.unwrap(), i));
                start = Some(i);
                current_id = Some(new_id);
            }
            //pas de nouvelle id
            (Some(id), None) => {
                ranges.push((id, start.unwrap(), i));
                start = None;
                current_id = None;
            }
            _ => {}
        }
    }

    //si la dernière id s"étend à la fin des blocks
    if let Some(id) = current_id {
        ranges.push((id, start.unwrap(), blocks.len()));
    }

    ranges
}

fn find_free_space(blocks: &[Option<usize>], size: usize, max_pos: usize) -> Option<usize> {
    let mut count = 0;
    let mut start = None;

    for (i, &block) in blocks[..max_pos].iter().enumerate() {
        if block.is_none() {
            if start.is_none() {
                start = Some(i);
            }
            count += 1;
            if count == size {
                return start;
            }
        } else {
            start = None;
            count = 0;
        }
    }
    None
}

fn _p2(input: &str) -> i64 {
    let mut blocks = parse_disk_map(input);
    compact_p2(&mut blocks);

    //iter au lieu de loop
    blocks.iter()
        .enumerate()
        .filter_map(|(pos, &block)| block.map(|file_id| pos * file_id))
        .sum::<usize>() as i64
}

pub fn p1() -> i64 {
    _p1(include_str!("day9"))
}

pub fn p2() -> i64 {
    _p2(include_str!("day9"))
}

#[cfg(test)]
mod tests {
    use crate::day9::*;
    #[test]
    fn test_p1() {
        assert_eq!(9, _p1(include_str!("day9")))
    }

    #[test]
    fn test_p2()
    {
        assert_eq!(6390781891880, _p2(include_str!("day9")))
    }

    /*#[test]
    fn real_p1(){
        assert_eq!(598484546, p1());
    }

    #[test]
    fn real_p2(){
        assert_eq!(64845668, p2());
    }*/
}