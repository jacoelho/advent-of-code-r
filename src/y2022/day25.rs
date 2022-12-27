fn from_snafu(s: &str) -> i64 {
    s.chars().fold(0, |carry, char| {
        (carry * 5)
            + match char {
                '-' => -1,
                '=' => -2,
                _ => char.to_digit(10).unwrap() as i64,
            }
    })
}

struct Snafu {
    value: i64,
    exausted: bool,
}

impl Snafu {
    fn new(value: i64) -> Self {
        Self {
            value,
            exausted: false,
        }
    }
}

const CHARS: [char; 5] = ['0', '1', '2', '=', '-'];

impl Iterator for Snafu {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exausted {
            return None;
        }

        let res = self.value % 5;

        self.value = (self.value + 2) / 5;

        if self.value == 0 {
            self.exausted = true
        }

        Some(CHARS[(res % 5) as usize])
    }
}

fn part01(path: &str) -> String {
    let sum = std::fs::read_to_string(path)
        .expect("should be able to read file")
        .lines()
        .map(from_snafu)
        .sum();

    Snafu::new(sum).collect::<Vec<_>>().iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day25-example.txt"), "2=-1=0".to_string())
    }

    #[test]
    fn part01_input() {
        assert_eq!(
            part01("data/y2022/day25.txt"),
            "2011-=2=-1020-1===-1".to_string()
        )
    }
}
