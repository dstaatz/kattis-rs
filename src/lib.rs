use std::io::{stdin, BufRead, Lines, StdinLock};
use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};

pub fn run() -> Result<()> {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    loop {
        let n = usize::from_str(&get_line(&mut lines)?)?;
        if n == 0 {
            // Process as many cases as are given
            return Ok(());
        }

        let mut boxes = Vec::new();
        for _ in 0..n {
            boxes.push(PeanutBox::from_str(&get_line(&mut lines)?)?);
        }

        let m = usize::from_str(&get_line(&mut lines)?)?;

        let mut peanuts = Vec::new();
        for _ in 0..m {
            peanuts.push(Peanut::from_str(&get_line(&mut lines)?)?);
        }

        println!("Boxes: {:#?}", boxes);
        println!("Peanuts: {:#?}", peanuts);
    }
}

fn get_line(lines: &mut Lines<StdinLock<'_>>) -> Result<String> {
    Ok(lines.next().ok_or_else(|| anyhow!("Expected"))??)
}

#[derive(Debug, Clone, Copy)]
enum PeanutType {
    Small,
    Medium,
    Large,
}

impl FromStr for PeanutType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match &s.to_lowercase()[..] {
            "small" => Ok(Self::Small),
            "medium" => Ok(Self::Medium),
            "large" => Ok(Self::Large),
            _ => bail!("Failed to parse peanut type from {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum PeanutStatus {
    Small,
    Medium,
    Large,
    Floor,
    Correct,
}

#[derive(Debug, Clone, Copy)]
struct Peanut {
    x: f64,
    y: f64,
    type_: PeanutType,
}

impl FromStr for Peanut {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.split_whitespace();

        let x = f64::from_str(
            iter.next()
                .ok_or_else(|| anyhow!("Not enough data for Peanut"))?,
        )?;
        let y = f64::from_str(
            iter.next()
                .ok_or_else(|| anyhow!("Not enough data for Peanut"))?,
        )?;
        let type_ = PeanutType::from_str(
            iter.next()
                .ok_or_else(|| anyhow!("Not enough data for Peanut"))?,
        )?;

        Ok(Self { x, y, type_ })
    }
}

#[derive(Debug, Clone, Copy)]
struct PeanutBox {
    // (min, max)
    x: (f64, f64),
    // (min, max)
    y: (f64, f64),
    type_: PeanutType,
}

impl PeanutBox {
    fn is_inside(&self, peanut: &Peanut) -> bool {
        self.x.0 < peanut.x && peanut.x < self.x.1 && self.y.0 < peanut.y && peanut.y < self.y.1
    }
}

impl FromStr for PeanutBox {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.split_whitespace();

        let x1 = f64::from_str(
            iter.next()
                .ok_or_else(|| anyhow!("Not enough data for PeanutBox"))?,
        )?;
        let y1 = f64::from_str(
            iter.next()
                .ok_or_else(|| anyhow!("Not enough data for PeanutBox"))?,
        )?;
        let x2 = f64::from_str(
            iter.next()
                .ok_or_else(|| anyhow!("Not enough data for PeanutBox"))?,
        )?;
        let y2 = f64::from_str(
            iter.next()
                .ok_or_else(|| anyhow!("Not enough data for PeanutBox"))?,
        )?;
        let type_ = PeanutType::from_str(
            iter.next()
                .ok_or_else(|| anyhow!("Not enough data for PeanutBox"))?,
        )?;

        Ok(Self {
            x: (x1, x2),
            y: (y1, y2),
            type_,
        })
    }
}
