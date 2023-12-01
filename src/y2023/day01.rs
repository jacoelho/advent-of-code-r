struct Slide<'a> {
    curr: usize,
    data: &'a str,
}

const fn slide_iter(s: &str) -> Slide {
    Slide { curr: 0, data: s }
}

impl<'a> Iterator for Slide<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr += 1;

        if current < self.data.len() {
            Some(&self.data[current..])
        } else {
            None
        }
    }
}

struct Calibration<'a>(&'a str, i32);
fn calibration_value(calibration: &[Calibration], v: &str) -> Option<i32> {
    calibration.iter().find(|Calibration(s, _)| v.starts_with(s)).map(|c| c.1)
}

fn calibrate(calibration: &[Calibration], path: &str) -> i32 {
    std::fs::read_to_string(path)
        .expect("expected file")
        .lines()
        .map(|line| {
            slide_iter(line)
                .filter_map(|line| calibration_value(calibration, line))
                .collect::<Vec<_>>()
        })
        .fold(0, |acc, elem| {
            elem.first().unwrap_or(&0) * 10 + elem.last().unwrap_or(&0) + acc
        })
}

pub fn part01(path: &str) -> i32 {
    let calibration = vec![
        Calibration("1", 1),
        Calibration("2", 2),
        Calibration("3", 3),
        Calibration("4", 4),
        Calibration("5", 5),
        Calibration("6", 6),
        Calibration("7", 7),
        Calibration("8", 8),
        Calibration("9", 9),
    ];

    calibrate(&calibration, path)
}

pub fn part02(path: &str) -> i32 {
    let calibration = vec![
        Calibration("one", 1),
        Calibration("1", 1),
        Calibration("two", 2),
        Calibration("2", 2),
        Calibration("three", 3),
        Calibration("3", 3),
        Calibration("four", 4),
        Calibration("4", 4),
        Calibration("five", 5),
        Calibration("5", 5),
        Calibration("six", 6),
        Calibration("6", 6),
        Calibration("seven", 7),
        Calibration("7", 7),
        Calibration("eight", 8),
        Calibration("8", 8),
        Calibration("nine", 9),
        Calibration("9", 9),
    ];

    calibrate(&calibration, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2023/day01-example1.txt"), 142);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day01.txt"), 54708);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2023/day01-example2.txt"), 281);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day01.txt"), 54087);
    }
}
