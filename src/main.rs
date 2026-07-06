
use crate::info::OSData;

pub mod info;
pub mod utils;

fn main() {
    let data = OSData::new();
    println!(r#"
{}@{}
OS: {}
Shell: {}
Mem: {}
    "#,
data.host, data.user, data.os, data.shell, data.mem);
}

