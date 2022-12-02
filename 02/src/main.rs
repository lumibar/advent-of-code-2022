use std::fs;


fn main() {
    let file_contents = fs::read_to_string("02/input").expect("File could not be read!");

    // loop over input file lines
    let score: u32 = file_contents.trim().split("\n").map(|x| {
        let mut score: u32 = 1;

        // get what position is in the input
        let y: Vec<&str> = x.trim().split(" ").collect();
        let oponent: i8 =  "ABC".find(y[0]).unwrap() as i8 ;
        let user: i8 = "XYZ".find(y[1]).unwrap() as i8;

        // Add loss draw win score
        score += user as u32 * 3;

        // calculate score based on what you have played
        score += [
            (oponent - 1).rem_euclid(3),
             oponent,
            (oponent + 1).rem_euclid(3)
        ][user as usize] as u32;

        score
    }).sum::<u32>();

    println!("02 : {}", score)
}
