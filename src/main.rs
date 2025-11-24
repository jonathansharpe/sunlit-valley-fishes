// what is this going to do?
// prompt the user for weather, season, water location, and time of day
// will return the fish available to catch in the area
// include options to go back or restart the prompt

use std::io;
use std::io::Read;
use std::collections::HashMap;
use std::fs;
use serde::Deserialize;
use toml;

// high level overview of what we're implementing:
// the user will be prompted to enter their current dimension
// if its the overworld, they will be prompted for the seaons, water type, time of day, and weather
// if its the nether, they will be prompted for the biome
// the data is stored in a TOML file

// there will be default values:
// season: all
// water_location: all
// time of day: all
// weather: all
// biome: any
// dimension: overworld
// the toml file will not provide the above fields unless they are different from above

// data types for each field:
// seasons: Vec
// water_location; Vec
// time_of_day: Vec
// weather: Vec
// biome: Vec
// dimension: String

// edge cases:
// some fish require different times of day depending on the season. for example, tropical fish are catchable in the spring ocean at night, but at any time in the summer ocean
// some fish require different weather conditions depending on the season. for example, carp are catchable in spring freshwater during the day when its raining, but in summer freshwater during the day in any weather

// how to solve this:
// option 1: have the conditions be all one field. for example:
/* 
    name: "Tropical Fish",
    conditions: {
        [
            season: "spring",
            time_of_day: "night"
        ],
        [
            season: "summer"
            // this is catchable any time during the summer, and any time is the default value
        ]
    }

    name: "Carp",
    conditions: {
        [
            season: "spring",
            time_of_day: "day",
            weather: "rain"
        ],
        [
            season: ["summer","autumn"],
            time_of_day: "day"
            // catchable during any weather, which is default value
        ],
        [
            season: "winter",
            time_of_day: "night"
        ]
    }

    need to convert the above to TOML
 */

 /*
  now we need to make the struct. there will be two
  first is the Fish
  it will have these fields: name, conditions[]

  next we have Conditions
  it will have these fields:
    water_location: string or vector
    season: string or vector
    time of day: string or vector
    biome: string
    weather: string or vector
  */

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct Conditions {
    water_location: Vec<String>,
    season: Vec<String>,
    time_of_day: Vec<String>,
    biome: Vec<String>,
    weather: Vec<String>,
    dimension: Vec<String>
}

// impl Default for Conditions {
//     fn default() -> Conditions {
//         Conditions {
//             water_location: vec![], 
//             season: vec![],
//             time_of_day: vec![],
//             biome: vec![],
//             weather: vec![],
//             dimension: vec![]
//         }
//     }
// }

#[derive(Debug, Deserialize)]
struct Fish {
    name: String,
    conditions: Vec<Conditions>
}

#[derive(Debug, Deserialize)]
struct FishData {
    fish: Vec<Fish>
}

fn text_input(display_text: &str, choices: &[&str]) -> String {
    // pseudocode
    // print the display text so the user knows what to enter
    // accept the input
    // run a match to validate the input; adjust depending on the bounds
    let choices_lower: Vec<String> = choices.iter()
        .map(|s| s.to_lowercase())
        .collect();

    loop {
        println!("{}", display_text);
        println!("Options: {}", choices.join(", "));

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let input_lower = input.trim().to_lowercase();

        if choices_lower.contains(&input_lower) {
            return input_lower;
        }
        else {
            println!("Invalid choice, please try again");
        }
    }
}

fn condition_matches(condition: &Conditions, filter: &Conditions) -> bool {
    (condition.season.is_empty() || filter.season.is_empty() || condition.season.iter().any(|s| filter.season.contains(s))) &&
    (condition.water_location.is_empty() || filter.water_location.is_empty() || condition.water_location.iter().any(|w| filter.water_location.contains(w))) &&
    (condition.time_of_day.is_empty() || filter.time_of_day.is_empty() || condition.time_of_day.iter().any(|t| filter.time_of_day.contains(t))) &&
    (condition.weather.is_empty() || filter.weather.is_empty() || condition.weather.iter().any(|w| filter.weather.contains(w))) &&
    (condition.biome.is_empty() || filter.biome.is_empty() || condition.biome.iter().any(|b| filter.biome.contains(b))) &&
    (condition.dimension.is_empty() || filter.dimension.is_empty() || condition.dimension.iter().any(|d| filter.dimension.contains(d)))
}

/*
following function will be given the parameters, run a loop through the list and find the catchable fish. it will return a list of those fish
 */
fn find_fish<'a>(filter: &Conditions, fish_list: &'a [Fish]) -> Vec<&'a Fish> {
    fish_list
        .iter()
        .filter(|fish| {
            fish.conditions.iter().any(|condition| condition_matches(condition, filter))
        })
        .collect()
}

fn main() {

    loop {

        let toml_str = fs::read_to_string("fish-data.toml")
            .expect("Failed to read file");

        let fish_data: FishData = toml::from_str(&toml_str).expect("Failed to parse TOML");
        let fish_list = fish_data.fish;

        //println!("{:#?}", fish_list);

        println!("Welcome to the Sunlit Valley Fish Radar!");
        let mut spawn_conditions = Conditions {
            water_location: vec![],
            season: vec![],
            time_of_day: vec![],
            biome: vec![],
            weather: vec![],
            dimension: vec![]
        };
        let dimension = text_input("Enter the dimension:", &["Overworld", "Nether"]);

        if dimension == "overworld" {
       		let season = text_input("Enter the current season:", &["Spring", "Summer", "Autumn", "Winter"]);
      		let water_location = text_input("Enter the water type:", &["Fresh", "River", "Ocean"]);
        	let time_of_day = text_input("Enter the time of day:", &["Day", "Night"]);
        	let weather = text_input("Enter the weather:", &["Clear", "Rain"]);
            spawn_conditions.season.push(season);
            spawn_conditions.time_of_day.push(time_of_day);
            spawn_conditions.water_location.push(water_location);
            spawn_conditions.weather.push(weather);
        }
        else {
			let biome = text_input("Enter the Nether biome:", &["Soul Sand Valley", "Nether Wastes", "Crimson Forest", "Warped Forest", "Basalt Deltas"]);
            spawn_conditions.biome.push(biome);
        }
        spawn_conditions.dimension.push(dimension);

        let return_list = find_fish(&spawn_conditions, &fish_list);
        println!("\nCatchable fish:");
        for fish in return_list {
            println!("  - {}", fish.name);
        }
    }

}