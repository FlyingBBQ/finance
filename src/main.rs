use std::io::{self, Write};

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

struct Info {
    hours: u32,
    fte: f32,
    gross: Salary,
    tax: Tax,
    net: Salary,
}

impl Info {
    fn new(input: Input) -> Info {
        let hours = input.hours;
        let fte = Info::calculate_fte(input.hours);
        let gross = Salary::new(input.salary, hours);
        let year = T2021;
        let tax = Tax::calculate_tax(gross.yearly, &year);
        let net = Salary::new(tax.calculate_net_salary(gross.yearly), hours);
        Info { hours, fte, gross, tax, net }
    }

    fn calculate_fte(hours: u32) -> f32 {
        hours as f32 / 40.0 * 100.0
    }

    // TODO: align printing
    fn print(&self) {
        println!("\n[Results]");
        println!("hours:  \t{}", self.hours);
        println!("FTE:    \t{:.1}%", self.fte);
        println!("\n[gross]");
        self.gross.print();
        println!("\n[tax]");
        self.tax.print();
        println!("\n[net]");
        self.net.print();
    }
}

struct Salary {
    hourly: f32,
    monthly: f32,
    yearly: u32,
}

impl Salary {
    fn new(salary: u32, hours: u32) -> Salary {
        let hourly = salary as f32 / (hours as f32 * 4.333);
        let monthly = salary as f32;
        let yearly = (1.08 * (salary * 12) as f32) as u32;
        Salary { hourly, monthly, yearly}
    }

    fn print(&self) {
        println!("salary: \t{}", self.monthly);
        println!("hourly: \t{:.2}", self.hourly);
        println!("yearly: \t{}", self.yearly);
    }
}

struct Tax {
    algemene_heffingskorting: u32,
    arbeidskorting: u32,
    box1: u32,
}

impl Tax {
    fn calculate_tax(year_salary: u32, tax: &(impl HeffingsKortingen + Boxen)) -> Tax {
        Tax {
            algemene_heffingskorting: tax.algemene_heffingskorting(year_salary),
            arbeidskorting: tax.arbeidskorting(year_salary),
            box1: tax.box1(year_salary),
        }
    }

    fn tax_to_pay(&self) -> u32 {
        self.box1 - (self.algemene_heffingskorting + self.arbeidskorting)
    }
    
    fn calculate_net_salary(&self, gross_yearly: u32) -> u32 {
        ((gross_yearly - self.tax_to_pay()) as f32 / 1.08) as u32 / 12
    }

    fn print(&self) {
        println!("                    box1: {}", self.box1);
        println!("algemene heffingskorting:  - {}", self.algemene_heffingskorting);
        println!("          arbeidskorting:  - {}", self.arbeidskorting);
        println!("                         -----------------");
        println!("                     tax: {}", self.tax_to_pay());
    }
}

// TODO: Generics for traits.
trait HeffingsKortingen {
    fn algemene_heffingskorting(&self, salary: u32) -> u32;
    fn arbeidskorting(&self, salary: u32) -> u32;
}

trait Boxen {
    fn box1(&self, salary: u32) -> u32;
}

// TODO: Move to separate year file.
struct T2021;

// TODO: Percentage calculation refactor with generics.

impl HeffingsKortingen for T2021 {
    fn algemene_heffingskorting(&self, salary: u32) -> u32 {
        if salary <= 21_043 {
            2837 
        } else if salary <= 68_507 {
            2837 - ((0.05_977 * (salary - 21_043) as f32) as u32)
        } else {
            0
        }
    }

    fn arbeidskorting(&self, salary: u32) -> u32 {
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

impl Boxen for T2021 {
    fn box1(&self, salary: u32) -> u32 {
        let percentage = if salary <= 68_508 { 0.37_10 } else { 0.49_50 };
        (percentage * salary as f32).round() as u32
    }
}

fn main() {
    let input = Input::get();
    let info = Info::new(input);
    info.print();
}
