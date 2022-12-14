use std::collections::HashMap;

fn parse_input(path: &str) -> HashMap<(i32, i32), char> {
    std::fs::read_to_string(path)
        .expect("expected file")
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|p| {
                    let (x, y) = p.split_once(',').unwrap();

                    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
                })
                .collect::<Vec<_>>()
                .windows(2)
                .flat_map(|el| {
                    let (sx, sy) = el[0];
                    let (ex, ey) = el[1];

                    if sx == ex {
                        (sy..=ey).map(|y| ((sx, y), '#')).collect::<Vec<_>>()
                    } else {
                        (sx..=ex).map(|x| ((x, sy), '#')).collect::<Vec<_>>()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<(i32, i32), char>>()
}

fn print_grid(g: &HashMap<(i32, i32), char>) {
    let x = g.keys().map(|(x, _)| *x).collect::<Vec<_>>();
    let y = g.keys().map(|(_, y)| *y).collect::<Vec<_>>();
    let min_x = *x.iter().min().unwrap();
    let max_x = *x.iter().max().unwrap();
    let min_y = *y.iter().min().unwrap();
    let max_y = *y.iter().max().unwrap();

    for y in min_y..=max_y {
        print!("{} ", y);
        for x in min_x..=max_x {
            print!("{}", g.get(&(x, y)).unwrap_or(&'.'));
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid() {
        let g = parse_input("data/y2022/day14-example.txt");

        print_grid(&g);
    }
}
