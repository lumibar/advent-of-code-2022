use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input").expect("File could not be read!");

    // List of total calories for each elf
    let mut elf_calories: Vec<i32> = file_contents.split("\n\n").map(
            |x: &str|
            x.split("\n")
            .map(|y| y.parse::<i32>().unwrap_or(0))
            .sum::<i32>()
    ).collect();

    // Get the amount of calories the top 3 elves have
    elf_calories.sort();
    elf_calories.reverse();
    elf_calories.truncate(3);

    // Find the total calories they have between them
    let total = elf_calories.iter().sum::<i32>();

    println!("{}", total)
}
