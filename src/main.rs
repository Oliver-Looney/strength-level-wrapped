mod get_input;
mod structs;
mod enums;

use std::error::Error;
use get_input::get_input;

fn main() -> Result<(), Box<dyn Error>> {
    let previous_month_data  = get_input()?;
    println!("{:#?}", previous_month_data);
    Ok(())
}
