use chrono::NaiveDate;

extern crate rs_workdays;


#[cfg(not(feature="wasm"))]
use rs_workdays::request_holidays::request_holidays_naikaku;

use rs_workdays::{get_workdays, Closed};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !cfg!(feature="wasm") {
        #[cfg(not(feature="wasm"))]
        request_holidays_naikaku(2016_i32, 2025_i32)?;

        let workday_start_date = NaiveDate::from_ymd(2021,1,1);
        let workday_end_date = NaiveDate::from_ymd(2021,2,1);
        let workdays_vec = get_workdays(workday_start_date, workday_end_date, Closed::Left);
        println!("workdays_vec: {:?}", workdays_vec)
    } else {
        println!("this file is for not wasm");
    }
;
    Ok(())
}