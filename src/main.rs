use clap::{App, Arg};
mod recipe;

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
    print!(
        "{}",
        recipe::recipe(flower, hydration, starter, salt, brine_water)
    )
}
