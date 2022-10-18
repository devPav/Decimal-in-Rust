use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::io;

fn main() {
    //Градус Фаренгейта в Цельсий.
    loop {
        let mut farengeit = String::new();

        println!("Input Farengeit");
        io::stdin()
            .read_line(&mut farengeit)
            .expect("Error input panic");

        let farengeit: f64 = match farengeit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input correct number");
                continue;
            }
        };
        let farengeit = match Decimal::from_f64(farengeit) {
            Some(num) => num,
            None => panic!("atal error"),
        };
        let celsiy = (farengeit - dec!(32)) / dec!(1.8);
        println!("Is {} celsiy", celsiy);
        break;
    }
}
