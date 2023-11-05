use std::collections::HashMap;

fn to_digits(v: u64) -> Vec<u64> {
    let mut remain = v;
    let mut digits = Vec::with_capacity(10);

    while remain > 0 {
        digits.push(remain % 10);
        remain /= 10;
    }

    digits.into_iter().rev().collect::<Vec<_>>()
}
fn is_increasing(number: &Vec<u64>) -> bool {
    number.windows(2).all(|el| el[0] <= el[1])
}

fn contains_pair(number: &Vec<u64>) -> bool {
    number
        .iter()
        .fold(HashMap::<u64, usize>::new(), |mut acc, v| {
            *acc.entry(*v).or_default() += 1;
            acc
        })
        .iter()
        .any(|(_, &count)| count >= 2)
}

fn contains_exactly_one_pair(number: &Vec<u64>) -> bool {
    number
        .iter()
        .fold(HashMap::<u64, usize>::new(), |mut acc, v| {
            *acc.entry(*v).or_default() += 1;
            acc
        })
        .iter()
        .any(|(_, &count)| count == 2)
}

fn part01(input: &[u64; 2]) -> usize {
    (input[0]..=input[1])
        .map(to_digits)
        .filter(|v| is_increasing(v) && contains_pair(v))
        .count()
}

fn part02(input: &[u64; 2]) -> usize {
    (input[0]..=input[1])
        .map(to_digits)
        .filter(|v| is_increasing(v) && contains_exactly_one_pair(v))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_input() {
        assert_eq!(part01(&[172_851, 675_869]), 1660);
    }

    #[test]
    fn part02_input() {
        assert_eq!(part02(&[172_851, 675_869]), 1135);
    }
}
