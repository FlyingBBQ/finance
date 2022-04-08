mod input;
mod tax;
mod year {
    pub mod tax_2021;
    pub mod tax_2022;
}

use std::fmt;
use crate::input::Input;
use crate::tax::Tax;
use crate::year::tax_2022::T2022;

fn main() {
    let input = Input::get();
    let info = Info::new(input);
    info.print();
}

pub mod printing {
    pub fn print_title(text: &str) {
        let title = format!("[{}]", text);
        println!("\n{:>19}", title);
    }

    pub fn print_row(text: &str, value: String) {
        println!("{:>18}: \t{}", text, value);
    }

    pub fn print_int(i: u32) -> String {
        let mut s = String::new();
        let str = i.to_string();
        let iterator = str.chars().rev().enumerate();
        for (index, val) in iterator {
            if index != 0 && index % 3 == 0 {
                s.insert(0, '.');
            }
            s.insert(0, val);
        }
        s
    }

    pub fn print_float(f: f32) -> String {
        let str = format!("{:.2}", f);
        str.trim().replace('.', ",").parse().unwrap()
    }
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
        let gross = Salary::new(input.salary as f32 * fte, hours, true);
        let year = T2022;
        let tax = Tax::calculate_tax(gross.yearly, &year);
        let net = Salary::new(tax.calculate_net_salary(gross.yearly) as f32, hours, false);
        Info { hours, fte, gross, tax, net }
    }

    fn calculate_fte(hours: u32) -> f32 {
        hours as f32 / 40.0
    }

    fn print(&self) {
        printing::print_title("Results");
        printing::print_row("hours", self.hours.to_string());
        printing::print_row("FTE", Percentage(self.fte * 100.0).to_string());

        printing::print_title("gross");
        self.gross.print();

        printing::print_title("tax");
        self.tax.print();

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
    fn new(salary: f32, hours: u32, holiday: bool) -> Salary {
        let hourly = salary / (hours as f32 * 4.333);
        let monthly = salary;
        let yearly = (if holiday { 1.08 } else { 1.0 } * (salary * 12.0)) as u32;
        Salary { hourly, monthly, yearly}
    }

    fn print(&self) {
        printing::print_row("monthly",format!("{:>7}", printing::print_int(self.monthly as u32)));
        printing::print_row("yearly", format!("{:>7}", printing::print_int(self.yearly)));
        printing::print_row("hourly", format!("{:>10}", printing::print_float(self.hourly)));
    }
}

