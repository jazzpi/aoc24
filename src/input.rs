use std::fs;

pub fn get_input_filename() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }
    args[1].clone()
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

pub fn check_testcase(day: u32, part: u32) -> Option<(String, i32)> {
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