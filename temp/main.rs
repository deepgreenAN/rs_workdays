use chrono::{NaiveDate, Duration, NaiveDateTime};

//mod global;
//mod workdays;
//mod intraday;

use rs_workdays::workdays::{get_workdays, check_workday, get_next_workday, get_previous_workday};
use rs_workdays::workdays::{get_near_workday, get_next_workdays_number, get_previous_workdays_number, get_workdays_number};

use rs_workdays::intraday::{check_workday_intraday, get_next_border_workday_intraday, get_previous_border_workday_intraday};
use rs_workdays::intraday::{add_workday_intraday_datetime, sub_workday_intraday_datetime, get_timedelta_workdays_intraday};
use rs_workdays::extract::{extract_workdays_bool_vec, extract_intraday_bool_vec, extract_workdays_intraday_bool_vec};


// 実行時間計測用
use std::time::{Instant};

fn main() {
    println!("program start");
    //println!("range_holidays_vec: {:?}", RANGE_HOLIDAYS_VEC.read().unwrap()); // グローバル変数の処理
    //println!("all_holidays_vec: {:?}", all_holidays_vec);

    // get_workdays
    let workday_start_date = NaiveDate::from_ymd(2021,1,1);
    let workday_end_date = NaiveDate::from_ymd(2021,2,1);

    // 時間計測の開始
    let start = Instant::now();

    let workdays_vec = get_workdays(
        workday_start_date,
        workday_end_date,
        &"left",
    );

    let end = start.elapsed();
    println!("get_workdays function time: {}.{:06}[s]", end.as_secs(), end.subsec_nanos() / 1_000_000);

    println!("workdays_vec: {:?}", workdays_vec);

    // check_workday
    let select_date = NaiveDate::from_ymd(2021,1,1);
    let is_workday = check_workday(select_date);
    println!("{:?} is_workday: {:?}",select_date, is_workday);

    // get_next_workday
    let select_date = NaiveDate::from_ymd(2021,1,1);
    // 時間計測の開始
    let start = Instant::now();
    let next_workday = get_next_workday(select_date, 6);
    let end = start.elapsed();
    println!("get_next_workday function time: {}.{:06}[s]", end.as_secs(), end.subsec_nanos() / 1_000_000);
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
    println!("near workday (after) of {:?} is {:?}", select_date, near_workday);
   
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
    // 時間計測の開始
    let start = Instant::now();
    let workdays_vec = get_workdays_number(start_date, 19);
    let end = start.elapsed();
    println!("get_workdays_number function time: {}.{:06}[s]", end.as_secs(), end.subsec_nanos() / 1_000_000);
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

    let select_datetime = NaiveDate::from_ymd(2021,1,4).and_hms(12,30,0);
    let (previous_border_datetime, border_symbol) = get_previous_border_workday_intraday(select_datetime, false);
    println!("previous_border_datetime: {:?}, border_symbol: {:?}", previous_border_datetime, border_symbol);

    // add_workday_intraday_datetime
    let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
    //let add_duration = Duration::hours(2);
    //let add_duration = Duration::weeks(48);
    let add_duration = Duration::days(51) + Duration::hours(1);
    // 時間計測の開始
    let start = Instant::now();
    let added_workday_intraday_datetime = add_workday_intraday_datetime(select_datetime, add_duration);
    let end = start.elapsed();
    println!("add_workday_intraday_datetime function time: {}.{:06}[s]", end.as_secs(), end.subsec_nanos() / 1_000_000);
    println!("add_workday_intraday_datetime: {:?} + {:?} = {:?}", select_datetime, add_duration, added_workday_intraday_datetime);

    let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
    //let sub_duration = Duration::hours(2);
    let sub_duration = Duration::weeks(48);
    let subed_workday_intraday_datetime = sub_workday_intraday_datetime(select_datetime, sub_duration);
    println!("sub_workday_intraday_datetime: {:?} + {:?} = {:?}", select_datetime, sub_duration, subed_workday_intraday_datetime);

    //get_timedelta_workdays_intraday
    let start_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
    let end_datetime = NaiveDate::from_ymd(2021,1,4).and_hms(15,0,0);
    let span_duration = get_timedelta_workdays_intraday(start_datetime, end_datetime);
    println!("{:?} and {:?} timedelta: {:?}", start_datetime, end_datetime, span_duration);

    //extract_workdays_bool_vec
    let start = Instant::now();
    let start_datetime_timestamp: i64 = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0).timestamp();
    let add_sec: i64 = 3600; // 1時間
    let datetime_vec: Vec<NaiveDateTime> = vec![0;10000].iter().cloned().enumerate()
    .map(|(i,_x)| {NaiveDateTime::from_timestamp(start_datetime_timestamp+ (i as i64) *add_sec, 0)}).collect();
    //println!("datetime_vec: {:?}", datetime_vec);
    let bool_vec: Vec<bool> = extract_workdays_bool_vec(&datetime_vec);
    let extracted_workdays_datetime: Vec<NaiveDateTime> = datetime_vec.iter().cloned().enumerate()
    .filter(|(i,_x)|{bool_vec[*i]}).map(|(_i,x)|{x}).collect();
    let end = start.elapsed();
    println!("extract_workday_bool_vec function time: {}.{:06}[s]", end.as_secs(), end.subsec_nanos() / 1_000_000);
    //println!("extrated workdays datetime: {:?}", extracted_workdays_datetime);

    //extract_intraday_bool_vec
    let start = Instant::now();
    let start_datetime_timestamp: i64 = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0).timestamp();
    let add_sec: i64 = 3600; // 1時間
    let datetime_vec: Vec<NaiveDateTime> = vec![0;1000].iter().cloned().enumerate()
    .map(|(i,_x)| {NaiveDateTime::from_timestamp(start_datetime_timestamp+ (i as i64) *add_sec, 0)}).collect();
    let bool_vec: Vec<bool> = extract_intraday_bool_vec(&datetime_vec);
    let extracted_intraday_datetime: Vec<NaiveDateTime> = datetime_vec.iter().cloned().enumerate()
    .filter(|(i,_x)|{bool_vec[*i]}).map(|(_i,x)|{x}).collect();
    let end = start.elapsed();
    println!("extract_intraday_bool_vec function time: {}.{:06}[s]", end.as_secs(), end.subsec_nanos() / 1_000_000);
    //println!("extracted intraday datetime: {:?}", extracted_intraday_datetime);

    //extract_workdays_intraday_bool_vec
    let start = Instant::now();
    let start_datetime_timestamp: i64 = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0).timestamp();
    let add_sec: i64 = 3600; // 1時間
    let datetime_vec: Vec<NaiveDateTime> = vec![0;1000].iter().cloned().enumerate()
    .map(|(i,_x)| {NaiveDateTime::from_timestamp(start_datetime_timestamp+ (i as i64) *add_sec, 0)}).collect();
    let bool_vec: Vec<bool> = extract_workdays_intraday_bool_vec(&datetime_vec);
    let extracted_workdays_intraday_datetime: Vec<NaiveDateTime> = datetime_vec.iter().cloned().enumerate()
    .filter(|(i,_x)|{bool_vec[*i]}).map(|(_i,x)|{x}).collect();
    let end = start.elapsed();
    println!("extract_workdays_intraday_bool_vec function time: {}.{:06}[s]", end.as_secs(), end.subsec_nanos() / 1_000_000);
    //println!("extracted intraday datetime: {:?}", extracted_workdays_intraday_datetime);
}