use std::str;

pub fn read_value_per_line<T>(path: &str) -> Vec<T>
where
    T: str::FromStr,
{
    std::fs::read_to_string(path)
        .expect("should be able to read file")
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect()
}

pub fn read_value_chunks<T>(path: &str) -> Vec<Vec<T>>
where
    T: str::FromStr,
{
    std::fs::read_to_string(path)
        .expect("expected file")
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .filter_map(|line| line.parse::<T>().ok())
                .collect()
        })
        .collect()
}
