use std::fs;

pub fn d5q1() {
    let strings = fs::read_to_string("d5q1.txt")
        .expect("Should have been able to read file");

    let ranges: Vec<FreshRange> = strings.lines()
        .filter(|line| line.contains("-"))
        .map(|line| {
            let vec_range = line.split("-").map(|num| num.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            FreshRange::new(vec_range[0], vec_range[1])
        })
        .collect();
    let ranges = FreshRanges::new(ranges);

    println!("{}", strings.lines()
        .filter(|line| !line.contains("-"))
        .filter(|line| line.is_empty() == false)
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|value| ranges.contains(*value))
        .count());
}

struct FreshRanges {
    ranges: Vec<FreshRange>
}

impl FreshRanges {
    fn new(ranges: Vec<FreshRange>) -> Self {
        let mut ranges = ranges.clone();
        ranges.sort_unstable_by_key(|a| a.start);
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