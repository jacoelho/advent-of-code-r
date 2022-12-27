use crate::io::read_value_per_line;

#[derive(Debug, Clone)]
struct Number {
    original_index: usize,
    value: i64,
}

fn decrypt(numbers: Vec<Number>) -> Vec<Number> {
    let mut result = numbers.clone();

    for original_index in 0..numbers.len() {
        let index = result
            .iter()
            .position(|el| el.original_index == original_index)
            .unwrap();

        let number = result.remove(index);

        let new_index = (index as i64 + number.value).rem_euclid(result.len() as i64) as usize;

        result.insert(new_index, number);
    }

    result
}

fn part01(path: &str) -> i64 {
    let values = read_value_per_line::<i64>(path)
        .iter()
        .enumerate()
        .map(|(idx, value)| Number {
            original_index: idx,
            value: *value,
        })
        .collect::<Vec<Number>>();

    let decrypted = decrypt(values);

    let zero = decrypted
        .iter()
        .position(|number| number.value == 0)
        .unwrap();

    [1000, 2000, 3000]
        .into_iter()
        .map(|offset| decrypted[(zero + offset) % decrypted.len()].value)
        .sum()
}

const DECRYPTION_KEY: i64 = 811589153;

fn part02(path: &str) -> i64 {
    let values = read_value_per_line::<i64>(path)
        .iter()
        .enumerate()
        .map(|(idx, value)| Number {
            original_index: idx,
            value: *value * DECRYPTION_KEY,
        })
        .collect::<Vec<Number>>();

    let mut decrypted = values;

    for _ in 0..10 {
        decrypted = decrypt(decrypted);
    }

    let zero = decrypted
        .iter()
        .position(|number| number.value == 0)
        .unwrap();

    [1000, 2000, 3000]
        .into_iter()
        .map(|offset| decrypted[(zero + offset) % decrypted.len()].value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2022/day20-example.txt"), 3);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2022/day20.txt"), 23321);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2022/day20-example.txt"), 1623178306);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2022/day20.txt"), 1428396909280);
    }
}
