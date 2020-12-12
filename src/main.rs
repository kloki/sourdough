use clap::{App, Arg};

const BREAD: char = '\u{1F35E}';
const SALT: char = '\u{1F9C2}';
const BOWL: char = '\u{1F963}';
const WATER: char = '\u{1F4A7}';
const FLOWER: char = '\u{1F33C}';

fn main() {
    let args = App::new("Sourdough calculator")
        .version("1.0")
        .author("koen")
        .arg(
            Arg::new("hydration")
                .short('h')
                .long("hydration")
                .value_name("hydration")
                .about("Hydration percentage.")
                .default_value("70"),
        )
        .arg(
            Arg::new("brine_water")
                .short('b')
                .long("brine_water")
                .value_name("hydration")
                .about("Water used in brine (subtracted from hydration)")
                .default_value("5"),
        )
        .arg(
            Arg::new("starter")
                .short('s')
                .long("starter")
                .value_name("starter")
                .about("Starter percentage.")
                .default_value("20"),
        )
        .arg(
            Arg::new("salt")
                .short('S')
                .long("salt")
                .value_name("salt")
                .about("Salt percentage.")
                .default_value("2"),
        )
        .arg(
            Arg::new("flower")
                .about("How much flower to use")
                .required(true)
                .index(1)
                .default_value("1000"),
        )
        .about("Create ingredient list based on bakers percent")
        .get_matches();
    let flower = args
        .value_of("flower")
        .unwrap()
        .parse::<i32>()
        .expect("invalid value for flower");
    let hydration = args
        .value_of("hydration")
        .unwrap()
        .parse::<i32>()
        .expect("invalid value for hydration");
    let starter = args
        .value_of("starter")
        .unwrap()
        .parse::<i32>()
        .expect("invalid value for starter");
    let salt = args
        .value_of("salt")
        .unwrap()
        .parse::<i32>()
        .expect("invalid value for salt");
    let brine_water = args
        .value_of("brine_water")
        .unwrap()
        .parse::<i32>()
        .expect("invalid value for brine_water");
    println!(
        "{} Sourdough {} - {}:{}:{}:{}",
        BREAD, flower, hydration, starter, salt, brine_water
    );
    println!("Dough:");
    println!(" {:4} gr{} flower", flower, FLOWER);
    println!(
        " {:4} gr{} water",
        flower * (hydration - brine_water) / 100,
        WATER,
    );
    println!(" {:4} gr{} starter", flower * starter / 100, BOWL);
    println!("Brine:");
    println!(" {:4} gr{} salt", flower * salt / 100, SALT);
    println!(" {:4} gr{} water", flower * (brine_water) / 100, WATER);
}
