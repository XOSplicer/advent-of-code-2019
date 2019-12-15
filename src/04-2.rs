#[derive(Debug)]
struct Password {
    digits: Vec<u8>,
}

impl Password {
    fn from_usize(u: usize) -> Self {
        Self {
            digits: u
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        }
    }
    fn meets_criteria(&self) -> bool {
        let six_digits = self.digits.len() == 6;
        let never_decrease = self
            .digits
            .iter()
            .zip(self.digits.iter().skip(1))
            .all(|(d1, d2)| d1 <= d2);
        #[allow(clippy::naive_bytecount)]
        let group_of_two = (0..=9u8)
            .map(|d| self.digits.iter().filter(|&c| d == *c).count())
            .filter(|&g| g == 2)
            .count()
            > 0;

        six_digits && never_decrease && group_of_two
    }
}

fn main() {
    let answer = (273_025..=767_253)
        .map(Password::from_usize)
        .filter(|p| p.meets_criteria())
        .count();
    println!("{}", answer);
}
