use std::{
    env,
    fs::{self},
};

use addr2line::fallible_iterator::FallibleIterator;

fn main() {
    println!("linedump. Usage: [address] [ELF path]");
    let args: Vec<String> = env::args().collect();
    let pc_str = &args[1];
    let input_path = &args[2];
    let bin_data = fs::read(input_path).unwrap();
    let parsed = addr2line::object::read::File::parse(bin_data.as_slice()).unwrap();
    let ctx = addr2line::Context::new(&parsed).unwrap();
    let loc = ctx.find_location(str_to_num(pc_str.as_str())).unwrap();
    match loc {
        Some(location) => {
            println!(
                "location: {} {} {}",
                location.file.unwrap(),
                location.line.unwrap(),
                location.column.unwrap()
            );
            let frames = ctx.find_frames(str_to_num(pc_str.as_str())).unwrap();
            for frame in frames.iterator().flatten() {
                let func = frame.function.unwrap();
                let name = String::from_utf8(func.name.to_vec()).unwrap();
                println!("function: {}, {}", name.as_str(), func.demangle().unwrap());
            }
        }
        None => {
            println!("line not found");
        }
    }
}

fn str_to_num(str: &str) -> u64 {
    let bytes = str.as_bytes();
    let mut result: i128 = 0;
    (0..bytes.len()).for_each(|i| {
        let b = bytes[i] as i128;
        let dec = 10_i128.pow(str.len() as u32 - 1 - i as u32);
        result += (b - 48) * dec;
    });
    result as u64
}
