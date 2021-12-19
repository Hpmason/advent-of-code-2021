use advent_of_code_2021::{
    day1::{
        part1::{compare_depths, get_depth_increases},
        part2::calculate_depth_sums,
    },
    io::{print_header, read_input_file},
};

const PUZZLE_ONE_FILE: &str = "data/first.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input file
    println!("Reading input file {}", PUZZLE_ONE_FILE);
    let depths: Vec<u32> = read_input_file(PUZZLE_ONE_FILE)?;

    print_header(1, 1);
    part1(&depths);
    print_header(1, 2);
    part2(&depths);

    Ok(())
}

fn part1(depths: &Vec<u32>) {
    println!("Comparing {} depths from file", depths.len());
    let depth_diffs = compare_depths(&depths);
    // Find total number of increases in depth
    let total_increases = get_depth_increases(&depth_diffs);
    println!(
        "Total number of increases vs decreased: {}",
        total_increases
    );
}

fn part2(depths: &Vec<u32>) {
    // Get sums of 3s
    println!("Calculating sums of 3 long windows");
    let depth_sums = calculate_depth_sums(&depths);

    println!("Comparing Depths of {} windows", depth_sums.len());
    let depth_diffs = compare_depths(&depth_sums);

    println!("Calculating finding number of increases");
    let total_sum_increased = get_depth_increases(&depth_diffs);

    println!("Total number of increases: {}", total_sum_increased);
}
