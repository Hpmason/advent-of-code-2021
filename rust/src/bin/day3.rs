use advent_of_code_2021::{io::{print_header, read_input_file}, day3::{count_number_of_ones, get_co2_scrubber_rating, get_oxygen_generator_rating}};

const PUZZLE_DATA: &str = "data/third.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input file
    println!("Reading input file {}", PUZZLE_DATA);
    let data: Vec<String> = read_input_file(PUZZLE_DATA)?;

    // Part 1
    print_header(3, 1);
    part1(&data);
    // Part 2
    print_header(3, 2);
    part2(&data);
    Ok(())
}

fn part1(data: &Vec<String>) {
    let size_of_word = data[0].len();
    let bit_counts = count_number_of_ones(&data);
    println!("Bits: {:?}", bit_counts);
    // Calculate half of total length
    let mut gamma = 0;
    for (i, &val) in bit_counts.iter().enumerate() {
        // If 1 is most commmon (more than half) set that bit in gamma
        if val > 0 {
            gamma |= 1 << (size_of_word - i - 1);
        }
    }
    // Get mask of with all 1s in pos 0..size-1
    let xor_mask = (1 << size_of_word) - 1;
    // Flip all 5 bits to get least common bits
    let epsilon =  gamma ^ xor_mask;
    println!("Gamma = {:05b}", gamma);
    println!("Epsilon = {:05b}", epsilon);
    println!("Gamma * Epsilon = {}", gamma * epsilon);
}

fn part2(data: &Vec<String>) {
    let o2_rating = get_oxygen_generator_rating(&data);
    println!("O2 rating = {}", o2_rating);
    let co2_rating = get_co2_scrubber_rating(&data);
    println!("CO2 rating = {}", co2_rating);

    let life_support_rating = o2_rating * co2_rating;
    println!("Life Support Rating: {}", life_support_rating);
}
