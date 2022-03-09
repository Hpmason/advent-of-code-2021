use std::{fs::File, io::Read, path::Path};

type LanternFish = u64;

#[derive(Debug, Clone)]
pub struct LanternFishTracker([LanternFish; 9]);

impl LanternFishTracker {
    pub fn new() -> Self {
        Self([0; 9])
    }
    pub fn from_file(file_path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        // Open file and read to string
        let mut f = File::open(file_path)?;
        let mut data = String::new();
        f.read_to_string(&mut data)?;
        // Lines to a Vec of elements of type T (Flattening parse result)
        let mut tracker = Self::new();
        for fish in data.split(",") {
            let i = fish.parse::<usize>().unwrap();
            tracker.0[i] += 1;
        }
        // Return vec of T
        Ok(tracker)
    }

    /// Mutates array in place
    pub fn step_number_of_days(&mut self, days: u32) {
        for _ in 0..days {
            // println!("Step: {:?}", self.0);
            self.step_one_day();
        }
    }

    /// Modifies Vec in place
    pub fn step_one_day(&mut self) {
        // Get number at index 0
        let new_fish = self.0.first().unwrap().clone();
        // Shift all values to the left, except for the first
        for i in 0..(self.0.len() - 1) {
            self.0[i] = self.0[i + 1];
        }
        // New duplicated fish start with a countdown of 8
        self.0[8] = new_fish;
        // Original fish that duplicated reset back to 6 
        self.0[6] += new_fish;
    }
    pub fn num_fish(self) -> LanternFish {
        self.0.clone().into_iter().sum()
    }
}

pub mod part1 {}
pub mod part2 {}
