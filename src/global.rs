use std::sync::RwLock;
use std::path::Path;
use std::collections::HashSet;
use chrono::{NaiveDate, Datelike, Weekday, NaiveTime};
use once_cell::sync::Lazy;

#[cfg(feature = "source")]
use anyhow::Context;

use crate::error::Error;

/// 営業時間の境界
/// Fields
/// - start: 開始時間
/// - end: 終了時間
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeBorder {
    pub start: NaiveTime,
    pub end: NaiveTime
}



/// csvを読み込んで祝日のVecにする
/// Argment
/// - path_str: csvファイルのパス
#[cfg(feature = "source")]
fn read_csv<P:AsRef<Path>>(source_path: P) -> Result<Vec<NaiveDate>, Error> {
    let source_path: &Path = source_path.as_ref();
    let source_path_str = source_path.to_str().context("cannot convert source path to string")?;

    let parse_from_str = NaiveDate::parse_from_str;
    let mut holiday_vec: Vec<NaiveDate> = Vec::new();

    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path(source_path)
        .map_err(|_|{Error::ReadCsvError{path_str: source_path_str.to_string()}})?;
    for result in rdr.records() {
        let record = result
            .map_err(|_|{Error::ReadCsvError{path_str: source_path_str.to_string()}})?;
        holiday_vec.push(
            parse_from_str(&record[0], "%Y-%m-%d")
            .map_err(|_|{Error::ParseDateError{date_str: record[0].into()}})?
        );
    }
    Ok(holiday_vec)
}


#[cfg(not(feature = "source"))]
fn read_csv<P:AsRef<Path>>(_: P) -> Result<Vec<NaiveDate>, Error> {
    Ok([].to_vec())
}

// グローバル変数
// 祝日データ
pub static RANGE_HOLIDAYS: Lazy<RwLock<Vec<NaiveDate>>> = Lazy::new(|| {
    let start_year = 2016_i32;
    let end_year = 2025_i32;

    let all_holidays_vec = read_csv("source/holidays.csv").unwrap_or([].to_vec());
    
    let range_holidays_set: HashSet<NaiveDate> = all_holidays_vec.into_iter().filter(|x| {
        (start_year <= x.year()) & (end_year >= x.year())
    }).collect();  // setにして重複を削除
    let mut range_holidays_vec: Vec<NaiveDate> = range_holidays_set.into_iter().collect();
    range_holidays_vec.sort();
    RwLock::new(range_holidays_vec)
});
// 休日曜日
pub static HOLIDAY_WEEKDAYS: Lazy<RwLock<HashSet<Weekday>>> = Lazy::new(|| {
    RwLock::new([Weekday::Sat, Weekday::Sun].iter().cloned().collect())
});
// 営業時間の境界
pub static INTRADAY_BORDERS: Lazy<RwLock<Vec<TimeBorder>>> = Lazy::new(|| {
    RwLock::new([
        TimeBorder {start: NaiveTime::from_hms(9,0,0), end: NaiveTime::from_hms(11,30,0)},
        TimeBorder {start: NaiveTime::from_hms(12,30,0), end: NaiveTime::from_hms(15,0,0)},
    ].iter().cloned().collect())
});
// どれとも重ならない日
pub static IMPOSSIBLE_DATE_1: Lazy<NaiveDate> = Lazy::new(||{ NaiveDate::from_ymd(2100,1,1) });  // どれとも重ならないような日にち
pub static IMPOSSIBLE_DATE_2: Lazy<NaiveDate> = Lazy::new(||{ NaiveDate::from_ymd(2101,1,1) });  // どれとも重ならないような日にち


/// csvを読み込んで利用できる祝日の更新をする
/// Argments
/// - path_str_vec: csvのパス
/// - start_year: 利用する開始年(その年の1月1日から)
/// - end_year: 利用する終了年(その年の12月31日まで)
pub fn set_holidays_csvs(path_str_vec: &Vec<String>, start_year: i32, end_year: i32) -> Result<(), Error>{
    let mut range_holidays_vec = RANGE_HOLIDAYS.write().unwrap();
    // 削除
    range_holidays_vec.clear();

    assert!(range_holidays_vec.is_empty());

    // 重複が無いようにsetを用意
    let mut range_holidays_set: HashSet<NaiveDate> = HashSet::new(); 

    for path_str in path_str_vec.iter() {
        let file_holiday_vec = read_csv(path_str)?;
    
        let made_range_holidays_vec: Vec<NaiveDate> = file_holiday_vec.into_iter().filter(|holiday| {
            (start_year <= holiday.year()) & (end_year >= holiday.year())
        }).collect();
        made_range_holidays_vec.into_iter().for_each(|range_holiday|{ range_holidays_set.insert(range_holiday); });
    }

    // 代入
    range_holidays_set.into_iter().for_each(|range_holiday|{ range_holidays_vec.push(range_holiday) });
    range_holidays_vec.sort();
    Ok(())
}

/// 祝日のvecから祝日の更新をする
/// Argments
/// - holidays_vec: 休日のベクター
/// - start_year: 利用する開始年(その年の1月1日から)
/// - end_year: 利用する終了年(その年の12月31日まで)
pub fn set_range_holidays(holidays_vec: &Vec<NaiveDate>, start_year: i32, end_year: i32) {
    let mut range_holidays_vec = RANGE_HOLIDAYS.write().unwrap();
    // 削除
    range_holidays_vec.clear();
    assert!(range_holidays_vec.is_empty());

    // 重複が無いようにsetを用意
    let mut range_holidays_set: HashSet<NaiveDate> = HashSet::new();

    let made_range_holidays_vec: Vec<NaiveDate> = holidays_vec.iter().cloned().filter(|holiday| {
        (start_year <= holiday.year()) & (end_year > holiday.year())
    }).collect();

    made_range_holidays_vec.into_iter().for_each(|range_holiday|{ range_holidays_set.insert(range_holiday); });

    // 代入
    range_holidays_set.into_iter().for_each(|range_holiday|{ range_holidays_vec.push(range_holiday) });
    range_holidays_vec.sort();
}

/// 祝日のvecから祝日の追加をする
/// Argments
/// - holidays_vec: 休日のベクター
/// - start_year: 利用する開始年(その年の1月1日から)
/// - end_year: 利用する終了年(その年の12月31日まで)
pub fn add_range_holidays(holidays_vec: &Vec<NaiveDate>, start_year: i32, end_year: i32) {
    let mut range_holidays_vec = RANGE_HOLIDAYS.write().unwrap();

    // 重複が無いようにsetを用意
    let mut range_holidays_set: HashSet<NaiveDate> = range_holidays_vec.iter().cloned().collect();

    let made_range_holidays_vec: Vec<NaiveDate> = holidays_vec.iter().cloned().filter(|holiday| {
        (start_year <= holiday.year()) & (end_year > holiday.year())
    }).collect();

    made_range_holidays_vec.into_iter().for_each(|range_holiday|{ range_holidays_set.insert(range_holiday); });

    // 削除
    range_holidays_vec.clear();
    assert!(range_holidays_vec.is_empty());

    // 代入
    range_holidays_set.into_iter().for_each(|range_holiday|{ range_holidays_vec.push(range_holiday) });
    range_holidays_vec.sort();
}

/// 休日曜日の更新
/// Argment
/// - new_holiday_weekdays: 休日曜日のセット
pub fn set_holiday_weekdays(new_holiday_weekdays: &HashSet<Weekday>) {
    let mut holiday_weekdays = HOLIDAY_WEEKDAYS.write().unwrap();
    // 削除
    holiday_weekdays.clear();

    assert!(holiday_weekdays.is_empty());

    // 代入
    for holiday_weekday in new_holiday_weekdays.iter().cloned() {
        holiday_weekdays.insert(holiday_weekday);
    }
}

/// 営業時間境界の更新
/// Argment
/// - new_intrada_borders: 営業時間境界のベクター
pub fn set_intraday_borders(new_intraday_borders: &Vec<TimeBorder>) {
    let mut intraday_borders = INTRADAY_BORDERS.write().unwrap();

    // 削除
    intraday_borders.clear();

    assert!(intraday_borders.is_empty());

    // 代入
    for one_intraday_border in new_intraday_borders.iter().cloned(){
        intraday_borders.push(one_intraday_border);
    }
    intraday_borders.sort();
}

/// 祝日データの取得
/// Return
/// - 祝日のvec
pub fn get_range_holidays() -> Vec<NaiveDate> {
    let range_holidays_vec = RANGE_HOLIDAYS.read().unwrap();
    range_holidays_vec.iter().cloned().collect::<Vec<NaiveDate>>()
}

/// 休日曜日データの取得
/// Return
/// - 休日曜日のset
pub fn get_holiday_weekdays() -> HashSet<Weekday> {
    let holiday_weekdays = HOLIDAY_WEEKDAYS.read().unwrap();
    holiday_weekdays.iter().cloned().collect::<HashSet<Weekday>>()
}

/// 営業時間境界の取得
/// Return
/// - 営業時間境界のvec
pub fn get_intraday_borders() -> Vec<TimeBorder> {
    let borders = INTRADAY_BORDERS.read().unwrap();
    borders.iter().cloned().collect::<Vec<TimeBorder>>()
}