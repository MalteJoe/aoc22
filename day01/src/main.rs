use std::path::Path;
use std::fs::File;
use std::io::Read;

fn main() {
    let input = read_input("input");

    let elves: Vec<&str> = input.split("\n\n").collect();

    println!("elves[0]:\n{:#?}", elves[0]);
    println!("elves[0].split:\n{:#?}", elves[0].split('\n').collect::<Vec<&str>>());
    println!("elves[0].sum:\n{:#?}", elves[0].split('\n').map(|food| food.parse::<u32>().unwrap()).sum::<u32>());

    let max: u32 = elves.iter().map(|inventory| inventory.split('\n')
        .map(|food| food.parse::<u32>().unwrap()).sum())
        .max().unwrap();

    println!("Day01: {}", max);
}

fn read_input(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {},
    }
    
    return s;
}
