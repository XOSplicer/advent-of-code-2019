use anyhow::Result as AnyResult;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() -> AnyResult<()> {
    let layer_size = 25 * 6;
    let answer = fs::read_to_string("input/08")?
        .trim()
        .chars()
        .chunks(layer_size)
        .into_iter()
        .map(|layer| {
            layer.fold(HashMap::new(), |mut acc, x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            })
        })
        .min_by_key(|layer_count| layer_count[&'0'])
        .map(|layer_count| layer_count[&'1'] * layer_count[&'2']);
    println!("{:?}", answer);

    Ok(())
}
