use crate::info::OSData;
use std::env;
use std::process;
use std::vec;
use comfy_table::{
    presets::UTF8_FULL,
    Attribute,
    Cell,
    Color,
    Row,
    Table,
    ContentArrangement
};

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
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    table.set_width(80);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.add_row(Row::from(vec![
        Cell::new(format!("{}", &data.host)).fg(Color::Cyan).add_attribute(Attribute::Bold),
        Cell::new(&data.user).fg(Color::Green),
    ]));

    table.add_row(Row::from(vec![
        Cell::new("OS").fg(Color::Blue).add_attribute(Attribute::Bold),
        Cell::new(&data.os),
    ]));

    table.add_row(Row::from(vec![
        Cell::new("Shell").fg(Color::Yellow).add_attribute(Attribute::Bold),
        Cell::new(&data.shell),
    ]));

    table.add_row(Row::from(vec![
        Cell::new("MEM").fg(Color::Magenta).add_attribute(Attribute::Bold),
        Cell::new(&data.mem),
    ]));

    table.add_row(Row::from(vec![
        Cell::new("CPU").fg(Color::DarkYellow).add_attribute(Attribute::Bold),
        Cell::new(&data.cpu)
    ]));
    println!("{table}");
}

