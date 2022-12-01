use derive_more::{Deref, DerefMut};
use dialoguer::Input;
use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;

#[derive(Clone, Debug, Deref, DerefMut)]
struct Tube(Vec<u8>);

impl Tube {
    fn new_empty() -> Self {
        Tube(Vec::with_capacity(4))
    }

    fn is_solved(&self) -> bool {
        self.len() == 4 && self.iter().all(|&i| i == self[0])
    }

    fn can_pour(&self, other: &Self) -> bool {
        !(self.is_empty()
            || self.is_solved()
            || other.is_solved()
            || other.len() == 4
            || (!other.is_empty() && self.last().unwrap() != other.last().unwrap()))
    }
}

#[derive(Debug, Deref, DerefMut)]
struct Rack(Vec<Tube>);

impl Rack {
    fn new(full_tube_count: usize, empty_tube_count: usize) -> Rack {
        let mut colors = Vec::with_capacity(full_tube_count * 4);
        let mut tubes = vec![Tube::new_empty(); full_tube_count + empty_tube_count];
        for color in 1..(full_tube_count + 1) {
            colors.append(&mut vec![color; 4]);
        }
        colors.shuffle(&mut thread_rng());
        for (i, color) in colors.iter().enumerate() {
            tubes[i / 4].push(*color as u8);
        }

        Rack(tubes)
    }

    fn print(&self, keys: &str) {
        let div = "--".repeat(self.len());
        println!("\x1b[2J\x1b[1;1H{}", div);
        for level in (0..4).rev() {
            for tube in self.iter() {
                let color = tube.get(level).map_or(0, |v| v + 1);
                print!("\x1b[48;5;{}m \x1b[0m ", color);
            }
            println!();
        }
        println!("{}", div);
        println!("{}", keys.split("").collect::<Vec<_>>().join(" ").trim());
    }

    fn is_solved(&self) -> bool {
        self.iter().all(|t| t.is_solved() || t.is_empty())
    }

    fn pour(&mut self, first: usize, second: usize) {
        while first != second && self[first].can_pour(&self[second]) {
            let color = self[first].pop().unwrap();
            self[second].push(color);
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let keys = "qwertyuiop";
    let input_re = Regex::new(&*format!("^[{}][{}]$", keys, keys)).unwrap();
    let mut rack = Rack::new(8, 2);

    while !rack.is_solved() {
        rack.print(keys);
        let val = Input::new()
            .with_prompt("Enter 2 tubes to transfer")
            .validate_with(|text: &String| -> Result<(), &str> {
                input_re
                    .is_match(text)
                    .then_some(())
                    .ok_or("Must be exactly 2 letters")
            })
            .interact()?;
        let split: Vec<usize> = val.chars().map(|c| keys.find(c).unwrap()).collect();
        rack.pour(split[0], split[1]);
    }
    rack.print(keys);
    println!("Solved!");
    Ok(())
}
