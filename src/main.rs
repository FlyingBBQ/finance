use std::fmt;
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

fn print_title(text: &str) {
    let title = format!("[{}]", text);
    println!("\n{:>19}", title);
}

fn print_row(text: &str, value: String) {
    println!("{:>18}: \t{}", text, value);
}

struct Percentage(f32);

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.0)
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

    fn print(&self) {
        print_title("Results");
        print_row("hours", self.hours.to_string());
        print_row("FTE", Percentage(self.fte).to_string());

        print_title("gross");
        self.gross.print();

        print_title("tax");
        self.tax.print();

        print_title("net");
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
        print_row("salary", self.monthly.to_string());
        print_row("yearly", self.yearly.to_string());
        print_row("hourly", format!("{:.2}", self.hourly));
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
        print_row("box1", self.box1.to_string());
        print_row("heffingskorting", format!("- {}", self.algemene_heffingskorting.to_string()));
        print_row("arbeidskorting", format!("- {}", self.arbeidskorting.to_string()));
        print_row("tax", self.tax_to_pay().to_string());
    }
}

trait TaxUtil {
    fn calculate_percentage(&self, percentage: f32, salary: u32) -> u32 {
        (percentage * (salary as f32)).round() as u32
    }
}

// TODO: Generics for traits.
trait HeffingsKortingen: TaxUtil {
    fn algemene_heffingskorting(&self, salary: u32) -> u32;
    fn arbeidskorting(&self, salary: u32) -> u32;
}

trait Boxen: TaxUtil {
    fn box1(&self, salary: u32) -> u32;
}

// TODO: Move to separate year file.
struct T2021;

impl TaxUtil for T2021 {}

impl HeffingsKortingen for T2021 {
    fn algemene_heffingskorting(&self, salary: u32) -> u32 {
        if salary <= 21_043 {
            2837 
        } else if salary <= 68_507 {
            2837 - self.calculate_percentage(0.05_977, salary - 21_043)
        } else {
            0
        }
    }

    fn arbeidskorting(&self, salary: u32) -> u32 {
        if salary <= 10_108 {
            self.calculate_percentage(0.04_581, salary)
        } else if salary <= 21_835 {
            463 + self.calculate_percentage(0.28_771, salary - 10_108)
        } else if salary <= 35_652 {
            3_837 + self.calculate_percentage(0.02_663, salary - 21_835)
        } else if salary <= 105_736 {
            4_205 - self.calculate_percentage(0.06, salary - 35_652)
        } else {
            0
        }
    }
}

impl Boxen for T2021 {
    fn box1(&self, salary: u32) -> u32 {
        let percentage = if salary <= 68_508 { 0.37_10 } else { 0.49_50 };
        self.calculate_percentage(percentage, salary)
    }
}

fn main() {
    let input = Input::get();
    let info = Info::new(input);
    info.print();
}
