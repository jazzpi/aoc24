use aoc24::{
    days::{day1, day2},
    input,
};

fn usage() {
    eprintln!("Usage: aoc24 <day> <part> <input file>");
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        usage();
    }
    let day: i32 = args[1].parse().unwrap();
    let part: i32 = args[2].parse().unwrap();
    let fname = args[3].clone();
    if (day < 1 || day > 25) || (part < 1 || part > 2) {
        usage();
    }

    let result: i64 = match day {
        1 => {
            let (mut col1, mut col2) = input::read_input_2col(fname.as_str());
            if part == 1 {
                day1::part1(&mut col1, &mut col2)
            } else {
                day1::part2(&mut col1, &mut col2)
            }
        }
        2 => {
            let reports = input::read_input_lines_spaces(fname.as_str());
            if part == 1 {
                day2::part1(&reports)
            } else {
                eprintln!("Day 2 part 2 not implemented");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Day {} not implemented", day);
            std::process::exit(1);
        }
    };

    println!("Result:\n{result}");
}
