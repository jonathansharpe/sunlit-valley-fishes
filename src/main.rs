// what is this going to do?
// prompt the user for weather, season, water location, and time of day
// will return the fish available to catch in the area
// include options to go back or restart the prompt

use std::io;

fn text_input(display_text: &str, x: i32) -> i32 {
    // pseudocode
    // print the display text so the user knows what to enter
    // accept the input
    // run a match to validate the input; adjust depending on the bounds
    let ret_val = loop {
        println!("{}", display_text);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        match input.trim().parse::<i32>() {
            Ok(num) if num >= 1 && num <= x => break num,
            _ => println!("Invalid input, try again")
        }
    };
    return ret_val;
}

fn main() {
    //let season = 1; // 1 = spring, 2 = summer, 3 = autumn, 4 = winter
    //let rain = true; // true = rain, false = no rain
    //let water_location = 0; // 1 = fresh water, 2 = river, 3 = ocean
    //let daytime = true; // true = day, false = night

    //let nether = false; // true = nether, false = overworld
    //let biome = 0; // 1 = basalt delta, 2 = crimson forest, 3 = warped forest, 4 = soul sand valley, 5 = nether wastes

    loop {

        println!("Welcome to the Sunlit Valley Fish Radar!");
        let dimension = text_input("Are you fishing in the Overworld or Nether? Enter 1 for Overworld, 2 for Nether", 2);

        if (dimension == 1) {
       		let season = text_input("Enter the season; 1. Spring, 2. Summer, 3. Fall, 4. Winter", 4);
      		let water_location = text_input("Enter the water type; 1. Fresh, 2. River, 3. Ocean", 3);
        	let daytime = text_input("Enter the time of day; 1. Daytime, 2. Nighttime", 2);
        	let rain = text_input("Enter the weather; 1. Clear, 2. Rain", 2);
        }
        else {
			let biome = text_input("Enter the biome: 1. Basalt Deltas, 2. Crimson Forest, 3. Warped Forest, 4. Soul Sand Valley, 5. Nether Wastes", 5);
        }
        // println!("{} {}", season, water_location);
    }

}