use std::{path::Path, fs::File, io::Read, str::FromStr};

pub fn print_header(day: usize, part: usize) {
    println!("");
    println!("=================================");
    println!("= Advent of Code Day {:02} Part {:02} =", day, part);
    println!("=================================");
    println!("");
}


/// Expects path to a file with elements seperated by only new lines
pub fn read_input_file<T: FromStr>(file_path: impl AsRef<Path>) -> Result<Vec<T>, std::io::Error>{
    // Open file and read to string
    let mut f = File::open(file_path)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    // Lines to a Vec of elements of type T (Flattening parse result)
    let vec = data
        .lines()
        .map(|line| {
            line.parse::<T>()
        })
        .flat_map(|t| t)
        .collect();
    // Return vec of T
    Ok(vec)
}