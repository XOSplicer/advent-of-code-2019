use anyhow::Result as AnyResult;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    #[error("Bad input`{0}`")]
    BadInput(String),
    #[error("Bad layout")]
    BadLayout,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Bug,
    Empty,
}

impl Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Self::Bug),
            '.' => Some(Self::Empty),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Bug => '#',
            Self::Empty => '.',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    tiles: Vec<Tile>, // row major
    rows: usize,
    columns: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::fmt::Write;
        for row in 0..self.rows {
            for column in 0..self.columns {
                f.write_char(self.tile_at(row, column).unwrap().to_char())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Grid {
    fn from_row_data(
        tiles: impl IntoIterator<Item = Tile>,
        columns: usize,
    ) -> Result<Self, MyError> {
        let tiles: Vec<Tile> = tiles.into_iter().collect();
        if tiles.len() % columns != 0 {
            return Err(MyError::BadLayout);
        }
        let rows = tiles.len() / columns;
        Ok(Self {
            tiles,
            rows,
            columns,
        })
    }

    fn tile_at(&self, row: usize, column: usize) -> Option<&Tile> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        Some(self.tiles.get(row * self.columns + column).unwrap())
    }

    fn tile_at_with_border(&self, row: isize, column: isize) -> Option<&Tile> {
        if row < 0 || column < 0 {
            return None;
        }
        self.tile_at(row as usize, column as usize)
    }

    fn tile_at_mut(&mut self, row: usize, column: usize) -> Option<&mut Tile> {
        if row >= self.rows || column >= self.columns {
            return None;
        }
        Some(self.tiles.get_mut(row * self.columns + column).unwrap())
    }

    fn biodiversity_rating(&self) -> u64 {
        self.tiles
            .iter()
            .zip(ShiftIter::new())
            .filter_map(|(t, v)| match t {
                Tile::Bug => Some(v),
                Tile::Empty => None,
            })
            .sum()
    }

    fn bugs_adjacent_at(&self, row: usize, column: usize) -> usize {
        let offsets = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        std::iter::repeat((row as isize, column as isize))
            .zip(offsets.into_iter())
            .map(|((row, col), (row_offset, col_offset))| (row + row_offset, col + col_offset))
            .filter_map(|(row, col)| self.tile_at_with_border(row, col))
            .filter(|&t| t == &Tile::Bug)
            .count()
    }

    fn evolve(&self) -> Self {
        let mut new: Grid = self.clone();
        for row in 0..self.rows {
            for column in 0..self.columns {
                let bugs = self.bugs_adjacent_at(row, column);
                let tile = self.tile_at(row, column).unwrap();
                *new.tile_at_mut(row, column).unwrap() = match (tile, bugs) {
                    (Tile::Bug, 1) => Tile::Bug,
                    (Tile::Bug, _) => Tile::Empty,
                    (Tile::Empty, 1) => Tile::Bug,
                    (Tile::Empty, 2) => Tile::Bug,
                    (Tile::Empty, _) => Tile::Empty,
                }
            }
        }
        new
    }
}

impl FromStr for Grid {
    type Err = MyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let throw = || MyError::BadInput(s.to_owned());
        let columns = s.lines().next().ok_or_else(throw)?.trim().len();
        let tiles: Vec<Tile> = s
            .chars()
            .filter(|c: &char| !c.is_whitespace() && c != &'\n')
            .map(Tile::from_char)
            .collect::<Option<_>>()
            .ok_or_else(throw)?;
        Self::from_row_data(tiles, columns)
    }
}

#[derive(Debug, Default)]
struct ShiftIter {
    item: u64,
}

impl ShiftIter {
    fn new() -> Self {
        Self { item: 1 }
    }
}

impl Iterator for ShiftIter {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.item;
        self.item <<= 1;
        Some(i)
    }
}

fn main() -> AnyResult<()> {
    let mut grid: Grid = fs::read_to_string("input/24")?.parse()?;
    let mut rounds = 0;
    println!("Input:\n{}", &grid);
    let mut all_previous: HashSet<Grid> = HashSet::new();
    while !all_previous.contains(&grid) {
        all_previous.insert(grid.clone());
        grid = grid.evolve();
        rounds += 1;
        if rounds % 10000 == 0 {
            println!("Rounds: {}", rounds);
        }

        print!("Grid:\n{}", &grid);
        println!("Adjacencies:");
        for row in 0..grid.rows {
            for col in 0..grid.columns {
                print!("{}", grid.bugs_adjacent_at(row, col));
            }
            println!();
        }
        println!();
    }
    println!("Final:\n{}", &grid);
    println!("Rounds: {}", rounds);
    println!("Biodiversity rating: {}", grid.biodiversity_rating());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn biodiversity_rating() {
        let grid: Grid = fs::read_to_string("input/24-example-2")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(grid.biodiversity_rating(), 2129920);
    }
    #[test]
    fn bugs_adjacent_at() {
        let grid: Grid = fs::read_to_string("input/24").unwrap().parse().unwrap();
        assert_eq!(grid.rows, 5);
        assert_eq!(grid.columns, 5);
        println!("{}", &grid);
        assert_eq!(grid.bugs_adjacent_at(0, 0), 1);
        assert_eq!(grid.bugs_adjacent_at(0, 1), 3);
        assert_eq!(grid.bugs_adjacent_at(0, 2), 2);
        assert_eq!(grid.bugs_adjacent_at(0, 2), 2);
        assert_eq!(grid.bugs_adjacent_at(0, 4), 0);
        assert_eq!(grid.bugs_adjacent_at(1, 0), 3);
        assert_eq!(grid.bugs_adjacent_at(1, 1), 2);
        assert_eq!(grid.bugs_adjacent_at(1, 2), 2);
        assert_eq!(grid.bugs_adjacent_at(1, 2), 2);
        assert_eq!(grid.bugs_adjacent_at(1, 4), 0);
        assert_eq!(grid.bugs_adjacent_at(4, 0), 2);
        assert_eq!(grid.bugs_adjacent_at(4, 1), 2);
        assert_eq!(grid.bugs_adjacent_at(4, 2), 2);
        assert_eq!(grid.bugs_adjacent_at(4, 2), 2);
        assert_eq!(grid.bugs_adjacent_at(4, 4), 2);
    }
}
