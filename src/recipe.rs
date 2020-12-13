const BREAD: char = '\u{1F35E}';
const SALT: char = '\u{1F9C2}';
const BOWL: char = '\u{1F963}';
const WATER: char = '\u{1F4A7}';
const FLOWER: char = '\u{1F33C}';

pub fn sourdough(flower: i32, hydration: i32, starter: i32, salt: i32, brine_water: i32) -> String {
    let mut recipe = format!(
        "{} Sourdough {} - {}:{}:{}:{}\n",
        BREAD, flower, hydration, starter, salt, brine_water
    );
    recipe = recipe + &format!("Dough:\n");
    recipe = recipe + &format!(" {:4} gr{} flower\n", flower, FLOWER);
    recipe = recipe
        + &format!(
            " {:4} gr{} water\n",
            flower * (hydration - brine_water) / 100,
            WATER,
        );
    recipe = recipe + &format!(" {:4} gr{} starter\n", flower * starter / 100, BOWL);
    recipe = recipe + &format!("Brine:\n");
    recipe = recipe + &format!(" {:4} gr{} salt\n", flower * salt / 100, SALT);
    recipe = recipe + &format!(" {:4} gr{} water\n", flower * (brine_water) / 100, WATER);
    return recipe;
}
