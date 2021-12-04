use std::collections::HashSet;
use chrono::{NaiveDate, Datelike, NaiveDateTime, Duration};

extern crate  rs_workdays;

use rs_workdays::workdays::*;
use rs_workdays::intraday::*;
use rs_workdays::extract::*;
use rs_workdays::global::{ONE_HOLIDAY_WEEKDAY_SET, INTRADAY_BORDERS};

mod test_global;
use test_global::{true_holidays_2021};

#[test]
fn related_workdays() {
    // get_workdays
    let one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.read().unwrap();
    let start_date = NaiveDate::from_ymd(2021, 1, 1);
    let end_date = NaiveDate::from_ymd(2021, 12, 31);
    let all_day_set: HashSet<NaiveDate> = start_date.iter_days()
    .take_while(|x| {x<=&end_date}).collect();  // 全ての日

    let true_holiday_set: HashSet<NaiveDate> = true_holidays_2021().iter().cloned().collect();
    let true_workdays_set: HashSet<NaiveDate> = all_day_set.iter().cloned()
    .filter(|x|{!true_holiday_set.contains(x)})
    .filter(|x|{!one_holiday_weekday_set.contains(&x.weekday())}).collect();

    let true_not_workdays_set: HashSet<NaiveDate> = all_day_set.difference(&true_workdays_set).cloned().collect();

    let workdays_set: HashSet<NaiveDate> = get_workdays(start_date, end_date, Closed::Both).iter().cloned().collect();
    assert_eq!(workdays_set, true_workdays_set);

    // get_workdays_number
    let workdays_set: HashSet<NaiveDate> = get_workdays_number(start_date, true_workdays_set.len() as i32).iter().cloned().collect();
    assert_eq!(workdays_set, true_workdays_set);
    let workdays_set: HashSet<NaiveDate> = get_workdays_number(end_date, -(true_workdays_set.len() as i32)).iter().cloned().collect();
    assert_eq!(workdays_set, true_workdays_set);

    // check_workday  
    let checked_wokdays: Vec<bool> = true_workdays_set.iter().cloned().map(|x|{check_workday(x)}).collect();
    assert!(checked_wokdays.iter().all(|x|{*x}));
    let checked_not_workdays: Vec<bool> = true_not_workdays_set.iter().cloned().map(|x|{check_workday(x)}).collect();
    assert!(!checked_not_workdays.iter().any(|x|{*x}));
    
    // get_next_workday
    let workdays_set: HashSet<NaiveDate> = (1..(true_workdays_set.len()+1) as i32).map(|x|{get_next_workday(start_date, x)}).collect();
    assert_eq!(workdays_set, true_workdays_set);

    // get_previous_workday
    let mut workdays_set: HashSet<NaiveDate> = (1..(true_workdays_set.len()) as i32).map(|x|{get_previous_workday(end_date, x)}).collect();
    workdays_set.insert(end_date); // end_dateが営業日であるため
    assert_eq!(workdays_set, true_workdays_set);

    // get_near_workday
    let near_workday = get_near_workday(start_date, true);
    assert_eq!(near_workday, NaiveDate::from_ymd(2021, 1, 4));
    let near_workday = get_near_workday(start_date, false);
    assert_eq!(near_workday, NaiveDate::from_ymd(2020, 12, 31));
}

#[test]
fn related_extract() {
    let start_datetime = NaiveDate::from_ymd(2021, 1, 1).and_hms(0, 0, 0);
    let end_datetime = NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0);
    let per_sec = 30*60; // 30分
    let mut all_datetime_vec: Vec<NaiveDateTime> = [].to_vec();
    let mut itered_datetime = start_datetime.clone();
    loop {
        itered_datetime = itered_datetime + Duration::seconds(per_sec);
        all_datetime_vec.push(itered_datetime);
        if itered_datetime >= end_datetime {
            break
        }
    }

    // extract_workdays_bool_vec
    let start_date = start_datetime.date();
    let end_date = end_datetime.date();
    let all_workdays_set: HashSet<NaiveDate> = get_workdays(start_date, end_date, Closed::Both).iter().cloned().collect();

    let true_workdays_datetime: Vec<NaiveDateTime> = all_datetime_vec.iter().cloned()
    .filter(|x|{all_workdays_set.contains(&x.date())}).collect();

    let extracted_workdays_datetime_bool: Vec<bool> = extract_workdays_bool(&all_datetime_vec);
    let extracted_workdays_datetime: Vec<NaiveDateTime> = all_datetime_vec.iter().cloned().enumerate()
    .filter(|(i,_x)|{extracted_workdays_datetime_bool[*i]}).map(|(_i,x)|{x}).collect();
    assert_eq!(extracted_workdays_datetime, true_workdays_datetime);

    // extract_intraday_bool_vec
    let intraday_borders = INTRADAY_BORDERS.read().unwrap();
    let true_intraday_datetime: Vec<NaiveDateTime> = all_datetime_vec.iter().cloned()
    .filter(|x|{
        intraday_borders.iter().map(|y|{(y.start<=x.time()) & (x.time()<y.end)})
        .reduce(|a,b|{a | b}).unwrap()
    }).collect();

    let extracted_intraday_datetime_bool: Vec<bool> = extract_intraday_bool(&all_datetime_vec);
    let extracted_intraday_datetime: Vec<NaiveDateTime> = all_datetime_vec.iter().cloned().enumerate()
    .filter(|(i,_x)|{extracted_intraday_datetime_bool[*i]}).map(|(_i,x)|{x}).collect();
    assert_eq!(extracted_intraday_datetime, true_intraday_datetime);

    // extract_workdays_intraday_bool_vec
    let true_workdays_intraday_datetime: Vec<NaiveDateTime> = all_datetime_vec.iter().cloned()
    .filter(|x|{all_workdays_set.contains(&x.date())})
    .filter(|x|{
        intraday_borders.iter().map(|y|{(y.start<=x.time()) & (x.time()<y.end)})
        .reduce(|a,b|{a | b}).unwrap()
    }).collect();

    let extracted_workdays_intraday_datetime_bool: Vec<bool> = extract_workdays_intraday_bool(&all_datetime_vec);
    let extracted_workdays_intraday_datetime: Vec<NaiveDateTime> = all_datetime_vec.iter().cloned().enumerate()
    .filter(|(i,_x)|{extracted_workdays_intraday_datetime_bool[*i]}).map(|(_i,x)|{x}).collect();
    assert_eq!(extracted_workdays_intraday_datetime, true_workdays_intraday_datetime);

}

#[test]
fn related_intraday() {
    // check_workday_intraday
    assert!(check_workday_intraday(NaiveDate::from_ymd(2021,1,4).and_hms(10, 0, 0)));
    assert!(!check_workday_intraday(NaiveDate::from_ymd(2021,1,1).and_hms(10, 0, 0)));
    assert!(!check_workday_intraday(NaiveDate::from_ymd(2021, 1, 4).and_hms(0, 0, 0)));

    // get_next_border_workday_intraday
    let next_border_tuple = get_next_border_workday_intraday(NaiveDate::from_ymd(2021, 1, 1).and_hms(0, 0, 0));
    assert_eq!(next_border_tuple, (NaiveDate::from_ymd(2021, 1, 4).and_hms(9, 0, 0), "border_start"));
    let next_border_tuple = get_next_border_workday_intraday(NaiveDate::from_ymd(2021, 1, 4).and_hms(9, 0, 0));
    assert_eq!(next_border_tuple, (NaiveDate::from_ymd(2021, 1, 4).and_hms(11, 30, 0), "border_end"));
    let next_border_tuple = get_next_border_workday_intraday(NaiveDate::from_ymd(2021, 1, 4).and_hms(11, 30, 0));
    assert_eq!(next_border_tuple, (NaiveDate::from_ymd(2021, 1, 4).and_hms(12, 30, 0), "border_start"));

    // get_previous_border_workday_intraday
    let previous_border_tuple = get_previous_border_workday_intraday(NaiveDate::from_ymd(2021, 1, 1).and_hms(10, 0, 0), true);
    assert_eq!(previous_border_tuple, (NaiveDate::from_ymd(2020, 12, 31).and_hms(15, 0, 0), "border_end"));
    let previous_border_tuple = get_previous_border_workday_intraday(NaiveDate::from_ymd(2020, 12, 31).and_hms(12, 30, 0), true);
    assert_eq!(previous_border_tuple, (NaiveDate::from_ymd(2020, 12, 31).and_hms(11, 30, 0), "border_end"));
    let previous_border_tuple = get_previous_border_workday_intraday(NaiveDate::from_ymd(2020, 12, 31).and_hms(15, 0, 0), false);
    assert_eq!(previous_border_tuple, (NaiveDate::from_ymd(2020, 12, 31).and_hms(15, 0, 0), "border_end"));
    let previous_border_tuple = get_previous_border_workday_intraday(NaiveDate::from_ymd(2020, 12, 31).and_hms(15, 0, 0), true);
    assert_eq!(previous_border_tuple, (NaiveDate::from_ymd(2020, 12, 31).and_hms(12, 30, 0), "border_start"));

    // get_near_workday_intraday
    let near_workday_intraday_tuple = get_near_workday_intraday(NaiveDate::from_ymd(2021, 1, 1).and_hms(10, 0, 0), true);
    assert_eq!(near_workday_intraday_tuple, (NaiveDate::from_ymd(2021, 1, 4).and_hms(9, 0, 0), "border_start"));
    let near_workday_intraday_tuple = get_near_workday_intraday(NaiveDate::from_ymd(2021, 1, 1).and_hms(10, 0, 0), false);
    assert_eq!(near_workday_intraday_tuple, (NaiveDate::from_ymd(2020, 12, 31).and_hms(15, 0, 0), "border_end"));

    // add_workday_intraday_datetime
    let start_datetime = NaiveDate::from_ymd(2021, 1, 1).and_hms(0, 0, 0);
    let end_datetime = NaiveDate::from_ymd(2022, 1, 1).and_hms(0, 0, 0);
    let per_sec = 30*60; // 30分
    let mut all_datetime_vec: Vec<NaiveDateTime> = [].to_vec();
    let mut itered_datetime = start_datetime.clone();
    loop {
        itered_datetime = itered_datetime + Duration::seconds(per_sec);
        all_datetime_vec.push(itered_datetime);
        if itered_datetime >= end_datetime {
            break
        }
    }

    let extracted_workdays_intraday_datetime_bool: Vec<bool> = extract_workdays_intraday_bool(&all_datetime_vec);
    let extracted_workdays_intraday_datetime: Vec<NaiveDateTime> = all_datetime_vec.iter().cloned().enumerate()
    .filter(|(i,_x)|{extracted_workdays_intraday_datetime_bool[*i]}).map(|(_i,x)|{x}).collect();

    let (first_workday_intraday_datetime, _) = get_next_border_workday_intraday(start_datetime);
    let mut added_workday_intraday_datetime: Vec<NaiveDateTime> = [first_workday_intraday_datetime].to_vec();

    for i in 1..extracted_workdays_intraday_datetime.len() {
        added_workday_intraday_datetime.push(
            add_workday_intraday_datetime(start_datetime, Duration::seconds(i as i64 * per_sec))
        );
    }
    assert_eq!(added_workday_intraday_datetime, extracted_workdays_intraday_datetime);

    let mut subed_workday_intraday_datetime: Vec<NaiveDateTime> = Vec::new();

    for i in 1..extracted_workdays_intraday_datetime.len()+1 {
        subed_workday_intraday_datetime.push(
            sub_workday_intraday_datetime(end_datetime, Duration::seconds(i as i64 * per_sec))
        );
    }
    let subed_workday_intraday_datetime: Vec<NaiveDateTime> = subed_workday_intraday_datetime.iter().cloned().rev().collect();
    assert_eq!(subed_workday_intraday_datetime, extracted_workdays_intraday_datetime);

    // get_timedelta_workdays_intraday
    let start_datetime = NaiveDate::from_ymd(2021, 1, 4).and_hms(9, 0, 0);
    let end_datetime = NaiveDate::from_ymd(2021, 12, 31).and_hms(9, 0, 0);
    let delta_time = get_timedelta_workdays_intraday(start_datetime, end_datetime);
    assert_eq!(add_workday_intraday_datetime(start_datetime, delta_time), end_datetime);
    assert_eq!(sub_workday_intraday_datetime(end_datetime, delta_time), start_datetime);
}