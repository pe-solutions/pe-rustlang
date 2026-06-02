use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn read_file_to_string(path: &str) -> io::Result<String> {
    std::fs::read_to_string(path)
}

pub fn read_csv_matrix(path: &str) -> io::Result<Vec<Vec<u32>>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut matrix = Vec::new();

    for line in reader.lines() {
        let row: Result<Vec<u32>, _> = line?
            .trim()
            .split(',')
            .map(|s| s.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)))
            .collect();
        matrix.push(row?);
    }

    Ok(matrix)
}

pub fn read_space_separated_matrix<T: FromStr>(path: &str) -> io::Result<Vec<Vec<T>>> {
    let content = std::fs::read_to_string(path)?;
    let matrix = content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<T>().ok())
                .collect::<Option<Vec<T>>>()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "parse error"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(matrix)
}

pub fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn create_test_file(content: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut path = std::env::temp_dir();
        path.push(format!("pe_lib_test_file_{}.txt", id));
        fs::write(&path, content).unwrap();
        path
    }

    fn create_csv_test_file(content: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut path = std::env::temp_dir();
        path.push(format!("pe_lib_test_csv_{}.csv", id));
        fs::write(&path, content).unwrap();
        path
    }

    fn create_space_separated_test_file(content: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut path = std::env::temp_dir();
        path.push(format!("pe_lib_test_space_{}.txt", id));
        fs::write(&path, content).unwrap();
        path
    }

    // read_file_to_string tests
    #[test]
    fn test_read_file_to_string_simple() {
        let path = create_test_file("Hello, World!");
        let content = read_file_to_string(path.to_str().unwrap()).unwrap();
        assert_eq!(content, "Hello, World!");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_read_file_to_string_multiline() {
        let content_str = "Line 1\nLine 2\nLine 3";
        let path = create_test_file(content_str);
        let content = read_file_to_string(path.to_str().unwrap()).unwrap();
        assert_eq!(content, content_str);
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_read_file_to_string_empty() {
        let path = create_test_file("");
        let content = read_file_to_string(path.to_str().unwrap()).unwrap();
        assert_eq!(content, "");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_read_file_to_string_nonexistent() {
        let result = read_file_to_string("/nonexistent/path/to/file.txt");
        assert!(result.is_err());
    }

    // read_csv_matrix tests
    #[test]
    fn test_read_csv_matrix_simple() {
        let content = "1,2,3\n4,5,6\n7,8,9";
        let path = create_csv_test_file(content);
        let matrix = read_csv_matrix(path.to_str().unwrap()).unwrap();
        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0], vec![1, 2, 3]);
        assert_eq!(matrix[1], vec![4, 5, 6]);
        assert_eq!(matrix[2], vec![7, 8, 9]);
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_read_csv_matrix_different_widths() {
        let content = "1,2\n3,4,5\n6";
        let path = create_csv_test_file(content);
        let matrix = read_csv_matrix(path.to_str().unwrap()).unwrap();
        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0], vec![1, 2]);
        assert_eq!(matrix[1], vec![3, 4, 5]);
        assert_eq!(matrix[2], vec![6]);
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_read_csv_matrix_with_spaces() {
        let content = "1, 2, 3\n4 , 5 , 6";
        let path = create_csv_test_file(content);
        let result = read_csv_matrix(path.to_str().unwrap());
        // Should fail because of leading/trailing spaces before parsing
        assert!(result.is_err());
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_read_csv_matrix_empty() {
        let path = create_csv_test_file("");
        let matrix = read_csv_matrix(path.to_str().unwrap()).unwrap();
        assert_eq!(matrix.len(), 0);
        fs::remove_file(&path).ok();
    }

    // read_space_separated_matrix tests
    #[test]
    fn test_read_space_separated_matrix_u32() {
        let content = "1 2 3\n4 5 6\n7 8 9";
        let path = create_space_separated_test_file(content);
        let matrix: Vec<Vec<u32>> = read_space_separated_matrix(path.to_str().unwrap()).unwrap();
        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0], vec![1, 2, 3]);
        assert_eq!(matrix[1], vec![4, 5, 6]);
        assert_eq!(matrix[2], vec![7, 8, 9]);
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_read_space_separated_matrix_i32() {
        let content = "-1 -2 -3\n4 5 6";
        let path = create_space_separated_test_file(content);
        let matrix: Vec<Vec<i32>> = read_space_separated_matrix(path.to_str().unwrap()).unwrap();
        assert_eq!(matrix.len(), 2);
        assert_eq!(matrix[0], vec![-1, -2, -3]);
        assert_eq!(matrix[1], vec![4, 5, 6]);
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_read_space_separated_matrix_u64() {
        let content = "1000000000 2000000000\n3000000000 4000000000";
        let path = create_space_separated_test_file(content);
        let matrix: Vec<Vec<u64>> = read_space_separated_matrix(path.to_str().unwrap()).unwrap();
        assert_eq!(matrix.len(), 2);
        assert_eq!(matrix[0][0], 1000000000);
        assert_eq!(matrix[1][1], 4000000000);
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_read_space_separated_matrix_tabs_and_spaces() {
        let content = "1  2\t3\n4   5\t\t6";
        let path = create_space_separated_test_file(content);
        let matrix: Vec<Vec<u32>> = read_space_separated_matrix(path.to_str().unwrap()).unwrap();
        assert_eq!(matrix.len(), 2);
        assert_eq!(matrix[0], vec![1, 2, 3]);
        assert_eq!(matrix[1], vec![4, 5, 6]);
        fs::remove_file(&path).ok();
    }

    // read_lines tests
    #[test]
    fn test_read_lines_simple() {
        let content = "Line 1\nLine 2\nLine 3";
        let path = create_test_file(content);
        let lines = read_lines(path.to_str().unwrap()).unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "Line 1");
        assert_eq!(lines[1], "Line 2");
        assert_eq!(lines[2], "Line 3");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_read_lines_single_line() {
        let content = "Single line";
        let path = create_test_file(content);
        let lines = read_lines(path.to_str().unwrap()).unwrap();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "Single line");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_read_lines_empty() {
        let path = create_test_file("");
        let lines = read_lines(path.to_str().unwrap()).unwrap();
        assert_eq!(lines.len(), 0);
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_read_lines_with_empty_lines() {
        let content = "Line 1\n\nLine 3";
        let path = create_test_file(content);
        let lines = read_lines(path.to_str().unwrap()).unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "Line 1");
        assert_eq!(lines[1], "");
        assert_eq!(lines[2], "Line 3");
        fs::remove_file(&path).ok();
    }
}
