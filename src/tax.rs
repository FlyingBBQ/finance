use crate::printing;

pub struct Tax {
    algemene_heffingskorting: u32,
    arbeidskorting: u32,
    box1: u32,
}

impl Tax {
    pub fn calculate_tax(year_salary: u32, tax: &(impl HeffingsKortingen + Boxen)) -> Tax {
        Tax {
            algemene_heffingskorting: tax.algemene_heffingskorting(year_salary),
            arbeidskorting: tax.arbeidskorting(year_salary),
            box1: tax.box1(year_salary),
        }
    }

    pub fn calculate_net_salary(&self, gross_yearly: u32) -> u32 {
        ((gross_yearly - self.tax_to_pay()) as f32 / 1.08) as u32 / 12
    }

    fn tax_to_pay(&self) -> u32 {
        self.box1 - (self.algemene_heffingskorting + self.arbeidskorting)
    }

    pub fn print(&self) {
        printing::print_row("yearly", format!("{:>7}", printing::print_int(self.box1)));
        printing::print_row(
            "heffingskorting",
            format!("- {}", printing::print_int(self.algemene_heffingskorting)),
        );
        printing::print_row(
            "arbeidskorting",
            format!("- {}", printing::print_int(self.arbeidskorting)),
        );
        printing::print_row(
            "tax",
            format!("{:>7}", printing::print_int(self.tax_to_pay())),
        );
    }
}

pub trait TaxUtil {
    fn calculate_percentage(&self, percentage: f32, salary: u32) -> u32 {
        (percentage * (salary as f32)).round() as u32
    }
}

// TODO: Generics for traits.
pub trait HeffingsKortingen: TaxUtil {
    fn algemene_heffingskorting(&self, salary: u32) -> u32;
    fn arbeidskorting(&self, salary: u32) -> u32;
}

pub trait Boxen: TaxUtil {
    fn box1(&self, salary: u32) -> u32;
}
