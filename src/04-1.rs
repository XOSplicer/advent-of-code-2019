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
        let same_adjacent = self
            .digits
            .iter()
            .zip(self.digits.iter().skip(1))
            .any(|(d1, d2)| d1 == d2);
        let never_decrease = self
            .digits
            .iter()
            .zip(self.digits.iter().skip(1))
            .all(|(d1, d2)| d1 <= d2);
        six_digits && same_adjacent && never_decrease
    }
}

fn main() {
    let answer = (273_025..=767_253)
        .map(|u| Password::from_usize(u))
        .filter(|p| p.meets_criteria())
        .count();
    println!("{}", answer);
}
