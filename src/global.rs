use std::collections::HashSet;
use chrono::{NaiveDate, Weekday, NaiveTime};
use lazy_static::lazy_static;
use std::error::Error;
use std::sync::RwLock;

#[derive(Debug, Copy, Clone)]
pub struct TimeBorder {
    pub start: NaiveTime,
    pub end: NaiveTime
}

pub fn read_csv(path_str:String) -> Result<Vec<NaiveDate>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let parse_from_str = NaiveDate::parse_from_str;
    let mut holiday_vec: Vec<NaiveDate> = Vec::new();
    let mut rdr = csv::Reader::from_path(path_str)?;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        holiday_vec.push(parse_from_str(&record[0], "%Y-%m-%d")?);
    }
    Ok(holiday_vec)
}

//　グローバル変数

lazy_static! {
    pub static ref RANGE_HOLIDAYS_VEC: RwLock<Vec<NaiveDate>> = {
        let start_date = NaiveDate::from_ymd(2016, 1, 1);
        let end_date = NaiveDate::from_ymd(2021, 12, 31);
        let all_holidays_vec = read_csv("source/holiday_naikaku.csv".to_string()).unwrap_or([].to_vec());
        //let all_holidays_vec = [].to_vec();
        let range_holidays_vec: Vec<NaiveDate> = all_holidays_vec.iter().cloned().filter(|x| {(&start_date <= x) & (&end_date > x)}).collect(); // clonedで要素の所有権を渡していることに注意
        RwLock::new(range_holidays_vec)
    }; 
    pub static ref ONE_HOLIDAY_WEEKDAY_SET: RwLock<HashSet<Weekday>> = {
        RwLock::new([Weekday::Sat, Weekday::Sun].iter().cloned().collect())
    };
    pub static ref INTRADAY_BORDERS: RwLock<Vec<TimeBorder>> = {
        RwLock::new([
            TimeBorder {start: NaiveTime::from_hms(9,0,0), end: NaiveTime::from_hms(11,30,0)},
            TimeBorder {start: NaiveTime::from_hms(12,30,0), end: NaiveTime::from_hms(15,0,0)},
        ].iter().cloned().collect())
    };
    pub static ref DEFAULT_DATE_1: NaiveDate = NaiveDate::from_ymd(2100,1,1);  // どれとも重ならないような日にち
    pub static ref DEFAULT_DATE_2: NaiveDate = NaiveDate::from_ymd(2101,1,1);  // どれとも重ならないような日にち
}

pub fn set_holidays_csv(path_str:String, start_year: i32, end_year: i32) {
    let all_holiday_vec = read_csv(path_str).unwrap_or([].to_vec());
    let mut range_holidays_vec = RANGE_HOLIDAYS_VEC.write().unwrap();
    // 削除
    range_holidays_vec.clear();

    assert!(range_holidays_vec.is_empty());

    // 代入
    let start_date = NaiveDate::from_ymd(start_year, 1, 1);
    let end_date = NaiveDate::from_ymd(end_year, 12, 31);
    let made_range_holiday: Vec<NaiveDate> = all_holiday_vec.iter().cloned().filter(|x| {(&start_date <= x) & (&end_date > x)}).collect(); // clonedで要素の所有権を渡していることに注意

    for range_holiday in made_range_holiday {
        range_holidays_vec.push(range_holiday);
    }
}

pub fn set_range_holidays(holidays_vec: Vec<NaiveDate>, start_year: i32, end_year: i32) {
    let mut range_holidays_vec = RANGE_HOLIDAYS_VEC.write().unwrap();
    // 削除
    range_holidays_vec.clear();

    assert!(range_holidays_vec.is_empty());

    // 代入
    let start_date = NaiveDate::from_ymd(start_year, 1, 1);
    let end_date = NaiveDate::from_ymd(end_year, 12, 31);
    let made_range_holiday: Vec<NaiveDate> = holidays_vec.iter().cloned().filter(|x| {(&start_date <= x) & (&end_date > x)}).collect(); // clonedで要素の所有権を渡していることに注意

    for range_holiday in made_range_holiday {
        range_holidays_vec.push(range_holiday);
    }
}

pub fn set_one_holiday_weekday_set(py_one_holiday_weekday_set:HashSet<Weekday>) {
    let mut one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.write().unwrap();
    // 削除
    one_holiday_weekday_set.clear();

    assert!(one_holiday_weekday_set.is_empty());

    // 代入
    for one_holiday_weekday in py_one_holiday_weekday_set.iter() {
        one_holiday_weekday_set.insert(*one_holiday_weekday);
    }
}

pub fn set_intraday_borders(py_intraday_borders:Vec<TimeBorder>) {
    let mut intraday_borders = INTRADAY_BORDERS.write().unwrap();

    // 削除
    intraday_borders.clear();

    assert!(intraday_borders.is_empty());

    // 代入
    for one_intraday_border in py_intraday_borders.iter(){
        intraday_borders.push(*one_intraday_border);
    }
}
