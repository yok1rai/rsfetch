use crate::info::OSData;
use std::env;
use std::process;

pub mod info;
pub mod utils;

struct Args {
    pub unit: String
}

impl Args {
    fn build(args: &[String]) -> Result<Args, ()> {
        let valid_units = ["kb", "mb", "gb"];
        let unit_option = match args.get(1) {
            Some(arg) => arg,
            None => "MB"
        };
        if !valid_units.contains(&unit_option.to_lowercase().as_str()) {
            return Err(())
        }
        Ok(Args { unit: unit_option.trim().to_string() })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let unit = match Args::build(&args) {
        Ok(option) => option.unit,
        Err(_) => {
            eprintln!("Invalid unit");
            process::exit(1);
        }
    };
    let data = OSData::new(&unit);
    println!(r#"
{}@{}
OS: {}
Shell: {}
Mem: {}
    "#,
data.host, data.user, data.os, data.shell, data.mem);
}

