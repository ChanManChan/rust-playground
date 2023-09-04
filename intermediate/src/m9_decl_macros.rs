#[cfg(test)]
mod tests {

    macro_rules! mad_skills {
        // ($x: expr) => {
        //     format!("You sent an expression: {}", $x)
        // };
        ($x: ty) => {
            match stringify!($x) {
                "i32" => "You sent an i32 type".to_string(),
                _ => "You sent something else".to_string()
            }
        }
    }

    #[test]
    fn tests_declarative_macro_examples() {
        println!("Hello 1");
        dbg!("Hello 2");
        let x: Vec<i32> = vec!(1, 2, 3);
        let formatted: String = format!("Hello 3 with vec {:?}", x);
        dbg!(formatted);
    }

    macro_rules! my_vec {
        ($($x: expr), +) => {
            {
                let mut temp_vec = Vec::new();

                $(
                    temp_vec.push($x);
                )+

                temp_vec
            }
        };
    }

    #[test]
    fn tests_declarative_macro() {
        // let some_vars: String = mad_skills!(1 + 2);
        // let some_vars: String = mad_skills!(u8);
        // dbg!(some_vars);
        let mut y: Vec<i32> = my_vec!(1);

        dbg!(y);

    }
}