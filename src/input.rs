use std::io::{self, Write};

pub struct Input {
    pub salary: u32,
    pub hours: u32,
}

impl Input {
    pub fn get() -> Input {
        let salary = Input::input_as_int("salary");
        let hours = Input::input_as_int("hours"); 
        Input { salary, hours }
    }

    fn input_as_int(text: &str) -> u32 {
        loop {
            // Get the input on the same line.
            print!("input {}: ", text);
            io::stdout().flush().unwrap();

            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            match input_text.trim().parse() {
                Ok(int) => if int > 0 {
                    return int
                } else {
                    println!("number must be > 0, retry");
                }
                Err(_) => {
                    println!("not a number, retry");
                }
            }
        }
    }
}
