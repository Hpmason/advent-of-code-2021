#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DepthDifference {
    Decrease = -1,
    Equal = 0,
    Increase = 1,
}

pub fn compare_depth(depth_1: u32, depth_2: u32) -> DepthDifference {
    // If next depth 1 is more than depth 2, the depth has decreased
    if depth_1 > depth_2 {
        DepthDifference::Decrease
    } else if depth_2 > depth_1 {
        DepthDifference::Increase
    } else {
        DepthDifference::Equal
    }
}
pub mod part1 {
    use super::DepthDifference;

    pub fn compare_depths(depths: &Vec<u32>) -> Vec<DepthDifference> {
        let depth_diffs: Vec<DepthDifference> = depths
            .windows(2)
            .map(|win| super::compare_depth(win[0], win[1]))
            .collect();
        depth_diffs
    }
    pub fn get_depth_difference(diffs: &Vec<DepthDifference>) -> i32 {
        diffs.iter().map(|&d| d as i32).sum()
    }
    pub fn get_depth_increases(diffs: &Vec<DepthDifference>) -> usize {
        diffs
            .into_iter()
            .filter(|&&d| d == DepthDifference::Increase)
            .collect::<Vec<&DepthDifference>>()
            .len()
    }
}

pub mod part2 {
    pub fn calculate_depth_sums(depths: &Vec<u32>) -> Vec<u32> {
        let depth_sums: Vec<u32> = depths
            .windows(3)
            .map(|win| win[0] + win[1] + win[2])
            .collect();
        depth_sums
    }
}
