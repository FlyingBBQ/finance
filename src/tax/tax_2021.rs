use crate::tax::tax::{TaxUtil, HeffingsKortingen, Boxen};

pub struct T2021;

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
