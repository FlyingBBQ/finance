use std::fmt;
use crate::input::Input;
use crate::tax::tax::Tax;
use crate::tax::tax_2021::T2021;

pub mod printing {
    pub fn print_title(text: &str) {
        let title = format!("[{}]", text);
        println!("\n{:>19}", title);
    }

    pub fn print_row(text: &str, value: String) {
        println!("{:>18}: \t{}", text, value);
    }
}

struct Percentage(f32);

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.0)
    }
}

pub struct Info {
    hours: u32,
    fte: f32,
    gross: Salary,
    tax: Tax,
    net: Salary,
}

impl Info {
    pub fn new(input: Input) -> Info {
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

    pub fn print(&self) {
        printing::print_title("Results");
        printing::print_row("hours", self.hours.to_string());
        printing::print_row("FTE", Percentage(self.fte).to_string());

        printing::print_title("gross");
        self.gross.print();

        printing::print_title("tax");
        self.tax.print();
        //Tax::call_print(&self.tax);

        printing::print_title("net");
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
        printing::print_row("salary", self.monthly.to_string());
        printing::print_row("yearly", self.yearly.to_string());
        printing::print_row("hourly", format!("{:.2}", self.hourly));
    }
}

