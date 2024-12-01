pub fn part1(col1: &mut Vec<i32>, col2: &mut Vec<i32>) -> i32 {
    // Sort each column
    col1.sort();
    col2.sort();
    assert_eq!(col1.len(), col2.len());
    // Find distances between each pair of elements
    let mut sum = 0;
    for i in 0..col1.len() {
        sum += (col1[i] - col2[i]).abs();
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::input;

    use super::*;

    #[test]
    fn test_part1() {
        let (mut col1, mut col2) = input::read_input_2col("day1_ex");
        assert_eq!(part1(&mut col1, &mut col2), 11);

        if let Some((input_filename, expected)) = input::check_testcase(1, 1) {
            let (mut col1, mut col2) = input::read_input_2col(input_filename.as_str());
            assert_eq!(part1(&mut col1, &mut col2), expected);
        }
    }
}
