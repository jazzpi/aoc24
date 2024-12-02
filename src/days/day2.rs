pub fn part1(reports: &Vec<Vec<i64>>) -> i64 {
    reports.iter().filter(is_safe).count().try_into().unwrap()
}

fn is_safe(report: &&Vec<i64>) -> bool {
    if report.len() == 1 {
        return true;
    }
    let inc = report[1] > report[0];
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if inc && diff < 0 || !inc && diff > 0 {
            return false;
        }
        let diff = diff.abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_part1() {
        let reports = input::read_input_lines_spaces("day2_ex");
        assert_eq!(part1(&reports), 2);

        if let Some((input_filename, output)) = input::check_testcase(2, 1) {
            let reports = input::read_input_lines_spaces(input_filename.as_str());
            assert_eq!(part1(&reports), output);
        }
    }
}
