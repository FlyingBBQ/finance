mod input;
mod info;
mod tax {
    pub mod tax;
    pub mod tax_2021;
}

use crate::input::Input;
use crate::info::Info;

fn main() {
    let input = Input::get();
    let info = Info::new(input);
    info.print();
}
