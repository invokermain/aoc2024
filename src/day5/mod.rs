use itertools::Itertools;
use std::collections::HashSet;

struct PageUpdates {
    rules: HashSet<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

impl PageUpdates {
    fn validate(&self) -> usize {
        let mut sum = 0;
        for update in &self.updates {
            if self.is_update_valid(update) {
                sum += update[update.len().div_euclid(2)];
            }
        }
        sum
    }

    fn fix(&self) -> usize {
        let mut sum = 0;
        for update in &self.updates {
            if !self.is_update_valid(update) {
                let fixed = self.sort_update(update);
                sum += fixed[fixed.len().div_euclid(2)];
            }
        }
        sum
    }

    fn is_update_valid(&self, update: &[usize]) -> bool {
        !update
            .iter()
            .tuple_combinations()
            .any(|(&l, &r)| self.rules.contains(&(r, l)))
    }

    fn sort_update(&self, update: &[usize]) -> Vec<usize> {
        let mut sorted = update.to_owned();
        let mut swap_in_pass = true;
        while swap_in_pass {
            swap_in_pass = false;
            for idx in (1..sorted.len()).rev() {
                let r = sorted[idx];
                let l = sorted[idx - 1];
                if self.rules.contains(&(r, l)) {
                    sorted.swap(idx, idx - 1);
                    swap_in_pass = true;
                }
            }
        }
        sorted
    }
}

fn parse(input: &str) -> PageUpdates {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: HashSet<(usize, usize)> = rules
        .split_whitespace()
        .map(|line| line.split_once('|').unwrap())
        .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
        .collect();
    let updates = updates
        .split_whitespace()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    PageUpdates { rules, updates }
}

#[cfg(test)]
mod tests {
    use crate::day5::parse;
    use crate::utils::{load_file, load_input_for_day};

    #[test]
    fn test_parse() {
        let contents = load_file(5, "input_test.txt");
        let page_updates = parse(contents.as_str());
        assert!(page_updates.rules.contains(&(47, 53)));
        assert!(page_updates.rules.contains(&(53, 13)));
        assert_eq!(
            page_updates.updates.first(),
            Some(vec![75, 47, 61, 53, 29]).as_ref()
        );
        assert_eq!(
            page_updates.updates.last(),
            Some(vec![97, 13, 75, 29, 47]).as_ref()
        );
    }

    #[test]
    fn test_example() {
        let contents = load_file(5, "input_test.txt");
        let page_updates = parse(contents.as_str());
        assert_eq!(page_updates.validate(), 143);
    }

    #[test]
    fn test_example_part_2() {
        let contents = load_file(5, "input_test.txt");
        let page_updates = parse(contents.as_str());
        assert_eq!(page_updates.fix(), 123);
    }

    #[test]
    fn test_example_sort() {
        let contents = load_file(5, "input_test.txt");
        let page_updates = parse(contents.as_str());
        assert_eq!(
            page_updates.sort_update(&vec![75, 97, 47, 61, 53]),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(
            page_updates.sort_update(&vec![61, 13, 29]),
            vec![61, 29, 13]
        );
        assert_eq!(
            page_updates.sort_update(&vec![97, 13, 75, 29, 47]),
            vec![97, 75, 47, 29, 13]
        );
    }

    #[test]
    fn calculate_pt_1() {
        let contents = load_input_for_day(5);
        let page_updates = parse(contents.as_str());
        assert_eq!(page_updates.validate(), 143);
    }

    #[test]
    fn calculate_pt_2() {
        let contents = load_input_for_day(5);
        let page_updates = parse(contents.as_str());
        assert_eq!(page_updates.fix(), 4130);
    }
}
