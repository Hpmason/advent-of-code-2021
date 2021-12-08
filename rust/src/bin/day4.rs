use advent_of_code_2021::{io::{print_header}, day4::{parse_bingo_boards, BingoBoard, BingoInfo, run_through_bingo_game}};

const PUZZLE_DATA: &str = "data/fourth.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input file
    println!("Reading input file {}", PUZZLE_DATA);
    let data: BingoInfo = parse_bingo_boards(PUZZLE_DATA)?;
    // Part 1
    print_header(4, 1);
    part1(&data);
    // Part 2
    print_header(4, 2);
    part2(&data);
    Ok(())
}

fn part1(data: &BingoInfo) {
    // We are going to be mutating data
    if let Some(res) = run_through_bingo_game(data) {
        println!("Bingo!");
        println!("Someone won! Tallying the results now!");
        let final_sum = res.board.sum_of_unmarked();
        println!("Final sum = {}", final_sum);
        println!("Final score = {}", final_sum * res.last_num_called as u32);
    } else {
        println!("No one won. So sad :(");
    }
    // Do while loop
    
    
}

fn part2(data: &BingoInfo) {
    
}
