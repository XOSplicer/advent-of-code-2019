use anyhow::Result as AnyResult;
use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt;
use std::fs;

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
}

impl Default for Vec3D {
    fn default() -> Self {
        Self::new(0, 0, 0)
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
    fn into_components(self) -> MoonComponents {
        MoonComponents {
            x: MoonComponent {
                position: self.position.x,
                velocity: self.velocity.x,
            },
            y: MoonComponent {
                position: self.position.y,
                velocity: self.velocity.y,
            },
            z: MoonComponent {
                position: self.position.z,
                velocity: self.velocity.z,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct MoonComponents {
    x: MoonComponent,
    y: MoonComponent,
    z: MoonComponent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MoonComponent {
    position: i64,
    velocity: i64,
}

impl MoonComponent {
    // only applies gravity to this moon, not the other
    fn apply_gravity(&mut self, other: &MoonComponent) {
        self.velocity += match self.position.cmp(&other.position) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        }
    }
    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }
}

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() -> AnyResult<()> {
    let moons = fs::read_to_string("input/12")?
        .lines()
        .map(Vec3D::from)
        .enumerate()
        .map(|(i, p)| Moon::new(i, p))
        .collect::<Vec<_>>();
    let initial_xs = moons
        .iter()
        .cloned()
        .map(|m| m.into_components().x)
        .collect::<Vec<_>>();
    let initial_ys = moons
        .iter()
        .cloned()
        .map(|m| m.into_components().y)
        .collect::<Vec<_>>();
    let initial_zs = moons
        .iter()
        .cloned()
        .map(|m| m.into_components().z)
        .collect::<Vec<_>>();

    let mut xs = initial_xs.clone();
    let mut ys = initial_ys.clone();
    let mut zs = initial_zs.clone();

    let mut cycle_x = 0;
    let mut cycle_y = 0;
    let mut cycle_z = 0;

    {
        let im = xs.clone();
        for n in im {
            for m in xs.iter_mut() {
                m.apply_gravity(&n);
            }
        }
        for m in xs.iter_mut() {
            m.apply_velocity()
        }
        cycle_x += 1;
    }

    while xs != initial_xs {
        let im = xs.clone();
        for n in im {
            for m in xs.iter_mut() {
                m.apply_gravity(&n);
            }
        }
        for m in xs.iter_mut() {
            m.apply_velocity()
        }
        cycle_x += 1;
    }

    {
        let im = ys.clone();
        for n in im {
            for m in ys.iter_mut() {
                m.apply_gravity(&n);
            }
        }
        for m in ys.iter_mut() {
            m.apply_velocity()
        }
        cycle_y += 1;
    }

    while ys != initial_ys {
        let im = ys.clone();
        for n in im {
            for m in ys.iter_mut() {
                m.apply_gravity(&n);
            }
        }
        for m in ys.iter_mut() {
            m.apply_velocity()
        }
        cycle_y += 1;
    }

    {
        let im = zs.clone();
        for n in im {
            for m in zs.iter_mut() {
                m.apply_gravity(&n);
            }
        }
        for m in zs.iter_mut() {
            m.apply_velocity()
        }
        cycle_z += 1;
    }

    while zs != initial_zs {
        let im = zs.clone();
        for n in im {
            for m in zs.iter_mut() {
                m.apply_gravity(&n);
            }
        }
        for m in zs.iter_mut() {
            m.apply_velocity()
        }
        cycle_z += 1;
    }

    println!("cycle x: {}", cycle_x);
    println!("cycle y: {}", cycle_y);
    println!("cycle z: {}", cycle_z);

    let full_cycle = lcm(lcm(cycle_x, cycle_y), cycle_z);
    println!("full cycle: {}", full_cycle);

    Ok(())
}
