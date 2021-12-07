use advent_of_code_2021::{day2::{Command, Position, Direction}, io::{print_header, read_input_file}};

const PUZZLE_DATA: &str = "data/second.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input file
    println!("Reading input file {}", PUZZLE_DATA);
    let commands: Vec<Command> = read_input_file(PUZZLE_DATA)?;
    print_header(2, 1);
    part1(&commands);
    Ok(())
}

fn part1(commands: &Vec<Command>) {
    let mut position = Position::new(0, 0);
    for command in commands {
        match command.direction {
            Direction::Forward => position.x += command.magnitude,
            Direction::Up => position.y -= command.magnitude,
            Direction::Down => position.y += command.magnitude,
        }
    }
    println!("Final position: (x: {}, y: {})", position.x, position.y);
    println!("{} * {} = {}", position.x, position.y, position.x * position.y);
}

fn part2(commands: &Vec<Command>) {
    
}
