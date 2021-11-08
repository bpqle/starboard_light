use std::{fs, thread};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sun_times;
use chrono;
use chrono::{Utc, DateTime, Timelike};
use std::cmp::{min,max};
use sun;
use std::fs::OpenOptions;
use std::io::{self,prelude::*,Write, Read};

fn main() {
    const PAR_EPHEMERA: bool = true;
    const LAT: f64 = 38.03;
    const LON: f64 = -78.51;
    const DAY_START: u32 = 7;
    const DAY_STOP: u32 = 19;
    const MAX_BRIGHTNESS: f64 = 255.0;
    let now: DateTime<Utc> = DateTime::from(SystemTime::now());


    let now = now.hour() + (now.minute() / 60) + (now.second() / 3600);
    //let dawn = dawn.hour() + (dawn.minute() / 60) + (dawn.second() / 3600);
    //let dusk = dusk.hour() + (dusk.minute() / 60) + (dusk.second() / 3600);
    let x  = (now + 24 - DAY_START) % 24;
    let y = (DAY_STOP + 24 - now) % 24;
    let result: f64 = (1.6/1.8) as f64 * 3.14;

    let mut sun_altitude: f64 = 0.0;

    if PAR_EPHEMERA == true {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        sun_altitude = sun::pos(now.as_millis() as i64, LAT, LON)
            .altitude;
    } else {
        sun_altitude = result;
    }
    let brightness = (sun_altitude.sin() * MAX_BRIGHTNESS) as u8;
    let brightness = max(0, brightness);
    //println!("Brightness is {:?}", brightness);

    let mut light = OpenOptions::new()
        .write(true)
        .read(true)
        //.truncate(true)
        .open("/sys/class/leds/starboard::lights/brightness")
        .unwrap();
    let mut buffer= Vec::new();
    let n = light.read(&mut buffer).unwrap();
    println!("{:?}",&buffer);
    light.write(&[brightness]).unwrap();
    let n = light.read(&mut buffer).unwrap();
    println!("{:?}",buffer);
    loop {
        thread::sleep(Duration::from_secs(5))
    }
}
