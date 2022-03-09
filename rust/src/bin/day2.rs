use advent_of_code_2021::{
    day2::{Command, Direction, Position, Submarine},
    io::{print_header, read_input_file},
};

const PUZZLE_DATA: &str = "data/second.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input file
    println!("Reading input file {}", PUZZLE_DATA);
    let commands: Vec<Command> = read_input_file(PUZZLE_DATA)?;
    // Part 1
    print_header(2, 1);
    part1(&commands);
    // Part 2
    print_header(2, 2);
    part2(&commands);
    Ok(())
}

fn part1(commands: &[Command]) {
    let mut position = Position::default();
    for command in commands {
        match command.direction {
            Direction::Forward => position.x += command.magnitude,
            Direction::Up => position.y -= command.magnitude,
            Direction::Down => position.y += command.magnitude,
        }
    }
    println!("Final position: (x: {}, y: {})", position.x, position.y);
    println!(
        "{} * {} = {}",
        position.x,
        position.y,
        position.x * position.y
    );
}

fn part2(commands: &[Command]) {
    let mut sub = Submarine::default();
    for command in commands {
        match command.direction {
            Direction::Forward => sub.forward(command.magnitude),
            Direction::Up => sub.up(command.magnitude),
            Direction::Down => sub.down(command.magnitude),
        }
    }
    println!(
        "Final position: (x: {}, y: {})",
        sub.get_pos().x,
        sub.get_pos().y
    );
    println!(
        "{} * {} = {}",
        sub.get_pos().x,
        sub.get_pos().y,
        sub.get_pos().x * sub.get_pos().y
    );
}
