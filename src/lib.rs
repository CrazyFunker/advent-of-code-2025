use std::fs;
use std::io;

pub fn read_input(day: u8) -> io::Result<String> {
    fs::read_to_string(format!("input/day{:02}.txt", day))
}
