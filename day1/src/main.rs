use core::ops::Add;

fn depths_increases<T: PartialOrd>(depths: &[T]) -> usize {
    assert!(depths.len() > 0);
    let mut count = 0;
    let mut last = &depths[0];
    for depth in depths.iter() {
        if depth > last {
            count += 1;
        }
        last = depth;
    }

    count
}

fn depths_increases_rolling<T: PartialOrd + Add<Output = T> + Copy>(depths: &[T]) -> usize {
    let mut count = 0;
    let mut last = depths[0] + depths[1] + depths[2];
    for depth in depths.windows(3) {
        let depth = depth[0] + depth[1] + depth[2];
        if depth > last {
            count += 1;
        }
        last = depth;
    }

    count
}

fn load_input(filename: &str) -> Vec<u32> {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect()
}

fn main() {
    let d = load_input("day1/input");
    let n = depths_increases(&d);
    println!("depth increases {}", n);

    let d = load_input("day1/input");
    let n = depths_increases_rolling(&d);
    println!("depth increases {}", n);
}
