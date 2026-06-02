// Names scores
// https://projecteuler.net/problem=22

use pe_lib::read_file_to_string;

fn name_value(name: &str) -> u32 {
    name.chars().map(|c| c as u32 - 'A' as u32 + 1).sum()
}

fn score_at(index: usize, names: &Vec<String>) -> u32 {
    (index + 1) as u32 * name_value(&names[index])
}

fn all_name_score(names: &Vec<String>) -> u32 {
    let mut sum = 0;
    for i in 0..names.len() {
        sum += score_at(i, names);
    }
    sum
}

fn solve() -> u32 {
    let mut names: Vec<String> = read_file_to_string("./data/0022_names.txt")
        .expect("failed to read ./data/0022_names.txt")
        .split(',')
        .map(|s| s.trim_matches('"').to_string())
        .collect();
    names.sort();
    all_name_score(&names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_value_simple() {
        // A=1, B=2, ..., Z=26
        assert_eq!(name_value("A"), 1);
        assert_eq!(name_value("Z"), 26);
        assert_eq!(name_value("AB"), 3); // 1 + 2
    }

    #[test]
    fn test_name_value_colin() {
        // COLIN = 3+15+12+9+14 = 53
        assert_eq!(name_value("COLIN"), 53);
    }

    #[test]
    fn test_score_at_position() {
        let names = vec!["COLIN".to_string()];
        // Position 0 (1-indexed as 1), value 53, score = 1*53 = 53
        assert_eq!(score_at(0, &names), 53);
    }

    #[test]
    fn test_all_name_score() {
        let names = vec!["ALICE".to_string(), "BOB".to_string()];
        let result = all_name_score(&names);
        // ALICE = 1+12+9+3+5 = 30, score = 1*30 = 30
        // BOB = 2+15+2 = 19, score = 2*19 = 38
        // total = 30 + 38 = 68
        assert_eq!(result, 68);
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
