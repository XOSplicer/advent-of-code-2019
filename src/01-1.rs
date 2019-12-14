use std::fs;
use anyhow::Result as AnyResult;

fn main() -> AnyResult<()> {
    let answer: i64 = fs::read_to_string("input/01")?
        .lines()
        .map(str::parse::<i64>)
        .filter_map(Result::ok)
        .map(|i| (i / 3) - 2)
        .sum();
    println!("{}", answer);
    Ok(())
}