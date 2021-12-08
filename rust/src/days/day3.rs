pub fn count_number_of_ones(arr_of_words: &Vec<String>) -> Vec<i32> {
    let size_of_word = arr_of_words[0].len();
    arr_of_words
        .iter()
        .map(|s| s.chars())
        // Keep track of how many 1s vs 0s per bit positions,
        //      incrementing by 1 for '1's and decrementing for '0's
        .fold(vec![0; size_of_word],|mut acc, chars| {
            // Loop through each position
            for (i, c) in chars.enumerate() {
                // If that bit is set, increment
                if c == '1' {
                    acc[i] += 1;
                }
                else {
                    acc[i] -= 1;
                }
            }
            acc
        })
}

pub fn get_oxygen_generator_rating(arr_of_words: &Vec<String>) -> u32 {
    let mut counts = count_number_of_ones(&arr_of_words);
    let mut arr_copy = arr_of_words.clone();
    let mut pos = 0;
    loop {
        arr_copy = arr_copy.into_iter().filter(|word| {
            // If 1 is more common and keep 1 if equal
            if counts[pos] >= 0 {
                word.chars().nth(pos) == Some('1')
            } else {
                word.chars().nth(pos) == Some('0')
            }
        })
        .collect();
        counts = count_number_of_ones(&arr_copy);
        if arr_copy.len() == 1{
            return u32::from_str_radix(&arr_copy[0], 2).unwrap();
        }
        pos += 1;
    }
}

pub fn get_co2_scrubber_rating(arr_of_words: &Vec<String>) -> u32 {
    let mut counts = count_number_of_ones(&arr_of_words);
    let mut arr_copy = arr_of_words.clone();
    let mut pos = 0;
    loop {
        arr_copy = arr_copy.into_iter().filter(|word| {
            // If 0 is more common and keep 0 if equal
            if counts[pos] >= 0 {
                word.chars().nth(pos) == Some('0')
            } else {
                word.chars().nth(pos) == Some('1')
            }
        })
        .collect();
        counts = count_number_of_ones(&arr_copy);
        if arr_copy.len() == 1{
            return u32::from_str_radix(&arr_copy[0], 2).unwrap();
        }
        pos += 1;
    }
}