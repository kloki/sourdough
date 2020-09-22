use clap::{App, Arg};

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
            Arg::new("flower")
                .about("How much flower to use")
                .required(true)
                .index(1)
                .default_value("1000"),
        )
        .get_matches();

    let flower = match args.value_of("flower").unwrap().parse::<i32>() {
        Ok(n) => n,
        Err(n) => panic!("{} invalid value for flower", n),
    };
    let hydration = match args.value_of("hydration").unwrap().parse::<i32>() {
        Ok(n) => n,
        Err(n) => panic!("{} invalid value for hydration", n),
    };
    let starter = 20;
    let salt = 2;
    let brine_water = 5;

    println!("Sourdough {} {}:{}:{}", flower, hydration, starter, salt);
    println!("dough:");
    println!(" {:4} gr flower", flower);
    println!(" {:4} gr water", flower * (hydration - brine_water) / 100);
    println!(" {:4} gr starter", flower * starter / 100);
    println!("brine:");
    println!(" {:4} gr water", flower * (brine_water) / 100);
    println!(" {:4} gr salt", flower * salt / 100);
}
