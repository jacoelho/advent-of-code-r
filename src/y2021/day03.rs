pub fn part01() -> Result<usize, ()> {
    Ok(5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_fails() {
        assert_eq!(part01().unwrap(), 5);
    }
}
