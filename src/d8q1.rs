use std::fs;

pub fn d8q1() {
    let strings = fs::read_to_string("d8q1.txt")
        .expect("Should have been able to read file");

    let boxes: Vec<Vec<JunctionBox>> = strings.lines().map(|line| {
        let mut split = line.split(',');
        let x = split.next().unwrap().parse::<u64>().unwrap();
        let y = split.next().unwrap().parse::<u64>().unwrap();
        let z = split.next().unwrap().parse::<u64>().unwrap();
        vec![JunctionBox::new(x, y, z)]
    }).collect();

    for i in 0..1000 {
        
    }
}

struct JunctionBox {
    x: u64,
    y: u64,
    z: u64
}

impl JunctionBox {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }

    fn distance_squared(&self, other: &JunctionBox) -> u64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}