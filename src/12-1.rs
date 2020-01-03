use anyhow::Result as AnyResult;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::ops::{Add, Neg};

#[derive(Debug, Clone)]
struct Vec3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3D {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
    fn unit_x() -> Self {
        Self { x: 1, y: 0, z: 0 }
    }
    fn unit_y() -> Self {
        Self { x: 0, y: 1, z: 0 }
    }
    fn unit_z() -> Self {
        Self { x: 0, y: 0, z: 1 }
    }
    fn abssum(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Default for Vec3D {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl<'a> Neg for &'a Vec3D {
    type Output = Vec3D;
    fn neg(self) -> Vec3D {
        Vec3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for Vec3D {
    type Output = Vec3D;
    fn neg(self) -> Vec3D {
        Vec3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<'a, 'b> Add<&'b Vec3D> for &'a Vec3D {
    type Output = Vec3D;
    fn add(self, other: &'b Vec3D) -> Vec3D {
        Vec3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> Add<Vec3D> for &'a Vec3D {
    type Output = Vec3D;
    fn add(self, other: Vec3D) -> Vec3D {
        Vec3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Vec3D> for Vec3D {
    type Output = Vec3D;
    fn add(self, other: Vec3D) -> Vec3D {
        Vec3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl fmt::Display for Vec3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={: >6}, y={: >6}, z={: >6}>", self.x, self.y, self.z)
    }
}

impl From<&str> for Vec3D {
    fn from(s: &str) -> Vec3D {
        let h: HashMap<&str, i64> = s
            .trim_start_matches('<')
            .trim_end_matches('>')
            .split(',')
            .map(|part| {
                let mut s = part.split('=');
                (
                    s.next().expect("No vec component name").trim(),
                    s.next()
                        .expect("No vec component value")
                        .trim()
                        .parse()
                        .expect("Invalid vec component value"),
                )
            })
            .collect();
        Vec3D {
            x: h["x"],
            y: h["y"],
            z: h["z"],
        }
    }
}

#[derive(Debug, Clone)]
struct Moon {
    id: usize,
    position: Vec3D,
    velocity: Vec3D,
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos={}, vel={}", self.position, self.velocity)
    }
}

impl Moon {
    fn new(id: usize, position: Vec3D) -> Self {
        Self {
            id,
            position,
            velocity: Vec3D::default(),
        }
    }
    // only applies gravity to this moon, not the other
    fn apply_gravity(&mut self, other: &Moon) {
        self.velocity = &self.velocity
            + match self.position.x.cmp(&other.position.x) {
                Ordering::Greater => -Vec3D::unit_x(),
                Ordering::Less => Vec3D::unit_x(),
                Ordering::Equal => Vec3D::default(),
            };
        self.velocity = &self.velocity
            + match self.position.y.cmp(&other.position.y) {
                Ordering::Greater => -Vec3D::unit_y(),
                Ordering::Less => Vec3D::unit_y(),
                Ordering::Equal => Vec3D::default(),
            };
        self.velocity = &self.velocity
            + match self.position.z.cmp(&other.position.z) {
                Ordering::Greater => -Vec3D::unit_z(),
                Ordering::Less => Vec3D::unit_z(),
                Ordering::Equal => Vec3D::default(),
            };
    }
    fn apply_velocity(&mut self) {
        self.position = &self.position + &self.velocity;
    }
    fn energy(&self) -> i64 {
        self.position.abssum() * self.velocity.abssum()
    }
}

fn main() -> AnyResult<()> {
    let mut moons = fs::read_to_string("input/12")?
        .lines()
        .map(Vec3D::from)
        .enumerate()
        .map(|(i, p)| Moon::new(i, p))
        .collect::<Vec<_>>();
    for _ in 0..1000 {
        let im_moons = moons.clone();
        for n in im_moons {
            for m in moons.iter_mut() {
                m.apply_gravity(&n);
            }
        }
        for m in moons.iter_mut() {
            m.apply_velocity()
        }
    }
    let answer: i64 = moons.iter().map(Moon::energy).sum();
    println!("{}", answer);
    Ok(())
}
