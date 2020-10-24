extern crate sys_info;

use sys_info::*;
use std::env::args;

fn main() {
    let args = args().collect::<Vec<String>>();
    let unit_mb = args.iter().any(|s| s.to_lowercase() == "mb");
    let unit_gb = args.iter().any(|s| s.to_lowercase() == "gb");

    println!(
        "OS: {} {}",
        os_type().unwrap_or_unknown_message(),
        os_release().unwrap_or_unknown_message()
    );

    println!(
        "CPU: {} Cores, {} MHz",
        cpu_num().unwrap_or_unknown_message(),
        cpu_speed().unwrap_or_unknown_message()
    );

    if let Some(load) = loadavg().unwrap_safe() {
        println!("Load Avg: {} {} {}", load.one, load.five, load.fifteen);
    } else {
        println!("Load Avg: unknown");
    }

    if let Some(mem) = mem_info().unwrap_safe() {
        if unit_gb {
            println!(
                "Mem: total {:.2} GB, free {:.2} GB, avail {:.2} GB, buffers {:.2} GB, cached {:.2} GB",
                mem.total as f64 / 1024.0 / 1024.0,
                mem.free as f64 / 1024.0 / 1024.0,
                mem.avail as f64 / 1024.0 / 1024.0,
                mem.buffers as f64 / 1024.0 / 1024.0,
                mem.cached as f64 / 1024.0 / 1024.0
            );
        } else if unit_mb {
            println!(
                "Mem: total {:.2} MB, free {:.2} MB, avail {:.2} MB, buffers {:.2} MB, cached {:.2} MB",
                mem.total as f64 / 1024.0,
                mem.free as f64 / 1024.0,
                mem.avail as f64 / 1024.0,
                mem.buffers as f64 / 1024.0,
                mem.cached as f64 / 1024.0
            );
        } else {
            println!(
                "Mem: total {} KB, free {} KB, avail {} KB, buffers {} KB, cached {} KB",
                mem.total, mem.free, mem.avail, mem.buffers, mem.cached
            );
        }
    } else {
        println!("Mem: unknown");
    }
}

trait SafeInfoDisplay<T> {
    fn unwrap_or_unknown_message(&self) -> String;
}

trait SafeInfo<'a, T, K> {
    fn unwrap_safe(&'a self) -> Option<&'a K>;
}

impl SafeInfoDisplay<Result<String, Error>> for Result<String, Error> {
    fn unwrap_or_unknown_message(&self) -> String {
        convert(self)
    }
}

impl SafeInfoDisplay<Result<u32, Error>> for Result<u32, Error> {
    fn unwrap_or_unknown_message(&self) -> String {
        convert(self)
    }
}

impl SafeInfoDisplay<Result<u64, Error>> for Result<u64, Error> {
    fn unwrap_or_unknown_message(&self) -> String {
        convert(self)
    }
}

fn convert<T>(value: &Result<T, Error>) -> String
where
    T: std::fmt::Display,
{
    if let Ok(n) = value {
        n.to_string()
    } else {
        "unknown".to_string()
    }
}

impl<'a> SafeInfo<'a, Result<LoadAvg, Error>, LoadAvg> for Result<LoadAvg, Error> {
    fn unwrap_safe(&'a self) -> Option<&'a LoadAvg> {
        convert_type(&self)
    }
}

impl<'a> SafeInfo<'a, Result<MemInfo, Error>, MemInfo> for Result<MemInfo, Error> {
    fn unwrap_safe(&'a self) -> Option<&'a MemInfo> {
        convert_type(&self)
    }
}

fn convert_type<T>(value: &Result<T, Error>) -> Option<&T> {
    match value {
        Ok(obj) => Some(obj),
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}
