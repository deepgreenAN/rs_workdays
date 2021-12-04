extern crate rs_workdays;
#[cfg(not(feature="wasm"))]
use rs_workdays::scrape_holidays::{make_source_naikaku};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature="wasm"))]
    make_source_naikaku("source/holidays.csv")?;
    
    Ok(())
}