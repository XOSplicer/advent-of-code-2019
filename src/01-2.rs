use std::fs;
use anyhow::Result as AnyResult;

struct FuelIter {
    current_mass: i64,
}

impl FuelIter {
    fn new(module_mass: i64) -> Self {
        Self {
            current_mass: module_mass
        }
    }

}

impl Iterator for FuelIter {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        let fuel = self.current_mass / 3 - 2;
        if fuel <= 0 {
            return None;
        }
        self.current_mass = fuel;
        return Some(fuel);
    }
}

fn main() -> AnyResult<()> {
    let answer: i64 = fs::read_to_string("input/01")?
        .lines()
        .map(str::parse::<i64>)
        .filter_map(Result::ok)
        .map(|i| FuelIter::new(i).fuse().sum::<i64>())
        .sum();
    println!("{}", answer);
    Ok(())
}