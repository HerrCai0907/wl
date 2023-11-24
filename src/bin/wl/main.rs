use std::{env, process};
use wl::run_file;

fn main() {
    // 获取程序的参数列表
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_file(),
        _ => process::exit(64),
    }
}
