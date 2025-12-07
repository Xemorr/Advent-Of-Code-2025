use std::fs;

pub fn d5q2() {
    let strings = fs::read_to_string("d5q1.txt")
        .expect("Should have been able to read file");

    let mut ranges: Vec<FreshRange> = strings.lines()
        .filter(|line| line.contains("-"))
        .map(|line| {
            let vec_range = line.split("-").map(|num| num.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            FreshRange::new(vec_range[0], vec_range[1])
        })
        .collect();
    ranges.sort_unstable_by_key(|a| a.start);
    println!("size of ranges initially, {}", ranges.len());
    let mut last_size = ranges.len();
    loop {
        ranges = ranges.iter().fold(Vec::new(), |mut acc: Vec<FreshRange>, b| {
            let new_elements = if let Some(a) = acc.pop() {
                if a.start >= b.start && a.end <= b.end {
                    vec![b.clone()]
                } else if b.start >= a.start && b.end <= a.end {
                    vec![a.clone()]
                } else if a.start <= b.start && a.end >= b.start && a.end <= b.end {
                    vec![FreshRange::new(a.start, b.end)]
                } else if b.start <= a.start && b.end >= a.start && b.end <= a.end {
                    vec![FreshRange::new(b.start, a.end)]
                } else {
                    vec![a.clone(), b.clone()]
                }
            } else {
                vec![b.clone()]
            };
            acc.extend(new_elements);
            acc
        });
        if last_size == ranges.len() {
            break;
        }
        last_size = ranges.len();
    }
    let value_count: u64 = ranges.iter().map(|range| {
        range.end - range.start + 1
    }).sum();
    println!("size of ranges after folding, {}", ranges.len());
    println!("{}", value_count);
}

struct FreshRanges {
    ranges: Vec<FreshRange>
}

impl FreshRanges {
    fn new(ranges: Vec<FreshRange>) -> Self {
        let mut ranges = ranges.clone();

        Self { ranges }
    }

    fn contains(&self, value: u64) -> bool {
        for range in &self.ranges {
            if range.contains(value) { return true; }
            if range.start > value { return false; }
        }
        false
    }
}

#[derive(Clone)]
struct FreshRange {
    start: u64,
    end: u64
}

impl FreshRange {
    fn new(start: u64, end: u64) -> Self {
        let start = start.min(end);
        let end = start.max(end);
        Self { start, end }
    }

    fn contains(&self, value: u64) -> bool {
        value >= self.start && value <= self.end
    }
}