extern crate rs_workdays;

use rs_workdays::scrape_holidays::{make_source};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    make_source("source/holidays.csv")?;
    Ok(())
}