use std::io::{self, Write};

#[derive(Copy, Clone)]
struct Input {
    salary: u32,
    hours: u32,
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

struct Tax {
    box1: u32,
}

struct Salary {
    base: Input,
    hourly: f32,
    yearly: u32,
    fte: f32,
}

impl Salary {
    fn new(input: Input) -> Salary {
        let hourly = input.salary as f32 / (input.hours as f32 * 4.333);
        let yearly = (1.08 * (input.salary * 12) as f32) as u32;
        let fte = input.hours as f32 / 40.0 * 100.0;
        Salary { base: input, hourly, yearly, fte }
    }
}

trait HeffingsKortingen {
    fn algemene_heffingskorting(salary: u32) -> u32;
    fn arbeidskorting(salary: u32) -> u32;
}

struct HK2021;

impl HeffingsKortingen for HK2021 {
    fn algemene_heffingskorting(salary: u32) -> u32 {
        if salary <= 21_043 {
            2837 
        } else if salary <= 68_507 {
            2837 - ((0.05_977 * (salary - 21_043) as f32) as u32)
        } else {
            0
        }
    }
    fn arbeidskorting(salary: u32) -> u32 {
        if salary <= 10_108 {
            (0.04_581 * salary as f32) as u32
        } else if salary <= 21_835 {
            463 + ((0.28_771 * (salary - 10_108) as f32) as u32)
        } else if salary <= 35_652 {
            3_837 + ((0.02_663 * (salary - 21_835) as f32) as u32)
        } else if salary <= 105_736 {
            4_205 - ((0.06 * (salary - 35_652) as f32) as u32)
        } else {
            0
        }
    }
}

fn main() {
    let input = Input::get();
    let salary = Salary::new(input);

    println!("\n[Results]");
    println!("salary: \t{}", salary.base.salary);
    println!("hours:  \t{}", salary.base.hours);
    println!("FTE:    \t{:.1}%", salary.fte);
    println!("hourly: \t{:.2}", salary.hourly);
    println!("yearly: \t{}", salary.yearly);
}
