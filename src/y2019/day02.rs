use crate::y2019::intcode;

fn read_input(path: &str) -> Vec<i32> {
    std::fs::read_to_string(path)
        .expect("should be able to read file")
        .split(',')
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<_>>()
}

fn computer(memory: &[i32], noun: i32, verb: i32) -> i32 {
    let mut computer = intcode::IntCode::new(&memory);

    computer.replace(noun, verb);
    computer.run();

    computer.dump()[0]
}

pub fn part01(path: &str) -> i32 {
    let memory = read_input(path);

    computer(&memory, 12, 2)
}

pub fn part02(path: &str) -> i32 {
    let memory = read_input(path);

    for x in 0..100 {
        for y in 0..100 {
            if computer(&memory, x, y) == 19_690_720 {
                return 100 * x + y;
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2019/day02.txt"), 3_706_713);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2019/day02.txt"), 8609);
    }
}
