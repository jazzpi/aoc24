use std::fs;

pub fn get_args() -> (String, u32) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input file> <part>", args[0]);
        std::process::exit(1);
    }
    (args[1].clone(), args[2].parse().unwrap())
}

pub fn read_input(filename: &str) -> String {
    let path = format!("input/{}", filename);
    fs::read_to_string(&path).expect(
        format!(
            "Failed to read {:?}/{}",
            std::env::current_dir().unwrap(),
            &path
        )
        .as_str(),
    )
}

pub fn read_input_2col(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = read_input(filename);
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    for line in contents.lines() {
        let mut split = line.split_whitespace();
        col1.push(split.next().unwrap().parse().unwrap());
        col2.push(split.next().unwrap().parse().unwrap());
    }
    (col1, col2)
}

pub fn read_input_lines_spaces(filename: &str) -> Vec<Vec<i64>> {
    let contents = read_input(filename);
    let mut lines = Vec::new();
    for line in contents.lines() {
        let mut nums = Vec::new();
        for num in line.split_whitespace() {
            nums.push(num.parse().unwrap());
        }
        lines.push(nums);
    }
    lines
}

pub fn check_testcase(day: u32, part: u32) -> Option<(String, i64)> {
    let input_filename = format!("day{}", day);
    let input_path = format!("input/{}", input_filename);
    let output_path = format!("results/day{}p{}", day, part);

    // Check if input file exists
    if std::path::Path::new(&input_path).exists() && std::path::Path::new(&output_path).exists() {
        let output = fs::read_to_string(output_path).expect("Failed to read output file");
        let output = output.trim().parse().unwrap();
        return Some((input_filename, output));
    } else {
        return None;
    }
}
