use anyhow::Result as AnyResult;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::fs;
use std::num::ParseIntError;
use std::ops::Add;
use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    #[error("encountered unknown direction `{0}`")]
    UnknownDirection(char),
    #[error("encountered bad movement syntax for `{0}`")]
    BadMovementSyntax(String),
    #[error("encountered invalid movement length: {0}")]
    InvalidMovementLength(#[from] ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = MyError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(MyError::UnknownDirection(c)),
        }
    }
}

#[derive(Debug, Clone)]
struct Movement {
    direction: Direction,
    length: u32,
}

impl TryFrom<&str> for Movement {
    type Error = MyError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let direction = Direction::try_from(
            s.chars()
                .next()
                .ok_or_else(|| MyError::BadMovementSyntax(s.to_owned()))?,
        )?;
        let n = s
            .char_indices()
            .nth(1)
            .ok_or_else(|| MyError::BadMovementSyntax(s.to_owned()))?
            .0;
        let length = s
            .get(n..)
            .ok_or_else(|| MyError::BadMovementSyntax(s.to_owned()))?
            .parse()?;
        Ok(Self { direction, length })
    }
}

impl Movement {
    fn to_vec2d(&self) -> Vec2D {
        let x = if self.direction == Direction::Left {
            -(self.length as i64)
        } else if self.direction == Direction::Right {
            self.length as i64
        } else {
            0
        };
        let y = if self.direction == Direction::Up {
            self.length as i64
        } else if self.direction == Direction::Down {
            -(self.length as i64)
        } else {
            0
        };
        Vec2D { x, y }
    }
    fn is_zero(&self) -> bool {
        self.length == 0
    }
    fn decrease(&mut self) {
        self.length = self.length.saturating_sub(1);
    }
    fn to_unit(&self) -> Self {
        Self {
            direction: self.direction.clone(),
            length: 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Wire {
    movements: Vec<Movement>,
}

impl Wire {
    fn into_iter(self) -> WireIter {
        WireIter {
            coord: Vec2D::new(0, 0),
            movements: self.movements.into(),
        }
    }
    fn first_visit(self, v: &Vec2D) -> Option<usize> {
        self.into_iter()
            .enumerate()
            .find(|&(_, ref c)| c == v)
            .map(|(i, _)| i)
    }
}

#[derive(Debug)]
struct WireIter {
    coord: Vec2D,
    movements: VecDeque<Movement>,
}

impl Iterator for WireIter {
    type Item = Vec2D;
    fn next(&mut self) -> Option<Self::Item> {
        if self.movements.is_empty() {
            return None;
        }
        while !self.movements.is_empty() && self.movements[0].is_zero() {
            self.movements.pop_front();
        }
        if self.movements.is_empty() {
            return None;
        }
        let coord = self.coord.clone();
        self.coord = self.coord.clone() + self.movements[0].to_unit().to_vec2d();
        self.movements[0].decrease();
        Some(coord)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vec2D {
    x: i64,
    y: i64,
}

impl Vec2D {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add<Self> for Vec2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() -> AnyResult<()> {
    let wires: Vec<Wire> = fs::read_to_string("input/03")?
        .lines()
        .map(|line| {
            Ok(Wire {
                movements: line
                    .split(',')
                    .map(Movement::try_from)
                    .collect::<Result<_, MyError>>()?,
            })
        })
        .collect::<Result<_, MyError>>()?;
    assert_eq!(wires.len(), 2);
    println!("Loaded wires");
    let wire_coords: Vec<HashSet<Vec2D>> = wires
        .clone()
        .into_iter()
        .map(|wire| wire.into_iter().collect())
        .collect();
    println!("Transformed coords");
    println!(
        "Wire lengths: {} {}",
        wire_coords[0].len(),
        wire_coords[1].len()
    );
    let intersections = wire_coords[0]
        .iter()
        .filter(|&c| wire_coords[1].contains(c))
        .filter(|&c| c != &Vec2D::new(0, 0));
    println!("Found intersections");
    let answer = intersections
        .filter_map(|c| {
            wires[0]
                .clone()
                .first_visit(c)
                .and_then(|s0| wires[1].clone().first_visit(c).map(|s1| s0 + s1))
        })
        .min();
    println!("{:?}", answer);
    Ok(())
}
