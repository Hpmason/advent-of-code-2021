use advent_of_code_2021::{
    day5::{
        *,
        part1::*,
        part2::*,
    },
    io::{print_header, read_input_file},
};

const PUZZLE_ONE_FILE: &str = "data/fifth.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input file
    println!("Reading input file {}", PUZZLE_ONE_FILE);
    let data: Vec<LineSegment> = read_input_file(PUZZLE_ONE_FILE)?;

    print_header(1, 1);
    part1(&data);
    print_header(1, 2);
    part2(&data);

    Ok(())
}

fn part1(data: &Vec<LineSegment>) {
    let mut grid = Grid::new(1000);
    data.iter().for_each(|line| grid.apply_line(line));
    // println!("{}", grid);
    println!("Sum overlapping: {}", grid.count_overlaps());
}

fn part2(data: &Vec<LineSegment>) {
    // 
    println!("");

}
