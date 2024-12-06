use std::collections::HashMap;

fn parse_locations(file_contents: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut is_left = true;
    for location in file_contents.split_whitespace() {
        let location_id = location.parse::<u32>().unwrap();
        if is_left {
            left.push(location_id);
        } else {
            right.push(location_id);
        };
        is_left = !is_left;
    }
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
}

fn calculate_total_distance(locations: &(Vec<u32>, Vec<u32>)) -> usize {
    locations
        .0
        .iter()
        .zip(&locations.1)
        .map(|(&l, &r)| r.abs_diff(l) as usize)
        .sum()
}

fn calculate_similarity_score(locations: &(Vec<u32>, Vec<u32>)) -> usize {
    let right_counts = locations
        .1
        .iter()
        .fold(HashMap::<u32, usize>::new(), |mut acc, &x| {
            acc.entry(x).and_modify(|e| *e += 1).or_insert(1);
            acc
        });
    locations
        .0
        .iter()
        .map(|&x| x as usize)
        .fold(0, |acc: usize, x: usize| {
            acc + x * right_counts.get(&(x as u32)).unwrap_or(&0)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load_file_contents;

    #[test]
    fn test_parse_locations() {
        let out = parse_locations(&"123 654\n321 456");
        assert_eq!(out.0, vec![123, 4321]);
        assert_eq!(out.1, vec![456, 654]);
    }

    #[test]
    fn test_calculate_total_distance() {
        let locations = (vec![1, 2, 5], vec![1, 3, 4]);
        let total = calculate_total_distance(&locations);
        assert_eq!(total, 2);
    }

    #[test]
    fn test_calculate_similarity_score() {
        let locations = (vec![1, 2, 3, 3, 3, 4], vec![3, 3, 3, 4, 5, 9]);
        let similarity = calculate_similarity_score(&locations);
        assert_eq!(similarity, 31);
    }

    #[test]
    fn calculate_pt_1() {
        let file_contents = load_file_contents(1);
        let locations = parse_locations(file_contents.as_str());
        let total_distance = calculate_total_distance(&locations);
        println!("total distance is {total_distance:?}");
    }

    #[test]
    fn calculate_pt_2() {
        let file_contents = load_file_contents(1);
        let locations = parse_locations(file_contents.as_str());
        let total_similarity = calculate_similarity_score(&locations);
        println!("total similarity is {total_similarity:?}");
    }
}
