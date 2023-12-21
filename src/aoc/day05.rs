// https://adventofcode.com/2023/day/4

use std::collections::HashMap;

use itertools::Itertools;

use super::utils::get_lines;

enum ParseStage {
    Seeds,
    SeedsToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: HashMap<u32, u32>,
    soil_to_fertilizer: HashMap<u32, u32>,
    fertilizer_to_water: HashMap<u32, u32>,
    water_to_light: HashMap<u32, u32>,
    light_to_temperature: HashMap<u32, u32>,
    temperature_to_humidity: HashMap<u32, u32>,
    humidity_to_location: HashMap<u32, u32>,
}

#[derive(Debug)]
struct Input {
    almanac: Almanac,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let input = Input {
        almanac: parse_almanac(lines),
    };

    input
}

fn parse_almanac(lines: Vec<String>) -> Almanac {
    let mut parse_stage: ParseStage = ParseStage::Seeds;
    let mut almanac = Almanac::default();
    let mut parse_nums;
    for almanac_line in lines {
        let split_line: Vec<&str> = almanac_line.split(':').collect();
        match split_line[0] {
            "seeds" => {
                parse_stage = ParseStage::Seeds;
                let seeds: Vec<u32> = split_line[1]
                    .trim()
                    .split(' ')
                    .map(|seed| seed.parse().unwrap())
                    .collect();
                almanac.seeds = seeds;
                continue;
            }
            "seed-to-soil map" => {
                parse_stage = ParseStage::SeedsToSoil;
                continue;
            }
            "soil-to-fertilizer map" => {
                parse_stage = ParseStage::SoilToFertilizer;
                continue;
            }
            "fertilizer-to-water map" => {
                parse_stage = ParseStage::FertilizerToWater;
                continue;
            }
            "water-to-light map" => {
                parse_stage = ParseStage::WaterToLight;
                continue;
            }
            "light-to-temperature map" => {
                parse_stage = ParseStage::LightToTemperature;
                continue;
            }
            "temperature-to-humidity map" => {
                parse_stage = ParseStage::TemperatureToHumidity;
                continue;
            }
            "humidity-to-location map" => {
                parse_stage = ParseStage::HumidityToLocation;
                continue;
            }
            "" => {
                continue;
            }
            _ => {
                parse_nums = true;
            }
        }

        if parse_nums {
            let nums: (u32, u32, u32) = split_line[0]
                .trim()
                .split(' ')
                .map(|seed| seed.parse().unwrap())
                .collect_tuple()
                .unwrap();

            match parse_stage {
                ParseStage::Seeds => (),
                ParseStage::SeedsToSoil => {
                    almanac.seed_to_soil.extend(create_ranges(nums));
                }
                ParseStage::SoilToFertilizer => {
                    almanac.soil_to_fertilizer.extend(create_ranges(nums));
                }
                ParseStage::FertilizerToWater => {
                    almanac.fertilizer_to_water.extend(create_ranges(nums));
                }
                ParseStage::WaterToLight => {
                    almanac.water_to_light.extend(create_ranges(nums));
                }
                ParseStage::LightToTemperature => {
                    almanac.light_to_temperature.extend(create_ranges(nums));
                }
                ParseStage::TemperatureToHumidity => {
                    almanac.temperature_to_humidity.extend(create_ranges(nums));
                }
                ParseStage::HumidityToLocation => {
                    almanac.humidity_to_location.extend(create_ranges(nums));
                }
            }
        }
    }
    almanac
}

fn create_ranges(nums: (u32, u32, u32)) -> HashMap<u32, u32> {
    let mut ranges: HashMap<u32, u32> = HashMap::new();

    let sources = nums.1..(nums.1 + nums.2);
    let dests = nums.0..(nums.0 + nums.2);
    for (source, dest) in sources.into_iter().zip(dests.into_iter()) {
        ranges.insert(source, dest);
    }

    ranges
}

fn get_lowest_location(input_file: &str) -> u32 {
    let mut locations: Vec<u32> = Vec::new();

    let input = parse_input(input_file);
 
    println!("Input parsed");  

    for seed in input.almanac.seeds {
        let soil_lookup;
        match input.almanac.seed_to_soil.get(&seed) {
            Some(&lookup_val) => soil_lookup = lookup_val,
            _ => soil_lookup = seed,
        }

        let fertilizer_lookup;
        match input.almanac.soil_to_fertilizer.get(&soil_lookup) {
            Some(&lookup_val) => fertilizer_lookup = lookup_val,
            _ => fertilizer_lookup = soil_lookup,
        }    
    
        let water_lookup;
        match input.almanac.fertilizer_to_water.get(&fertilizer_lookup) {
            Some(&lookup_val) => water_lookup = lookup_val,
            _ => water_lookup = fertilizer_lookup,
        }   

        let light_lookup;
        match input.almanac.water_to_light.get(&water_lookup) {
            Some(&lookup_val) => light_lookup = lookup_val,
            _ => light_lookup = water_lookup,
        }   

        let temperature_lookup;
        match input.almanac.light_to_temperature.get(&light_lookup) {
            Some(&lookup_val) => temperature_lookup = lookup_val,
            _ => temperature_lookup = light_lookup,
        }   

        let humidity_lookup;
        match input.almanac.temperature_to_humidity.get(&temperature_lookup) {
            Some(&lookup_val) => humidity_lookup = lookup_val,
            _ => humidity_lookup = temperature_lookup,
        } 

        let location_lookup;
        match input.almanac.humidity_to_location.get(&humidity_lookup) {
            Some(&lookup_val) => location_lookup = lookup_val,
            _ => location_lookup = humidity_lookup,
        }

        locations.push(location_lookup);
    }

    *locations.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lowest_location_test01() {
        assert_eq!(35, get_lowest_location("input/day05_test01.txt"));
    }

    #[test]
    fn test_get_lowest_location_test02() {
        assert_eq!(13, get_lowest_location("input/day05_test02.txt"));
    }

    #[test]
    fn test_get_lowest_location_score() {
        assert_eq!(0, get_lowest_location("input/day05.txt"));
    }
}