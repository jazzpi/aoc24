use aoc24::{days::day1, input};

fn main() {
    let (fname, part) = input::get_args();
    let (mut col1, mut col2) = input::read_input_2col(fname.as_str());

    if part == 1 {
        println!("Sum of distances:\n{}", day1::part1(&mut col1, &mut col2));
    } else {
        println!("Similarity score:\n{}", day1::part2(&mut col1, &mut col2));
    }
}
