#[derive(Debug)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move {
        x: i32,
        y: i32
    },
    Write(String)
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("I quit");
        },
        Message::ChangeColor(red, green, blue) => {
            println!("Red {}, Green {}, Blue {}", red, green, blue);
        },
        Message::Move { x, y } => {
            println!("X is {}, Y is {}", x, y);
        },
        Message::Write(text) => {
            println!("Text {}", text);
        }
    };
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;

    #[test]
    fn tests_large_enum() {
        let my_quit: Message = Message::Quit;
        let my_color: Message = Message::ChangeColor(10, 20, 30);
        let my_move: Message = Message::Move { x: 20, y: 50 };
        let my_write: Message = Message::Write("My name is grapes".to_string());

        process_message(my_write);
    }

    #[test]
    fn tests_match_literals() {
        let number: i32 = 20;

        let res: &str = match number {
            1 => "it was the first",
            2 | 3 | 5 | 7 | 15 | 20 => "We found it in the sequence",
            _ => "It was something else",
        };

        println!("Result is:: {}", res);
    }

    #[test]
    fn tests_match_option() {
        let some_num: Option<i32> = Some(10);

        let my_int: i32 = if let Some(i) = some_num {
            i
        } else {
            panic!("There was a problem");
        };

        println!("{}", my_int);

        // let res: i32 = match some_num {
        //     Some(i) => i,
        //     None => {
        //         panic!("There was a problem");
        //     }
        // };

        // println!("{:?}", some_num);
        // println!("{}", res);
    }

    #[test]
    fn tests_match_result() {
        let some_res: Result<i32, &str> = Ok(50);
        // let some_err: Result<i32, &str> = Err("There was a problem");

        // let res = match some_res {
        //     Ok(val) => val,
        //     Err(e) => panic!("{}", e) 
        // };

        // println!("{}", res);

        let my_int: i32 = if let Ok(r) = some_res {
            r
        } else {
            panic!("There was a problem");
        };

        println!("{}", my_int);
    }

    #[test]
    fn tests_match_guard() {
        let pair: (i32, i32) = (2, -2);

        match pair {
            (x, y) if x == y => println!("They match"),
            (x, y) if x + y == 0 => println!("They neutralize"),
            (_, y) if y == 2 => println!("Y is indeed +2"),
            _ => println!("We are not bothered")
        };
    }

    #[test]
    fn tests_match_struct() {
        struct Location {
            x: i32,
            y: i32
        }

        let location = Location {
            x: 20,
            y: 0
        };

        match location {
            Location { x, y: 0 } => println!("Y is on the axis"),
            Location { x: 0, y } => println!("X is on the axis"),
            Location { x, y } => println!("X and Y are not on the axis")
        };
    }

}