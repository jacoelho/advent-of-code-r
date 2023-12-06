use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
struct GardenRange {
    to: u64,
    from: u64,
    length: u64,
}

impl FromStr for GardenRange {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split_whitespace()
            .filter_map(|v| v.parse::<u64>().ok())
            .collect::<Vec<_>>();

        if values.len() != 3 {
            return Err("invalid range");
        }

        Ok(GardenRange { to: values[0], from: values[1], length: values[2] })
    }
}

impl GardenRange {
    const fn translate(&self, pos: u64) -> Option<u64> {
        if self.from <= pos && pos < self.from + self.length {
            Some(pos - self.from + self.to)
        } else {
            None
        }
    }

    const fn rev(&self) -> Self {
        Self { to: self.from, from: self.to, length: self.length }
    }
}

#[derive(Debug, PartialEq)]
struct Almanac(Vec<Vec<GardenRange>>);
// seed_to_soil: Vec<GardenRange>,
// soil_to_fertilizer: Vec<GardenRange>,
// fertilizer_to_water: Vec<GardenRange>,
// water_to_light: Vec<GardenRange>,
// light_to_temperature: Vec<GardenRange>,
// temperature_to_humidity: Vec<GardenRange>,
// humidity_to_location: Vec<GardenRange>,

impl FromStr for Almanac {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections = s
            .split("\n\n")
            .map(|section| {
                section
                    .split('\n')
                    .skip(1)
                    .filter_map(|s| s.parse::<GardenRange>().ok())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if sections.len() != 7 {
            return Err("invalid almanac");
        }

        // Ok(Almanac {
        //     seed_to_soil: sections[0].clone(),
        //     soil_to_fertilizer: sections[1].clone(),
        //     fertilizer_to_water: sections[2].clone(),
        //     water_to_light: sections[3].clone(),
        //     light_to_temperature: sections[4].clone(),
        //     temperature_to_humidity: sections[5].clone(),
        //     humidity_to_location: sections[6].clone(),
        // })
        Ok(Almanac(sections))
    }
}

impl Almanac {
    fn translate(table: &[GardenRange], pos: u64) -> u64 {
        table.iter().find_map(|t| t.translate(pos)).unwrap_or(pos)
    }

    fn find(&self, pos: u64) -> u64 {
        self.0.iter().fold(pos, |acc, t| {
            t.iter().find_map(|range| range.translate(acc)).unwrap_or(acc)
        })
    }
}

fn parse_input(path: &str) -> (Vec<u64>, Almanac) {
    let contents =
        std::fs::read_to_string(path).expect("should be able to read file");

    contents
        .split_once("\n\n")
        .map(|(seeds, almanac)| {
            let seeds = seeds
                .split_once(": ")
                .map(|(_, seeds)| {
                    seeds
                        .split_whitespace()
                        .filter_map(|s| s.parse::<u64>().ok())
                        .collect::<Vec<_>>()
                })
                .expect("parsing seeds to work");

            let almanac = almanac.parse::<Almanac>().expect("almanac");

            (seeds, almanac)
        })
        .expect("to parse")
}

fn part01(path: &str) -> u64 {
    let (seeds, almanac) = parse_input(path);

    seeds.iter().map(|seed| almanac.find(*seed)).min().expect("expected a min")
}

fn part02(path: &str) -> u64 {
    let (seeds, almanac) = parse_input(path);

    let seeds = seeds
        .chunks(2)
        .flat_map(|chunk| (chunk[0]..chunk[0] + chunk[1]).collect::<Vec<_>>())
        .collect::<HashSet<_>>();

    let almanac = Almanac(
        almanac
            .0
            .iter()
            .map(|section| {
                section.iter().map(GardenRange::rev).collect::<Vec<_>>()
            })
            .rev()
            .collect::<Vec<_>>(),
    );

    (1..)
        .find(|location| seeds.contains(&almanac.find(*location)))
        .expect("expected a location")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part01_example() {
        assert_eq!(part01("data/y2023/day05-example1.txt"), 35);
    }

    #[test]
    fn part01_input() {
        assert_eq!(part01("data/y2023/day05.txt"), 551_761_867);
    }

    #[test]
    fn part02_example() {
        assert_eq!(part02("data/y2023/day05-example1.txt"), 46);
    }

    #[ignore]
    #[test]
    fn part02_input() {
        assert_eq!(part02("data/y2023/day05.txt"), 57_451_709);
    }
}
