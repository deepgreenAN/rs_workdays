use std::sync::RwLock;
use std::path::Path;
use std::collections::HashSet;
use chrono::{NaiveDate, Weekday, NaiveTime};
use once_cell::sync::Lazy;
use anyhow::Context;
use crate::error::Error;


#[derive(Debug, Copy, Clone)]
pub struct TimeBorder {
    pub start: NaiveTime,
    pub end: NaiveTime
}

/// csvを読み込んで祝日のVecにする
/// # Argments
/// - path_str: csvファイルのパス
fn read_csv<P:AsRef<Path>>(source_path: P) -> Result<Vec<NaiveDate>, Error> {
    let source_path: &Path = source_path.as_ref();
    let source_path_str = source_path.to_str().context("cannot convert source path to string")?;

    let parse_from_str = NaiveDate::parse_from_str;
    let mut holiday_vec: Vec<NaiveDate> = Vec::new();

    let mut rdr = csv::Reader::from_path(source_path)
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


// グローバル変数
// 祝日データ
pub static RANGE_HOLIDAYS_VEC: Lazy<RwLock<Vec<NaiveDate>>> = Lazy::new(|| {
    let start_date = NaiveDate::from_ymd(2016, 1, 1);
    let end_date = NaiveDate::from_ymd(2021, 12, 31);
    let all_holidays_vec = read_csv("source/holidays.csv").unwrap_or([].to_vec());
    //let all_holidays_vec = [].to_vec();
    let range_holidays_vec: Vec<NaiveDate> = all_holidays_vec.iter().cloned().filter(|x| {(&start_date <= x) & (&end_date > x)}).collect(); // clonedで要素の所有権を渡していることに注意
    RwLock::new(range_holidays_vec)
});
// 休日曜日
pub static ONE_HOLIDAY_WEEKDAY_SET: Lazy<RwLock<HashSet<Weekday>>> = Lazy::new(|| {
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


/// csvを読み込んで利用できる休日の更新をする
/// # Argments
/// - path_str_vec: csvのパス
/// - start_year: 利用する開始年(その年の1月1日から)
/// - end_year: 利用する終了年(その年の12月31日まで)
pub fn set_holidays_csvs(path_str_vec: &Vec<String>, start_year: i32, end_year: i32) -> Result<(), Error>{
    let mut range_holidays_vec = RANGE_HOLIDAYS_VEC.write().unwrap();
    // 削除
    range_holidays_vec.clear();

    assert!(range_holidays_vec.is_empty());

    for path_str in path_str_vec.iter() {
        let file_holiday_vec = read_csv(path_str)?;
    
        // 代入
        let start_date = NaiveDate::from_ymd(start_year, 1, 1);
        let end_date = NaiveDate::from_ymd(end_year, 12, 31);
        let made_range_holiday: Vec<NaiveDate> = file_holiday_vec.iter().cloned().filter(|x| {(&start_date <= x) & (&end_date > x)}).collect();
    
        for range_holiday in made_range_holiday {
            range_holidays_vec.push(range_holiday);
        }
    }

    Ok(())
}

/// 祝日のベクターから祝日の更新をする
/// # Argments
/// - holidays_vec: 休日のベクター
/// - start_year: 利用する開始年(その年の1月1日から)
/// - end_year: 利用する終了年(その年の12月31日まで)
pub fn set_range_holidays(holidays_vec: &Vec<NaiveDate>, start_year: i32, end_year: i32) {
    let mut range_holidays_vec = RANGE_HOLIDAYS_VEC.write().unwrap();
    // 削除
    range_holidays_vec.clear();

    assert!(range_holidays_vec.is_empty());

    // 代入
    let start_date = NaiveDate::from_ymd(start_year, 1, 1);
    let end_date = NaiveDate::from_ymd(end_year, 12, 31);
    let made_range_holiday: Vec<NaiveDate> = holidays_vec.iter().cloned().filter(|x| {(&start_date <= x) & (&end_date > x)}).collect();

    for range_holiday in made_range_holiday {
        range_holidays_vec.push(range_holiday);
    }
}

/// 休日曜日の更新
/// # Argment
/// - new_one_holiday_weekday_set: 休日曜日のセット
pub fn set_one_holiday_weekday_set(new_one_holiday_weekday_set: &HashSet<Weekday>) {
    let mut one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.write().unwrap();
    // 削除
    one_holiday_weekday_set.clear();

    assert!(one_holiday_weekday_set.is_empty());

    // 代入
    for one_holiday_weekday in new_one_holiday_weekday_set.iter().cloned() {
        one_holiday_weekday_set.insert(one_holiday_weekday);
    }
}

/// 営業時間境界の更新
/// # Argment
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
}

/// 祝日データの取得
pub fn get_range_holidays_vec() -> Vec<NaiveDate> {
    let range_holidays_vec = RANGE_HOLIDAYS_VEC.read().unwrap();
    range_holidays_vec.iter().cloned().collect::<Vec<NaiveDate>>()
}

/// 祝日曜日データの取得
pub fn get_holiday_weekdays() -> HashSet<Weekday> {
    let holiday_weekdays = ONE_HOLIDAY_WEEKDAY_SET.read().unwrap();
    holiday_weekdays.iter().cloned().collect::<HashSet<Weekday>>()
}

/// 営業時間境界の取得
pub fn get_intraday_borders() -> Vec<TimeBorder> {
    let borders = INTRADAY_BORDERS.read().unwrap();
    borders.iter().cloned().collect::<Vec<TimeBorder>>()
}