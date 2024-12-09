fn parse_reports(file_contents: &str) -> Vec<Vec<i32>> {
    file_contents
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &[i32], dampener: bool) -> bool {
    let mut previous = report[0];
    let ascending = report[1] - report[0] > 0;
    for (current_index, &current) in report[1..].iter().enumerate() {
        let diff = current - previous;
        let is_valid = validate_level_change(ascending, diff);
        if !is_valid {
            return if dampener {
                dampener_routine(current_index, report)
            } else {
                false
            };
        }
        previous = current;
    }
    true
}

fn dampener_routine(current_index: usize, report: &[i32]) -> bool {
    let mut right_report = report.to_owned();
    right_report.remove(current_index + 1); // we start from index 1
    let mut middle_report = report.to_owned();
    middle_report.remove(current_index);
    let mut alternative_reports = vec![right_report, middle_report];
    if current_index >= 1 {
        let mut left_report = report.to_owned();
        left_report.remove(current_index - 1);
        alternative_reports.push(left_report);
    }
    alternative_reports
        .iter()
        .any(|report| is_safe(report, false))
}

fn validate_level_change(ascending: bool, diff: i32) -> bool {
    if ascending && diff <= 0 {
        return false;
    }
    if !ascending && diff >= 0 {
        return false;
    }
    if diff.abs() > 3 || diff.abs() < 1 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load_input_for_day;

    #[test]
    fn test_parse_reports() {
        let out = parse_reports(&"1 2 3\n4 5 6");
        assert_eq!(out[0], vec![1, 2, 3]);
        assert_eq!(out[1], vec![4, 5, 6]);
    }

    #[test]
    fn test_is_safe() {
        assert!(is_safe(&vec![7, 6, 4, 2, 1], false));
        assert!(!is_safe(&vec![1, 2, 7, 8, 9], false));
        assert!(!is_safe(&vec![9, 7, 6, 2, 1], false));
        assert!(!is_safe(&vec![1, 3, 2, 4, 5], false));
        assert!(!is_safe(&vec![8, 6, 4, 4, 1], false));
        assert!(is_safe(&vec![1, 3, 6, 7, 9], true));
    }

    #[test]
    fn test_is_safe_dampened() {
        // aoc test cases
        assert!(is_safe(&vec![7, 6, 4, 2, 1], true));
        assert!(!is_safe(&vec![1, 2, 7, 8, 9], true));
        assert!(!is_safe(&vec![9, 7, 6, 2, 1], true));
        assert!(is_safe(&vec![1, 3, 2, 4, 5], true));
        assert!(is_safe(&vec![8, 6, 4, 4, 1], true));
        assert!(is_safe(&vec![1, 3, 6, 7, 9], true));
        // custom
        assert!(is_safe(&vec![5, 1, 2, 3, 4], true));
        assert!(is_safe(&vec![1, 1, 2, 3, 4], true));
        assert!(!is_safe(&vec![1, 1, 1, 3, 4], true));
        assert!(is_safe(&vec![2, 1, 2, 3, 4], true));
    }

    #[test]
    fn calculate_pt_1() {
        let file_contents = load_input_for_day(2);
        let reports = parse_reports(file_contents.as_str());
        let total_safe: usize = reports
            .into_iter()
            .map(|report| is_safe(&report, false) as usize)
            .sum();
        println!("total safe reports is {total_safe:?}");
        assert_eq!(total_safe, 524)
    }

    #[test]
    fn calculate_pt_2() {
        let file_contents = load_input_for_day(2);
        let reports = parse_reports(file_contents.as_str());
        let total_safe: usize = reports
            .into_iter()
            .map(|report| is_safe(&report, true) as usize)
            .sum();
        println!("total safe reports is {total_safe:?}");
        assert_eq!(total_safe, 569)
    }
}
