fn to_digits(v: u64) -> Vec<u64> {
    let mut remain = v;
    let mut digits = Vec::with_capacity(10);

    while remain > 0 {
        digits.push(remain % 10);
        remain /= 10;
    }

    digits.into_iter().rev().collect::<Vec<_>>()
}

fn to_digit(v: Vec<u64>) -> u64 {
    v.iter().fold(0, |acc, digit| acc * 10 + digit)
}
