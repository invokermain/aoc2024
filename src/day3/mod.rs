use regex::Regex;

const RE_DO: &str = r"mul\(\d+,\d+\)";

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    Mul(usize, usize),
    Do,
    DoNot,
}

trait OperationExtractor {
    fn compile_regex(&self) -> Regex;

    fn extract(&self, s: &str) -> Operation;
}

pub struct MultiplyExtractor;

impl OperationExtractor for MultiplyExtractor {
    fn compile_regex(&self) -> Regex {
        Regex::new(r"mul\(\d+,\d+\)").unwrap()
    }

    fn extract(&self, s: &str) -> Operation {
        let (left, right) = s
            .strip_prefix("mul(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split_once(",")
            .unwrap();
        Operation::Mul(left.parse().unwrap(), right.parse().unwrap())
    }
}

pub struct DoExtractor;

impl OperationExtractor for DoExtractor {
    fn compile_regex(&self) -> Regex {
        Regex::new(r"do\(\)").unwrap()
    }

    fn extract(&self, _s: &str) -> Operation {
        Operation::Do
    }
}
pub struct DontExtractor;

impl OperationExtractor for DontExtractor {
    fn compile_regex(&self) -> Regex {
        Regex::new(r"don't\(\)").unwrap()
    }

    fn extract(&self, _s: &str) -> Operation {
        Operation::DoNot
    }
}

fn extract(input: &str) -> Vec<Operation> {
    let extractors: [Box<dyn OperationExtractor>; 3] = [
        Box::new(MultiplyExtractor),
        Box::new(DoExtractor),
        Box::new(DontExtractor),
    ];
    let mut unsorted_operators: Vec<(usize, Operation)> = extractors
        .iter()
        .flat_map(|ext| {
            ext.compile_regex()
                .find_iter(input)
                .map(|m| (m.start(), ext.extract(m.as_str())))
                .collect::<Vec<(usize, Operation)>>()
        })
        .collect();
    unsorted_operators.sort_by(|(a, _), (b, _)| a.cmp(b));
    unsorted_operators.into_iter().map(|(_, op)| op).collect()
}

fn compute(input: &Vec<Operation>) -> usize {
    let mut sum = 0;
    let mut enabled = true;

    for op in input {
        match op {
            Operation::Mul(l, r) => {
                if enabled {
                    sum += l * r;
                }
            }
            Operation::Do => {
                enabled = true;
            }
            Operation::DoNot => {
                enabled = false;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load_input_for_day;

    #[test]
    fn test_cleaning() {
        let out =
            extract(&r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(
            out,
            vec![
                Operation::Mul(2, 4),
                Operation::Mul(5, 5),
                Operation::Mul(11, 8),
                Operation::Mul(8, 5),
            ]
        );
    }

    #[test]
    fn test_cleaning_2() {
        let out =
            extract(&r"mul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(
            out,
            vec![
                Operation::Mul(2, 4),
                Operation::DoNot,
                Operation::Mul(5, 5),
                Operation::Mul(11, 8),
                Operation::Do,
                Operation::Mul(8, 5),
            ]
        );
    }

    #[test]
    fn test_compute() {
        let out = compute(&vec![
            Operation::Mul(2, 4),
            Operation::DoNot,
            Operation::Mul(5, 5),
            Operation::Mul(11, 8),
            Operation::Do,
            Operation::Mul(8, 5),
        ]);
        assert_eq!(out, 48);
    }

    #[test]
    fn calculate_pt_1() {
        let file_contents = load_input_for_day(3);
        let extracted = extract(file_contents.as_str())
            .into_iter()
            // part 1 just cares about Mul operator
            .filter(|op| match op {
                Operation::Mul(_, _) => true,
                _ => false,
            })
            .collect();
        let total = compute(&extracted);
        println!("extracted: {extracted:?}");
        println!("total: {total:?}");
        assert_eq!(total, 153469856)
    }

    #[test]
    fn calculate_pt_2() {
        let file_contents = load_input_for_day(3);
        let extracted = extract(file_contents.as_str());
        let total = compute(&extracted);
        println!("extracted: {extracted:?}");
        println!("total: {total:?}");
        assert_eq!(total, 77055967)
    }
}
