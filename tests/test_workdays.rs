use chrono::{NaiveDate};

extern crate  rs_workdays;

use rs_workdays::workdays::{check_workday, get_workdays};

#[test]
fn all_test() {
    let select_date = NaiveDate::from_ymd(2021,1,1);
    assert!(!check_workday(select_date));   
}