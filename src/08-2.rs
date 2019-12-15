use anyhow::Result as AnyResult;
use itertools::Itertools;
use std::fmt;
use std::fs;
use std::iter;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pixel {
    Black,
    White,
    Tranparent,
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '0' => Self::Black,
            '1' => Self::White,
            '2' => Self::Tranparent,
            _ => panic!("Unkown pixel `{}`", c),
        }
    }
}

impl Pixel {
    fn invert(self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
            Self::Tranparent => Self::Tranparent,
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = match self {
            Self::Black => '■',
            Self::White => ' ',
            Self::Tranparent => ' ',
        };
        write!(f, "{}", d)
    }
}

fn merge_pixel(top: Pixel, bottom: Pixel) -> Pixel {
    if top == Pixel::Tranparent {
        bottom
    } else {
        top
    }
}

fn layer_to_string(layer_width: usize, layer: &[Pixel]) -> String {
    layer
        .iter()
        .map(|p| p.clone().invert().to_string())
        .chunks(layer_width)
        .into_iter()
        .map(|mut row| row.join(""))
        .join("\n")
}

fn main() -> AnyResult<()> {
    let layer_width = 25;
    let layer_heigt = 6;
    let layer_size = layer_width * layer_heigt;
    let answer = layer_to_string(
        layer_width,
        &fs::read_to_string("input/08")?
            .trim()
            .chars()
            .chunks(layer_size)
            .into_iter()
            .map(|layer| layer.map(Pixel::from))
            .map(|layer| {
                let s = layer.collect::<Vec<Pixel>>();
                println!("{}\n\n", layer_to_string(layer_width, &s));
                s.into_iter()
            })
            .fold(
                iter::repeat(Pixel::Tranparent)
                    .take(layer_size)
                    .collect::<Vec<Pixel>>(),
                |acc, x| {
                    acc.into_iter()
                        .zip(x)
                        .map(|(t, b)| merge_pixel(t, b))
                        .collect()
                },
            ),
    );
    println!("{}", answer);
    Ok(())
}
