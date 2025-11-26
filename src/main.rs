// what is this going to do?
// prompt the user for weather, season, water location, and time of day
// will return the fish available to catch in the area
// include options to go back or restart the prompt

use std::io;
use std::fmt;
use serde::Deserialize;
use toml;

const FISH_DATA_TOML: &str = include_str!("fish-data.toml");

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

#[derive(Debug, Deserialize)]
struct Fish {
    name: String,
    conditions: Vec<Conditions>
}

impl fmt::Display for Conditions {
    fn fmt(&self, c: &mut fmt::Formatter) -> fmt::Result {
        /*
        so what are we trying to do?
        we are trying to log all the fields in a conditions struct
        each field is a vector of strings

        how to do it?
        for each field, iterate through each value
        concatenate the vector values into one comma separated list, prepended by the field name
         */

        fn format_field(field_name: &str, field: &Vec<String>) -> String {
            let mut output: String = format!("\t{}: ", field_name);
            if field.is_empty() {
                output.push_str(&format!("{}\n", "any".to_string()));
            }
            else {
                output.push_str(&format!("{}\n", field.iter().map(|a| a.as_str()).collect::<Vec<_>>().join(", ")));
            }
            output
        }
        write!(c, "{}", format_field("Dimension", &self.dimension))?;
        if !self.dimension.contains(&"nether".to_string()) {
            write!(c, "{}", format_field("Water location", &self.water_location))?;
        }
        if !self.dimension.contains(&"nether".to_string()) {
            write!(c, "{}", format_field("Seasons", &self.season))?;
        }
        if !self.dimension.contains(&"nether".to_string()) {
            write!(c, "{}", format_field("Time of day", &self.time_of_day))?;
        }
        if !self.dimension.contains(&"overworld".to_string()) {
            write!(c, "{}", format_field("Biome", &self.biome))?;
        }
        if !self.dimension.contains(&"nether".to_string()) {
            write!(c, "{}", format_field("Weather", &self.weather))?;
        }
        Ok(())
    }
}

impl fmt::Display for Fish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Fish name: {}\n", self.name)?;
        if self.name == "Neptuna" {
            println!("IMPORTANT NOTE! Neptuna becomes catchable once you've unlocked the skill tree perk.");
        }
        self.conditions.iter().try_for_each(|condition| {
            write!(f, "  - Condition set:\n {}", condition)
        })
    }
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

    let num_columns = 6;
    let column_width = 24;
    let max_list_length = 6;
    let formatted_list: String = if choices.len() < max_list_length {
        choices.join(", ")
    }
    else {
        choices.iter()
            .enumerate()
            .map(|(index, choice)| {
                let padded_choice = format!("{:<width$}", choice, width = column_width);

                if (index + 1) % num_columns == 0 {
                    format!("{}\n", padded_choice)
                }
                else {
                    padded_choice
                }
            })
            .collect()
    };
    loop {
        println!("{}", display_text);
        println!("Options:\n{}", formatted_list);

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

fn reverse_search<'a>(fish_name: &str, fish_list: &'a [Fish]) -> &'a Fish {
    fish_list.iter().find(|fish| fish.name.to_lowercase() == fish_name.to_lowercase()).expect("Internal logic error, validated fish name not found in the list")
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

        // let toml_str = fs::read_to_string("fish-data.toml")
        //     .expect("Failed to read file");

        let fish_data: FishData = toml::from_str(FISH_DATA_TOML).expect("Failed to parse TOML");
        let fish_list = fish_data.fish;

        println!("Welcome to the Society: Sunlit Valley Fish Radar!");
        let mut spawn_conditions = Conditions {
            water_location: vec![],
            season: vec![],
            time_of_day: vec![],
            biome: vec![],
            weather: vec![],
            dimension: vec![]
        };
        let search_method = text_input("Are you looking for what fish are catchable around you? Or when to catch a specific fish?", &["All", "Specific", "Exit"]);
        if search_method == "all" {
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
            println!("Here are the spawn conditions you provided:\n{}", spawn_conditions);
            println!("\nCatchable fish:");
                for fish in return_list {
                    println!("  - {}", fish.name);
                }
            }
        else if search_method == "specific" {
            let fish_name_refs : Vec<&str> = fish_list
                .iter()
                .map(|fish| fish.name.as_str())
                .collect();
            let fish_name_slice: &[&str] = fish_name_refs.as_slice();
            let fish_name = text_input("Please enter the fish you are searching for:", fish_name_slice);
            let found_fish_conditions = reverse_search(fish_name.as_str(), &fish_list);
            println!("\nHere are the conditions required to catch {}:", found_fish_conditions.name);
            println!("\n{}", found_fish_conditions);
        }
        else {
            break;
        }
    }
    println!("\nThank you for using the Society: Sunlit Valley Fish Radar!");

}