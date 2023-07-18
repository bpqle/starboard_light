use std::fs;
use std::path::{self, PathBuf};
use std::time::{self, SystemTime};
use chrono::{self, Timelike, prelude::*};
use std::thread;
use serde::{Serialize, Deserialize};
use serde_yaml;
use serde_json;

fn main() {
    // let dc_rs = std::path::Path::new("~/.config/decide/config.yaml");
    // let dc_cdm = std::path::Path::new("~/decide/config/bbb-config.json");
    // if dc_rs.exists() {
    //     let contents = fs::read_to_string(dc_rs)
    //         .expect("Should have been able to read the decide-rs config file");
    //     let conf_file: ConfigFile = serde_yaml::from_str::<ConfigFile>(&contents)
    //         .expect("Should have been able to parse the decide-rs config file");
    // } else {
    //     let contents = fs::read_to_string(dc_cdm)
    //         .expect("Should have been able to read the decide config file");
    //     let conf_file: ConfigFile = serde_json::from_str::<ConfigFile>(&contents)
    //         .expect("Should have been able to parse the decide config file");
    //
    // }

    let write_loc = "/sys/class/leds/starboard::lights/brightness";

    loop {
        let altitude = calc_altitude(3.0, 19.0);
        let brightness = calc_brightness(altitude, 255);
        println!("Writing {:?} tp file", brightness);
        let path = fs::canonicalize(PathBuf::from(write_loc)).unwrap();
        fs::write(path, brightness.to_string()).expect("Unable to write brightness");
        thread::sleep(time::Duration::from_millis(300000))
    }
}


fn calc_altitude(dawn:f64, dusk:f64) -> f64 {
    let now: DateTime<Local> = Local::now();
    let hour = now.hour() as f64;
    let minute = now.minute() as f64;
    let second = now.second() as f64;
    let time = hour + minute / 60.0 + second / 3600.0;
    println!("Time is {}", time);
    let x: f64 = (time + 24.0 - dawn) % 24.0;
    let y: f64 = (dusk + 24.0 - dawn) % 24.0;
    let alt = (x / y) * std::f64::consts::PI;
    alt
}
fn calc_brightness(altitude: f64, max_brightness: u8) -> i8 {
    let x = altitude.sin() * (max_brightness as f64);
    let y = x.round() as i8;
    //let brightness = max(0.0, x); //trait 'Ord' is not implemented for '{float}'
    if y > 0 { y } else { 0 }
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    components: Vec<Component>
}

#[derive(Debug, Serialize, Deserialize)]
struct Component {
    dawn: Option<f64>,
    dusk: Option<f64>
}
