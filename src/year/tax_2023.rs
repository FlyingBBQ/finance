use crate::tax::{Boxen, HeffingsKortingen, TaxUtil};

pub struct T2023;

impl TaxUtil for T2023 {}

impl HeffingsKortingen for T2023 {
    fn algemene_heffingskorting(&self, salary: u32) -> u32 {
        if salary < 22_661 {
            3070
        } else if salary < 73_031 {
            3070 - self.calculate_percentage(0.06_095, salary - 22_660)
        } else {
            0
        }
    }

    fn arbeidskorting(&self, salary: u32) -> u32 {
        if salary < 10_741 {
            self.calculate_percentage(0.08_231, salary)
        } else if salary < 23_201 {
            884 + self.calculate_percentage(0.29_861, salary - 10_740)
        } else if salary < 37_691 {
            4_605 + self.calculate_percentage(0.03_085, salary - 23_200)
        } else if salary < 115_295 {
            5_052 - self.calculate_percentage(0.06_510, salary - 37_690)
        } else {
            0
        }
    }
}

impl Boxen for T2023 {
    fn box1(&self, salary: u32) -> u32 {
        let percentage = if salary < 73_031 { 0.36_93 } else { 0.49_50 };
        self.calculate_percentage(percentage, salary)
    }
}
