use std::fs::File;
use std::io::{self, BufRead};

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

pub fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}
