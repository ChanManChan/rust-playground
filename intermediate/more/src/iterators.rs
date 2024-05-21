// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game powerups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 powerup obtained), 5, 7, 9, ...
//
// Requirements:
// * Write a program that uses an iterator to generate a score multiplier
// * The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through powerups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity

// struct Score {
//     current: u8,
//     powerup: u8,
// }
//
// impl Iterator for Score {
//     type Item = u8;
//     fn next(&mut self) -> Option<Self::Item> {
//         let next_value = self.current + self.powerup;
//         self.current = next_value;
//         Some(next_value)
//     }
// }
//
// fn main() {
//     let mut score = Score {
//         current: 0,
//         powerup: 1,
//     };
//
//     let first = score.next().unwrap();
//     println!("{}", first);
//
//     let second = score.next().unwrap();
//     println!("{}", second);
//
//     let third = score.next().unwrap();
//     println!("{}", third);
//
//     let fourth = score.next().unwrap();
//     println!("{}", fourth);
//     score.powerup = 2;
//
//     println!("Power up changed to 2");
//
//     let fifth = score.next().unwrap();
//     println!("{}", fifth);
//
//     let sixth = score.next().unwrap();
//     println!("{}", sixth);
//
//     score.powerup = 3;
//
//     println!("Power up changed to 3");
//
//     let seventh = score.next().unwrap();
//     println!("{}", seventh);
//
//     let eighth = score.next().unwrap();
//     println!("{}", eighth);
// }

use std::collections::{
    hash_map::{IntoIter, Iter, IterMut},
    HashMap,
};

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

struct ColorIntoIter {
    color: Color,
    position: u8,
}

struct ColorIter<'a> {
    color: &'a Color,
    position: u8,
}

impl Iterator for ColorIntoIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let next_value = match self.position {
            0 => Some(self.color.r),
            1 => Some(self.color.g),
            2 => Some(self.color.b),
            _ => None,
        };

        self.position += 1;
        next_value
    }
}

impl Iterator for ColorIter<'_> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let next_value = match self.position {
            0 => Some(self.color.r),
            1 => Some(self.color.g),
            2 => Some(self.color.b),
            _ => None,
        };

        self.position += 1;
        next_value
    }
}

impl IntoIterator for Color {
    type Item = u8;
    type IntoIter = ColorIntoIter;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            color: self,
            position: 0,
        }
    }
}

impl<'a> IntoIterator for &'a Color {
    type Item = u8;
    type IntoIter = ColorIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            color: &self,
            position: 0,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Fruit {
    Banana,
    Apple,
    Orange,
}

struct FruitStand {
    fruits: HashMap<Fruit, u8>,
}

impl IntoIterator for FruitStand {
    type Item = (Fruit, u8);
    type IntoIter = IntoIter<Fruit, u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruits.into_iter()
    }
}

impl<'a> IntoIterator for &'a FruitStand {
    type Item = (&'a Fruit, &'a u8);
    type IntoIter = Iter<'a, Fruit, u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruits.iter()
    }
}

impl<'a> IntoIterator for &'a mut FruitStand {
    type Item = (&'a Fruit, &'a mut u8);
    type IntoIter = IterMut<'a, Fruit, u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.fruits.iter_mut()
    }
}

fn main() {
    let mut fruits = HashMap::new();
    fruits.insert(Fruit::Banana, 12);
    fruits.insert(Fruit::Apple, 3);
    fruits.insert(Fruit::Orange, 17);

    let fruits = fruits;

    let mut fruit_stand = FruitStand { fruits };

    for fruit in &fruit_stand {
        println!("Data: {:?}", fruit);
    }

    for fruit in &fruit_stand {
        println!("Data: {:?}", fruit);
    }

    for fruit in &mut fruit_stand {
        *fruit.1 = *fruit.1 + 20;
        println!("Data: {:?}", fruit);
    }

    // let color = Color {
    //     r: 200,
    //     g: 16,
    //     b: 20,
    // };
    //
    // for shade in &color {
    //     println!("{}", shade);
    // }
    //
    // for shade in &color {
    //     println!("{}", shade);
    // }
}
