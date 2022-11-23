use rand::seq::SliceRandom;
use rand::thread_rng;
use dialoguer::Input;
use regex::Regex;

const KEYS: &str = "qwertyuiopasdfghjklzxcvbnm";

struct Tube {
    height: usize,
    contents: [u8; 4],
}

impl Tube {
    fn new_empty() -> Self {
        Tube {
            height: 0,
            contents: [0, 0, 0, 0],
        }
    }

    fn top_color(&self) -> u8 {
        if self.height == 0 {
            panic!("no top color");
        }
        self.contents[self.height - 1]
    }

    fn is_empty(&self) -> bool {
        self.height == 0
    }

    fn is_full(&self) -> bool {
        self.height == 4
    }

    fn is_solved(&self) -> bool {
        self.is_full()
            && self.contents[0] == self.contents[1]
            && self.contents[0] == self.contents[2]
            && self.contents[0] == self.contents[3]
    }

    fn can_pour(&self, other: &Self) -> bool {
        !(self.is_empty()
            || self.is_solved()
            || other.is_solved()
            || other.is_full()
            || (!other.is_empty() && self.top_color() != other.top_color()))
    }
}

struct Rack {
    tubes: Vec<Tube>,
}

impl Rack {
    fn new(full_tube_count: u8, empty_tube_count: u8) -> Rack {
        let mut colors = vec![];
        let mut tubes = vec![];
        for color in 1..(full_tube_count+1) {
            let mut tube = Tube::new_empty();
            tube.height = 4;
            tubes.push(tube);
            for _ in 0..4 {
                colors.push(color);
            }
        }
        colors.shuffle(&mut thread_rng());
        for i in 0..colors.len() {
            let tube = i / 4;
            let place = i % 4;
            tubes[tube].contents[place] = colors[i];
        }
        for _ in 0..empty_tube_count {
            tubes.push(Tube::new_empty())
        }

        Rack { tubes }
    }

    fn print(&self) {
        print!("\x1b[2J\x1b[1;1H");
        for _ in 0..self.tubes.len() {
            print!("--");
        }
        println!("");
        for i in 0..4 {
            let level = 3 - i;
            for id in 0..self.tubes.len() {
                if self.tubes[id].contents[level] == 0 {
                    print!("  ");
                } else {
                    print!("\x1b[48;5;{}m \x1b[0m ", self.tubes[id].contents[level]+1);
                }
            }
            println!("");
        }
        for _ in 0..self.tubes.len() {
            print!("--");
        }
        println!("");
        for id in 0..self.tubes.len() {
            print!("{} ", KEYS.chars().into_iter().collect::<Vec<char>>()[id]);
        }
        println!("\n");
    }

    fn is_solved(&self) -> bool {
        for tube in self.tubes.iter() {
            if !(tube.is_solved() || tube.is_empty()) {
                return false;
            }
        }
        true
    }

    fn pour(&mut self, first: usize, second: usize) -> bool {
        if first == second || !self.tubes[first].can_pour(&self.tubes[second]) {
            return false;
        }
        while self.tubes[first].can_pour(&self.tubes[second])  {
            let top_color = self.tubes[first].top_color();
            {
                let mut_first_tube = &mut self.tubes[first];
                mut_first_tube.contents[mut_first_tube.height - 1] = 0;
                mut_first_tube.height -= 1;
            }
            let mut_second_tube = &mut self.tubes[second];
            mut_second_tube.contents[mut_second_tube.height] = top_color;
            mut_second_tube.height += 1;
            println!("")
        }
        true
    }
}

fn main() -> Result<(), std::io::Error> {
    let input_re = Regex::new(r"^[a-z][a-z]$").unwrap();
    let mut rack = Rack::new(8, 2);
    let mut tube_list = vec![];
    for i in 0..rack.tubes.len() {
        tube_list.push(format!("{}", i));
    }
    while !rack.is_solved() {
        let mut poured = false;
        while !poured {
            rack.print();
            let val = Input::new().with_prompt("Enter two tubes to transfer").validate_with(|result: &String| -> Result<(),&str> {
                if input_re.is_match(result) {
                    Ok(())
                } else {
                    Err("Must be two letters, no spaces or other characters")
                }
            }).interact()?;
            let split: Vec<char> = val.chars().collect();
            let first: usize = KEYS.find(split[0]).unwrap();
            let second: usize = KEYS.find(split[1]).unwrap();
            poured = rack.pour(first, second);
        }
    }
    rack.print();
    println!("Solved!");
    Ok(())
}
