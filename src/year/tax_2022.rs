use crate::tax::{TaxUtil, HeffingsKortingen, Boxen};

pub struct T2022;

impl TaxUtil for T2022 {}

impl HeffingsKortingen for T2022 {
    fn algemene_heffingskorting(&self, salary: u32) -> u32 {
        if salary <= 21_318 {
            2888 
        } else if salary <= 69_399 {
            2888 - self.calculate_percentage(0.06_007, salary - 21_318)
        } else {
            0
        }
    }

    fn arbeidskorting(&self, salary: u32) -> u32 {
        if salary <= 10_351 {
            self.calculate_percentage(0.04_541, salary)
        } else if salary <= 22_357 {
            470 + self.calculate_percentage(0.28_461, salary - 10_351)
        } else if salary <= 36_650 {
            3_887 + self.calculate_percentage(0.02_610, salary - 22_357)
        } else if salary <= 109_347 {
            4_260 - self.calculate_percentage(0.05_860, salary - 36_650)
        } else {
            0
        }
    }
}

impl Boxen for T2022 {
    fn box1(&self, salary: u32) -> u32 {
        let percentage = if salary <= 69_399 { 0.37_07 } else { 0.49_50 };
        self.calculate_percentage(percentage, salary)
    }
}
