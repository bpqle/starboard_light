use std::fs;
use std::time::{Duration, SystemTime};
use sun_times;
use chrono;
use chrono::{Utc, DateTime, Timelike};
use std::cmp::min;

fn fake_sun_pos(dawn: chrono::DateTime<Utc>, dusk: chrono::DateTime<Utc>)  -> Result<u32, E> {
    const NOW: DateTime<Utc> = DateTime::from(SystemTime::now());
    let now = NOW.hour() + (NOW.minute() / 60) + (NOW.second() / 3600);
    let dawn = dawn.hour() + (dawn.minute() / 60) + (dawn.second() / 3600);
    let dusk = dusk.hour() + (dusk.minute() / 60) + (dusk.second() / 3600);
    let x = (hour + 24 - dawn) % 24;
    let y = (dusk + 24 - hour) % 24;
    Ok(min(2, x/y)*3.14)
}


pub struct Light {

}

impl Light {
    const par_ephemera: bool = true;
    const state_sun_alt: u32 = 0;

    fn clock_brightness() {
        if (Self::par_ephemera == true) {
            Self::state_sun_alt =
        }
    }
}

fn main() {
    println!("Hello, world!");
}
"/sys/class/leds/starboard::lights"