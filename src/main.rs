use std::io::{self, Write};

struct Cli {
    salary: u32,
    hours: u32,
}

fn input_as_int(text: &str) -> u32 {
    print!("input {}: ", text);
    io::stdout().flush().unwrap();

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    input_text.trim().parse::<u32>().expect("input was not an integer")
}

fn main() {
    let input = Cli {
        salary: input_as_int("salary"),
        hours: input_as_int("hours"),
    };

    assert!(input.salary > 0);
    assert!(input.hours > 0);

    println!("\n[Results]");
    println!("salary: \t{}", input.salary);
    println!("hours:  \t{}", input.hours);

    let fte = input.hours as f32 / 40.0 * 100.0;
    println!("FTE:    \t{:.1}%", fte);

    let hourly = input.salary as f32 / (input.hours as f32 * 4.333);
    println!("hourly: \t{:.2}", hourly);
}
