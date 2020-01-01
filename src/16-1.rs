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
    let input: Vec<u32> = fs::read_to_string("input/16")
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<_>>()
        .unwrap();
    print_8(&input);
    let num_phases = 100;
    let folded = (0..num_phases).fold(input, |list, _phase| {
        let new_list: Vec<u32> = (0..list.len())
            .map(|element_index| {
                let summed = list
                    .iter()
                    .zip(pattern(element_index))
                    .map(|(element, coefficient)| *element as i32 * coefficient)
                    .sum();
                // println!("{}", summed);
                to_last_digit(summed)
            })
            .collect();
        // print_8(&new_list);
        new_list
    });
    print_8(&folded)
}

fn print_8(list: &[u32]) {
    for i in list.iter().take(8) {
        print!("{}", i);
    }
    println!("");
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
