// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug, PartialEq)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn main() {
    // let tile = Tile::Brick(BrickStyle::Red);
    // let tile = Tile::Brick(BrickStyle::Dungeon);
    // let tile = Tile::Water(Pressure(12));
    // let tile = Tile::Water(Pressure(9));
    // let tile = Tile::Grass;
    // let tile = Tile::Dirt;
    // let tile = Tile::Sand;
    // let tile = Tile::Wood;
    let tile = Tile::Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 105,
    });

    match tile {
        Tile::Brick(color @ BrickStyle::Red | color @ BrickStyle::Gray) => {
            println!("The brick color is {:?}", color)
        }
        Tile::Brick(brick_style) => println!("{:?} brick", brick_style),
        Tile::Water(Pressure(pressure)) if pressure >= 10 => println!("High water pressure"),
        Tile::Water(Pressure(pressure)) if pressure < 10 => {
            println!("Water pressure level: {}", pressure)
        }
        Tile::Grass | Tile::Dirt | Tile::Sand => println!("Ground tile"),
        Tile::Treasure(TreasureChest { content, amount })
            if content == TreasureItem::Gold && amount >= 100 =>
        {
            println!("Lots of gold")
        }
        _ => println!("Rest of them"),
    }
}

// fn data() -> &'static [u64] {
//     &[5, 5, 4, 4, 3, 3, 1]
// }
//
// fn main() {
//     // `stream` is an iterator of Option<&[u64]>
//     let mut stream = data().chunks(2);
//
//     while let Some(slice_data) = stream.next() {
//         match slice_data {
//             [first, second, ..] => println!("Pair sum: {:?}", (first, second, first + second)),
//             [single] => println!("Unpaired value: {}", single),
//             [] => println!("Data stream complete"),
//         }
//     }
// }
//
