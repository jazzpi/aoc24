pub fn part1(col1: &mut Vec<i32>, col2: &mut Vec<i32>) -> i64 {
    assert_eq!(col1.len(), col2.len());
    // Sort each column
    col1.sort();
    col2.sort();
    // Find distances between each pair of elements
    let mut sum = 0;
    for i in 0..col1.len() {
        sum += (col1[i] - col2[i]).abs();
    }
    sum.into()
}

pub fn part2(col1: &mut Vec<i32>, col2: &mut Vec<i32>) -> i64 {
    assert_eq!(col1.len(), col2.len());
    // The naive solution (iterating through the right column for each element
    // in the left column) would be O(n^2), or at least O(n_unique * n). By
    // sorting each column we can then only iterate once through each, which is
    // O(n log n) + O(n) = O(n log n).
    col1.sort();
    col2.sort();

    let mut sum = 0;
    let mut i = 0;
    let mut j = 0;
    let mut count_right = 0;
    while i < col1.len() && j < col2.len() {
        if col2[j] < col1[i] {
            // We haven't reached the current left-element in the right column yet
            j += 1;
        } else if col2[j] > col1[i] {
            // We've passed the current left-element in the right column
            let val = col1[i];
            let i_orig = i;
            while i + 1 < col1.len() && col1[i + 1] == val {
                i += 1;
            }
            i += 1;
            sum += count_right * (i - i_orig) as i32 * val;
            count_right = 0;
        } else {
            // Count this element and move to the next element in the right column.
            count_right += 1;
            j += 1;
        }
    }

    sum.into()
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

    #[test]
    fn test_part2() {
        let (mut col1, mut col2) = input::read_input_2col("day1_ex");
        assert_eq!(part2(&mut col1, &mut col2), 31);

        if let Some((input_filename, expected)) = input::check_testcase(1, 2) {
            let (mut col1, mut col2) = input::read_input_2col(input_filename.as_str());
            assert_eq!(part2(&mut col1, &mut col2), expected);
        }
    }
}
