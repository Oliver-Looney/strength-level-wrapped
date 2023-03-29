mod get_input;
mod structs;
mod enums;
mod save_html_file;

use std::error::Error;
use get_input::get_input;
use save_html_file::save_file;

fn main() -> Result<(), Box<dyn Error>> {
    let previous_month_data  = get_input()?;
    println!("{:#?}", previous_month_data);
    save_file()?;
    Ok(())
}
