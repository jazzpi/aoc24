use aoc24::{days::day1, input};

fn main() {
    let fname = input::get_input_filename();
    let (mut col1, mut col2) = input::read_input_2col(fname.as_str());

    println!("Sum of distances:\n{}", day1::part1(&mut col1, &mut col2));
}
