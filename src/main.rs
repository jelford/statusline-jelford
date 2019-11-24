use std::{thread, time};
use std::fs::File;
use std::io::Read;
use getopts::Options;
use chrono::prelude::*;

const DEFAULT_POLL_INTERVAL : u64 = 1;

#[derive(Debug, PartialEq)]
enum BatteryStatus {
   Charging,
   Discharging,
   Full,
   Unknown,
}

#[derive(Debug)]
struct ParseBatteryStatusError(String);

impl std::str::FromStr for BatteryStatus {
   type Err = ParseBatteryStatusError;
   fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s.trim() {
      	    "Charging" => Ok(BatteryStatus::Charging),
	    "Discharging" => Ok(BatteryStatus::Discharging),
	    "Full" => Ok(BatteryStatus::Full),
	    "Unknown" => Ok(BatteryStatus::Unknown),
	    _ => Err(ParseBatteryStatusError(s.to_string())),
      }
   }
}

impl std::fmt::Display for BatteryStatus {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      let str_rep = match self {
         BatteryStatus::Charging => "Charging",
	 BatteryStatus::Discharging => "Discharging",
	 BatteryStatus::Full => "Full",
	 BatteryStatus::Unknown => "Unknown",
      };
      
      write!(f, "{}", str_rep)
   }
}

fn get_battery_status() -> Option<BatteryStatus> {
   let bat_status = File::open("/sys/class/power_supply/BAT0/status");
   if let Err(_) = bat_status {
      return None;
   }

   let mut bat_status = bat_status.unwrap();
   let mut bat_status_string = String::new();
   bat_status.read_to_string(&mut bat_status_string).expect("Unable to read battery status");
   let bat_status: BatteryStatus = bat_status_string.parse().expect(&format!("Unable to parse battery status: {}", bat_status_string));
   Some(bat_status)
}

fn get_battery_level() -> Option<u8> {
   let bat_level = File::open("/sys/class/power_supply/BAT0/capacity");
   if let Err(_) = bat_level {
      return None;
   }

   let mut bat_level = bat_level.unwrap();
   let mut bat_level_str = String::new();
   bat_level.read_to_string(&mut bat_level_str).expect("Unable to read battery capacity");
   Some(bat_level_str.trim().parse().expect("Reported battery capacity not a number in range 1..100"))
}

fn do_print_status_line() {
   let date_str = Local::now().format("%Y-%m-%d %H:%M");
   let bat_status = get_battery_status().unwrap();
   let bat_level = if bat_status != BatteryStatus::Full {
       get_battery_level().unwrap()
   } else {
       100
   };

   let bat_status_display = match bat_status {
       BatteryStatus::Full => "🔋✓".to_string(),
       BatteryStatus::Charging => format!("🔋{}+", bat_level),
       BatteryStatus::Discharging => format!("🔋{}-", bat_level),
       BatteryStatus::Unknown => "🔋?!".to_string(),
   };
   
   println!("{} - {}", date_str, bat_status_display);
}

fn main() {
   let args: Vec<String> = std::env::args().collect();
   let mut opts = Options::new();
   opts.optflagopt("c", "continuous", "run forever, printing a new line when things change. If an option is given, interpreted as poll interval", "INTERVAL");
   let opts = opts.parse(&args[1..]).expect("Invalid arguments passed");

   let continuous = opts.opt_present("c");
   let interval = opts.opt_str("c").map(|i| i.parse());

   if continuous {
      let interval = interval.unwrap_or(Ok(DEFAULT_POLL_INTERVAL)).expect("Unable to parse poll interval as string");
      loop {
          do_print_status_line();
	  thread::sleep(time::Duration::from_secs(interval));
      }
   } else {
      do_print_status_line();
   }

}

