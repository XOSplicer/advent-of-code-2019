use rayon::prelude::*;
use std::fs;
use std::iter;

fn pattern(element_index: usize) -> impl Iterator<Item = i32> {
    let n = element_index + 1;
    iter::empty()
        .chain(iter::repeat(0).take(n))
        .chain(iter::repeat(1).take(n))
        .chain(iter::repeat(0).take(n))
        .chain(iter::repeat(-1).take(n))
        .cycle()
        .skip(1)
}

fn to_last_digit(n: i32) -> u32 {
    format!("{}", n)
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap()
}

fn main() {
    let raw_input: Vec<u32> = fs::read_to_string("input/16")
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<_>>()
        .unwrap();
    let input_repeat = 10_000;
    let mut input: Vec<u32> = iter::repeat(raw_input.iter())
        .take(input_repeat)
        .flatten()
        .cloned()
        .collect();
    println!("Input");
    print_8(&input);
    println!("Input length {}", input.len());
    let message_offset_len = 7;
    let message_offset: usize = input
        .iter()
        .take(message_offset_len)
        .map(|i| format!("{}", i))
        .fold(String::new(), |mut acc, x| {
            acc += &x;
            acc
        })
        .parse()
        .unwrap();
    println!("Message Offset {}", &message_offset);
    let message_len = 8;
    // let truncate_to = message_offset + message_len + 2;
    // TODO: the input can likeley be truncated, as the pattern has lots of leading zeros at some point, so that later digits are only influenced by the digits behind a certain point relative to themself
    // this is actually not correct, they are not influenced by the digits leading up to them, not including themself
    // futhermore the input and the pattern are cyclic with period 10_000 and 4n (offset -1), which might be useful
    let pretruncate = message_offset;
    input.drain(0..pretruncate);
    println!("Truncated Input");
    print_8(&input);
    println!("Truncated Input length {}", input.len());
    let message_offset = 0;
    let num_phases = 100;
    let final_list: Vec<u32> = (0..num_phases).fold(input, |list, phase| {
        println!("Phase {} Start", &phase);
        let new_list: Vec<u32> = list
            .par_iter()
            .enumerate()
            .map(|(element_index, _element)| element_index)
            .map(|element_index| {
                // println!("Phase {} Index {}", &phase, element_index);
                let summed = list
                    .iter()
                    .zip(pattern(element_index).skip(pretruncate))
                    .map(|(element, coefficient)| *element as i32 * coefficient)
                    .sum();
                to_last_digit(summed)
            })
            .collect();
        new_list
    });
    println!("Input (truncated)");
    print_8(&final_list);
    let message: usize = final_list
        .iter()
        .skip(message_offset)
        .take(message_len)
        .map(|i| format!("{}", i))
        .fold(String::new(), |mut acc, x| {
            acc += &x;
            acc
        })
        .parse()
        .unwrap();
    println!("Message {}", &message);
}

fn print_8(list: &[u32]) {
    for i in list.iter().take(8) {
        print!("{}", i);
    }
    println!();
}

#[cfg(test)]
mod test {
    #[test]
    fn to_last_digit() {
        assert_eq!(super::to_last_digit(1), 1);
        assert_eq!(super::to_last_digit(-9), 9);
        assert_eq!(super::to_last_digit(38), 8);
        assert_eq!(super::to_last_digit(-17), 7);
    }
    #[test]
    fn pattern() {
        assert_eq!(
            super::pattern(0).take(8).collect::<Vec<_>>(),
            &[1, 0, -1, 0, 1, 0, -1, 0]
        );
    }
}
