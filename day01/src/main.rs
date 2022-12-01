use std::path::Path;
use std::fs::File;
use std::io::Read;

fn main() {
    let input = read_input("input");

    let elves: Vec<&str> = input.split("\n\n").collect();

    let max: u32 = elves.iter().map(|inventory| inventory.split('\n')
        .map(|food| food.parse::<u32>().unwrap()).sum())
        .max().unwrap();

    println!("Part 1: {}", max);

    let mut elf_sums: Vec<u32> = elves.iter().map(|inventory| inventory.split('\n')
    .map(|food| food.parse::<u32>().unwrap()).sum()).collect();

    elf_sums.sort();

    let sum_top_3: u32 = elf_sums[elf_sums.len()-3..].iter().sum();

    println!("Part 2: {sum_top_3}")

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
