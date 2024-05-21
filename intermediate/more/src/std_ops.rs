use std::ops::{Add, Index};

#[derive(Debug)]
struct Speed(u32);

impl Add<Self> for Speed {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<u32> for Speed {
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

struct Letter(char);

impl Add<Self> for Letter {
    type Output = String;
    fn add(self, rhs: Self) -> Self::Output {
        format!("{}{}", self.0, rhs.0)
    }
}

struct Hvac {
    current_temp: i16,
    max_temp: i16,
    min_temp: i16,
}

enum Temp {
    Current,
    Max,
    Min,
}

impl Index<Temp> for Hvac {
    type Output = i16;
    fn index(&self, index: Temp) -> &Self::Output {
        match index {
            Temp::Current => &self.current_temp,
            Temp::Max => &self.max_temp,
            Temp::Min => &self.min_temp,
        }
    }
}

fn main() {
    let output = Speed(1) + Speed(2) + Speed(3);
    let output = Speed(1) + 2;
    let output = Letter('a') + Letter('b');

    let hvac = Hvac {
        current_temp: 33,
        max_temp: 38,
        min_temp: 30,
    };

    let output = hvac[Temp::Current];

    println!("{:?}", output);
}
