use chrono::{NaiveDate};

pub fn true_holidays_2021() -> Vec<NaiveDate> {
    let holidays_vec: Vec<NaiveDate> = [
        NaiveDate::from_ymd(2021,1,1),
        NaiveDate::from_ymd(2021,1,11),
        NaiveDate::from_ymd(2021,2,11),
        NaiveDate::from_ymd(2021,2,23),
        NaiveDate::from_ymd(2021,3,20),
        NaiveDate::from_ymd(2021,4,29),
        NaiveDate::from_ymd(2021,5,3),
        NaiveDate::from_ymd(2021,5,4),
        NaiveDate::from_ymd(2021,5,5),
        NaiveDate::from_ymd(2021,7,22),
        NaiveDate::from_ymd(2021,7,23),
        NaiveDate::from_ymd(2021,8,8),
        NaiveDate::from_ymd(2021,8,9),  //振替
        NaiveDate::from_ymd(2021,9,20),
        NaiveDate::from_ymd(2021,9,23),
        NaiveDate::from_ymd(2021,11,3),
        NaiveDate::from_ymd(2021,11,23)
    ].to_vec();
    return holidays_vec;
}
