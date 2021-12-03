use std::collections::HashSet;

use chrono::{NaiveDate, Duration, NaiveDateTime, Weekday, NaiveTime};

extern crate rs_workdays;
use rs_workdays::workdays::{get_workdays, check_workday, get_next_workday, get_previous_workday};
use rs_workdays::workdays::{get_near_workday, get_next_workdays_number, get_previous_workdays_number, get_workdays_number};

use rs_workdays::intraday::{check_workday_intraday, get_next_border_workday_intraday, get_previous_border_workday_intraday, get_near_workday_intraday};
use rs_workdays::intraday::{add_workday_intraday_datetime, sub_workday_intraday_datetime, get_timedelta_workdays_intraday};
use rs_workdays::extract::{extract_workdays_bool_vec, extract_intraday_bool_vec, extract_workdays_intraday_bool_vec};

//use rs_workdays::global::{set_holidays_csv};
use rs_workdays::global::{set_one_holiday_weekday_set, set_intraday_borders, TimeBorder};

fn main() {
    println!("program start");
    //println!("range_holidays_vec: {:?}", RANGE_HOLIDAYS_VEC.read().unwrap()); // グローバル変数の処理
    //println!("all_holidays_vec: {:?}", all_holidays_vec);

    //set_holidays_csv("source/holiday_naikaku.csv".to_string(), 2016, 2021);

    // get_workdays
    let workday_start_date = NaiveDate::from_ymd(2021,1,1);
    let workday_end_date = NaiveDate::from_ymd(2021,2,1);
    let workdays_vec = get_workdays(workday_start_date, workday_end_date, "left");
    println!("workdays_vec: {:?}", workdays_vec);

    // check_workday
    let select_date = NaiveDate::from_ymd(2021,1,1);
    let is_workday = check_workday(select_date);
    println!("{:?} is_workday: {:?}",select_date, is_workday);

    // get_next_workday
    let select_date = NaiveDate::from_ymd(2021,1,1);
    let next_workday = get_next_workday(select_date, 6);
    println!("next workday of {:?} is {:?}", select_date, next_workday);

    // get_previous_workday
    let select_date = NaiveDate::from_ymd(2021,1,12);
    let previous_workday = get_previous_workday(select_date, 6);
    println!("previous workday of {:?} is {:?}", select_date, previous_workday);

    // get_near_workday
    let select_date = NaiveDate::from_ymd(2021, 1, 1);
    let near_workday = get_near_workday(select_date, true);
    println!("near workday (after) of {:?} is {:?}", select_date, near_workday);

    // get_near_workday
    let select_date = NaiveDate::from_ymd(2021, 1, 1);
    let near_workday = get_near_workday(select_date, false);
    println!("near workday (before) of {:?} is {:?}", select_date, near_workday);
   
    // get_next_workdays_number
    let start_date = NaiveDate::from_ymd(2021, 1, 1);
    let workdays_vec = get_next_workdays_number(start_date, 19);
    println!("workdays_vec: {:?}", workdays_vec);

    // get_previous_workdays_number
    let start_date = NaiveDate::from_ymd(2021,1,29);
    let mut workdays_vec = get_previous_workdays_number(start_date, 19);
    workdays_vec.sort();
    println!("workdays_vec: {:?}", workdays_vec);

    // get_workdays_number
    let start_date = NaiveDate::from_ymd(2021, 1, 1);
    let workdays_vec = get_workdays_number(start_date, 19);
    println!("workdays_vec: {:?}", workdays_vec);

    // check_workday_intraday
    let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(10,0,0);
    let is_workday_intraday = check_workday_intraday(select_datetime);
    println!("{:?} is workday and intraday: {:?}", select_datetime, is_workday_intraday);

    // get_next_border_workday_intraday
    let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
    let (next_border_datetime, border_symbol) = get_next_border_workday_intraday(select_datetime);
    println!("next_border_datetime: {:?}, border_symbol: {:?}", next_border_datetime, border_symbol);

    // get_previous_border_workday_intraday
    let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
    let (previous_border_datetime, border_symbol) = get_previous_border_workday_intraday(select_datetime, false);
    println!("previous_border_datetime: {:?}, border_symbol: {:?}", previous_border_datetime, border_symbol);

    let select_datetime = NaiveDate::from_ymd(2021,1,4).and_hms(15,0,0);
    let (previous_border_datetime, border_symbol) = get_previous_border_workday_intraday(select_datetime, false);
    println!("previous_border_datetime: {:?}, border_symbol: {:?}", previous_border_datetime, border_symbol);

    let select_datetime = NaiveDate::from_ymd(2021,1,4).and_hms(15,0,0);
    let (previous_border_datetime, border_symbol) = get_previous_border_workday_intraday(select_datetime, true);
    println!("previous_border_datetime: {:?}, border_symbol: {:?}", previous_border_datetime, border_symbol);

    // get_near_workday_intraday
    let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
    let (near_workday_intraday_datetime, border_symbol) = get_near_workday_intraday(select_datetime, true);
    println!("near_workday_intraday_datetime: {:?}, border_symbol: {:?}", near_workday_intraday_datetime, border_symbol);

    let select_datetime = NaiveDate::from_ymd(2021,1,4).and_hms(10,0,0);
    let (near_workday_intraday_datetime, border_symbol) = get_near_workday_intraday(select_datetime, true);
    println!("near_workday_intraday_datetime: {:?}, border_symbol: {:?}", near_workday_intraday_datetime, border_symbol);  

    let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
    let (near_workday_intraday_datetime, border_symbol) = get_near_workday_intraday(select_datetime, false);
    println!("near_workday_intraday_datetime: {:?}, border_symbol: {:?}", near_workday_intraday_datetime, border_symbol);  

    // add_workday_intraday_datetime
    let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
    let add_duration = Duration::hours(2);
    let added_workday_intraday_datetime = add_workday_intraday_datetime(select_datetime, add_duration);
    println!("add_workday_intraday_datetime: {:?} + {:?} = {:?}", select_datetime, add_duration, added_workday_intraday_datetime);

    let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
    let sub_duration = Duration::hours(2);
    let subed_workday_intraday_datetime = sub_workday_intraday_datetime(select_datetime, sub_duration);
    println!("sub_workday_intraday_datetime: {:?} + {:?} = {:?}", select_datetime, sub_duration, subed_workday_intraday_datetime);

    //get_timedelta_workdays_intraday
    let start_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
    let end_datetime = NaiveDate::from_ymd(2021,1,4).and_hms(15,0,0);
    let span_duration = get_timedelta_workdays_intraday(start_datetime, end_datetime);
    println!("{:?} and {:?} timedelta: {:?}", start_datetime, end_datetime, span_duration);

    //extract_workdays_bool_vec
    let start_datetime_timestamp: i64 = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0).timestamp();
    let add_sec: i64 = 3600; // 1時間
    let datetime_vec: Vec<NaiveDateTime> = vec![0;100].iter().cloned().enumerate()
    .map(|(i,_x)| {NaiveDateTime::from_timestamp(start_datetime_timestamp+ (i as i64) *add_sec, 0)}).collect();
    let bool_vec: Vec<bool> = extract_workdays_bool_vec(&datetime_vec);
    let extracted_workdays_datetime: Vec<NaiveDateTime> = datetime_vec.iter().cloned().enumerate()
    .filter(|(i,_x)|{bool_vec[*i]}).map(|(_i,x)|{x}).collect();
    println!("extrated workdays datetime: {:?}", extracted_workdays_datetime);

    //extract_intraday_bool_vec;
    let start_datetime_timestamp: i64 = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0).timestamp();
    let add_sec: i64 = 3600; // 1時間
    let datetime_vec: Vec<NaiveDateTime> = vec![0;100].iter().cloned().enumerate()
    .map(|(i,_x)| {NaiveDateTime::from_timestamp(start_datetime_timestamp+ (i as i64) *add_sec, 0)}).collect();
    let bool_vec: Vec<bool> = extract_intraday_bool_vec(&datetime_vec);
    let extracted_intraday_datetime: Vec<NaiveDateTime> = datetime_vec.iter().cloned().enumerate()
    .filter(|(i,_x)|{bool_vec[*i]}).map(|(_i,x)|{x}).collect();
    println!("extracted intraday datetime: {:?}", extracted_intraday_datetime);

    //extract_workdays_intraday_bool_vec
    let start_datetime_timestamp: i64 = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0).timestamp();
    let add_sec: i64 = 3600; // 1時間
    let datetime_vec: Vec<NaiveDateTime> = vec![0;100].iter().cloned().enumerate()
    .map(|(i,_x)| {NaiveDateTime::from_timestamp(start_datetime_timestamp+ (i as i64) *add_sec, 0)}).collect();
    let bool_vec: Vec<bool> = extract_workdays_intraday_bool_vec(&datetime_vec);
    let extracted_workdays_intraday_datetime: Vec<NaiveDateTime> = datetime_vec.iter().cloned().enumerate()
    .filter(|(i,_x)|{bool_vec[*i]}).map(|(_i,x)|{x}).collect();
    println!("extracted workday intraday datetime: {:?}", extracted_workdays_intraday_datetime);

    // 休日曜日と営業時間の変更
    let weekday_set: HashSet<Weekday> = [Weekday::Mon, Weekday::Tue].iter().cloned().collect();
    set_one_holiday_weekday_set(&weekday_set);

    let intraday_borders: Vec<TimeBorder> =[
        TimeBorder {start: NaiveTime::from_hms(8,0,0), end:NaiveTime::from_hms(10,0,0)}
    ].to_vec();
    set_intraday_borders(&intraday_borders);

    //extract_workdays_intraday_bool_vec
    let start_datetime_timestamp: i64 = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0).timestamp();
    let add_sec: i64 = 3600; // 1時間
    let datetime_vec: Vec<NaiveDateTime> = vec![0;100].iter().cloned().enumerate()
    .map(|(i,_x)| {NaiveDateTime::from_timestamp(start_datetime_timestamp+ (i as i64) *add_sec, 0)}).collect();
    let bool_vec: Vec<bool> = extract_workdays_intraday_bool_vec(&datetime_vec);
    let extracted_workdays_intraday_datetime: Vec<NaiveDateTime> = datetime_vec.iter().cloned().enumerate()
    .filter(|(i,_x)|{bool_vec[*i]}).map(|(_i,x)|{x}).collect();
    println!("changed extracted workday intraday datetime: {:?}", extracted_workdays_intraday_datetime);
}
