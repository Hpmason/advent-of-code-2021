#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DepthDifference {
    Decrease = -1,
    Equal = 0,
    Increase = 1,
}

pub fn compare_depth(depth_1: u32, depth_2: u32) -> DepthDifference {
    // If next depth 1 is more than depth 2, the depth has decreased
    match depth_1.cmp(&depth_2) {
        std::cmp::Ordering::Less => DepthDifference::Increase,
        std::cmp::Ordering::Equal => DepthDifference::Equal,
        std::cmp::Ordering::Greater => DepthDifference::Decrease,
    }
}
pub mod part1 {
    use super::DepthDifference;

    pub fn compare_depths(depths: &[u32]) -> Vec<DepthDifference> {
        let depth_diffs: Vec<DepthDifference> = depths
            .windows(2)
            .map(|win| super::compare_depth(win[0], win[1]))
            .collect();
        depth_diffs
    }
    pub fn get_depth_difference(diffs: &[DepthDifference]) -> i32 {
        diffs.iter().map(|&d| d as i32).sum()
    }
    pub fn get_depth_increases(diffs: &[DepthDifference]) -> usize {
        diffs
            .iter()
            .filter(|&&d| d == DepthDifference::Increase)
            .count()
    }
}

pub mod part2 {
    pub fn calculate_depth_sums(depths: &[u32]) -> Vec<u32> {
        let depth_sums: Vec<u32> = depths
            .windows(3)
            .map(|win| win[0] + win[1] + win[2])
            .collect();
        depth_sums
    }
}
