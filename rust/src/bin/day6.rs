use advent_of_code_2021::{
    day6::*,
    io::print_header,
};

const PUZZLE_SIX_FILE: &str = "data/sixth.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input file
    println!("Reading input file {}", PUZZLE_SIX_FILE);
    let tracker = LanternFishTracker::from_file(PUZZLE_SIX_FILE)?;

    print_header(6, 1);
    part1(tracker.clone());
    print_header(6, 2);
    part2(tracker.clone());

    Ok(())
}

fn part1(mut data: LanternFishTracker) {

    // println!("Fish: {:?}", copy);
    data.step_number_of_days(80);
    // println!("Fish: {:?}", copy);
    println!("Number of fish: {}", data.num_fish());
}

fn part2(mut data: LanternFishTracker) {
    // println!("Fish: {:?}", copy);
    data.step_number_of_days(256);
    // println!("Fish: {:?}", copy);
    println!("Number of fish: {}", data.num_fish());

}
