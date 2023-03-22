use std::{fs, thread};
use std::io::{self,prelude::*,Write, Read};


const PERIOD: u32 = 500000;

fn main() {

    loop {
        let mut input = String::new();
        println!("Input new brightness level (0-100):");
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Failed to read input");
        let brightness = input.parse::<u32>().unwrap();
        let duty_cycle = get_duty(brightness);


        fs::write("/sys/class/pwm/pwmchip7/pwm1/duty_cycle", duty_cycle)
            .expect("Unable to write file");

    }
}

fn get_duty(brightness: u32) -> String {
    return (PERIOD * brightness/100).to_string()
}

