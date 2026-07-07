use std::env;
use crate::utils::read;

pub enum Unit {
    Gb,
    Mb,
    Kb,
}

impl Unit {
    fn get(unit: &str) -> Self {
        match unit.trim().to_lowercase().as_str() {
            "gb" => Unit::Gb,
            "mb" => Unit::Mb,
            "kb" => Unit::Kb,
            _ => Unit::Mb,
        }
    }
}

pub struct OSData {
    pub host: String,
    pub user: String,
    pub os: String,
    pub shell: String,
    pub mem: String,
    pub mem_unit: Unit
}

impl OSData {
    pub fn new(unit: &str) -> Self {
        let host = get_hostname();
        let user = get_user();
        let os = get_os();
        let shell = get_shell();
        let mem_unit = Unit::get(unit);
        let mem = get_mem(&mem_unit);
        OSData { host, user, os, shell, mem, mem_unit }
    }
}

fn get_user() -> String {
    env::var("USER")
        .unwrap_or_else(|_| "Unknown".to_string())
}

fn get_hostname() -> String {
    match read("/etc/hostname") {
        Ok(content) => content.trim().to_string(),
        Err(_) => "Unknown".to_string()
    }
}

fn get_shell() -> String {
    let full_shell = env::var("SHELL")
        .unwrap_or_else(|_| "Unknown".to_string());
    let shell_array = full_shell.split("/");
    match shell_array.last() {
        Some(shell) => shell.to_string(),
        None => "Unknown".to_string()
    }
}


fn get_os() -> String {
    match read("/etc/os-release") {
        Ok(content) => {
            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    return line
                        .replace("PRETTY_NAME=", "")
                        .replace('"', "");
                }
            }
            "Unknown".to_string()
        }
        Err(_) => "Unknown".to_string(),
    }
}

fn get_mem(unit: &Unit) -> String {
    match read("/proc/meminfo") {
        Ok(content) => {
            let mut total = 0;
            let mut available = 0;

            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    total = line
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();
                } else if line.starts_with("MemAvailable:") {
                    available = line
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();
                }
            }

            let used = total - available;
            let total_and_used = match unit {
                Unit::Kb => (total, used, "KB"),
                Unit::Mb => (total / 1024, used / 1024, "MB"),
                Unit::Gb => (total / 1024 / 1024, used / 1024 / 1024, "GB"),
            };
            format!("{} {} / {} {}", total_and_used.0, total_and_used.2, total_and_used.1, total_and_used.2)
        }
        Err(_) => "Unknown".to_string()
    }
}
