extern crate rs_workdays;
#[cfg(feature="source")]
use rs_workdays::scrape_holidays::{make_source_naikaku};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature="source")]
    make_source_naikaku("source/holidays.csv")?;
    
    Ok(())
}