use chrono::{NaiveDateTime};

use super::global::{INTRADAY_BORDERS, DEFAULT_DATE_1};
use super::workdays::{get_workdays};

/// Vec<NaiveDatetime>から営業日のものをboolとして抽出
/// # Argments
/// - datetime_vec: 抽出したい日時のベクター
/// 
/// # Returns
/// ブールのベクター
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDateTime, NaiveDate};
/// use rs_workdays::extract::*;
/// let start_datetime_timestamp: i64 = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0).timestamp();
/// let add_sec: i64 = 3600; // 1時間
/// let datetime_vec: Vec<NaiveDateTime> = vec![0;100].iter().cloned().enumerate()
/// .map(|(i,_x)| {NaiveDateTime::from_timestamp(start_datetime_timestamp+ (i as i64) *add_sec, 0)}).collect();
/// let bool_vec: Vec<bool> = extract_workdays_bool_vec(&datetime_vec);
/// let extracted_workdays_datetime: Vec<NaiveDateTime> = datetime_vec.iter().cloned().enumerate()
/// .filter(|(i,_x)|{bool_vec[*i]}).map(|(_i,x)|{x}).collect();
/// println!("extrated workdays datetime: {:?}", extracted_workdays_datetime);
/// ~~~~
/// 
/// extrated workdays datetime: [2021-01-04T00:00:00, 2021-01-04T01:00:00, 2021-01-04T02:00:00, 2021-01-04T03:00:00, 2021-01-04T04:00:00,
///  2021-01-04T05:00:00, 2021-01-04T06:00:00, 2021-01-04T07:00:00, 2021-01-04T08:00:00, 2021-01-04T09:00:00, 2021-01-04T10:00:00,
///  2021-01-04T11:00:00, 2021-01-04T12:00:00, 2021-01-04T13:00:00, 2021-01-04T14:00:00, 2021-01-04T15:00:00, 2021-01-04T16:00:00,
///  2021-01-04T17:00:00, 2021-01-04T18:00:00, 2021-01-04T19:00:00, 2021-01-04T20:00:00, 2021-01-04T21:00:00, 2021-01-04T22:00:00,
///  2021-01-04T23:00:00, 2021-01-05T00:00:00, 2021-01-05T01:00:00, 2021-01-05T02:00:00, 2021-01-05T03:00:00]
pub fn extract_workdays_bool_vec(datetime_vec:&Vec<NaiveDateTime>) -> Vec<bool> {
    let mut bool_vec = vec![false;datetime_vec.len()];
    let first_date = datetime_vec.first().unwrap().date();
    let last_date = datetime_vec.last().unwrap().date();

    let workdays_vec = get_workdays(first_date, last_date, "not");
    let mut workdays_iter = workdays_vec.iter();

    let mut one_workday = workdays_iter.next().unwrap_or(&DEFAULT_DATE_1);
    let mut now_date = first_date;
    let mut is_end_today = false;

    // 最初はここで判定
    if one_workday==&now_date {  // その日が営業日の場合
        one_workday = workdays_iter.next().unwrap_or(&DEFAULT_DATE_1);  // onw_workdayをインクリメント
    } else {  // その日が営業日でない場合
        is_end_today = true; // その日が終了
    }
    
    for (i, datetime) in datetime_vec.iter().enumerate() {
        // now_dateのインクリメント
        let date = datetime.date();
        if now_date < date { // 日付が変わるとき
            now_date = date;
            if one_workday==&now_date {  // その日が営業日の場合
                is_end_today = false; // フラッグを初期化                
                one_workday = workdays_iter.next().unwrap_or(&DEFAULT_DATE_1);  // onw_workdayをインクリメント

            } else {  // その日が営業日でない場合
                is_end_today = true; // その日が終了
            }

        }

        if is_end_today {  // その日が終了しているとき
            continue;
        }

        // bool_vecの変更
        bool_vec[i] = true;
    }
    return bool_vec;
}

/// Vec<NaiveDatetime>から営業時間のものをboolとして抽出
/// # Argments
/// - datetime_vec: 抽出したい日時のベクター
/// 
/// # Returns
/// ブールのベクター
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDateTime, NaiveDate};
/// use rs_workdays::extract::*;
/// let start_datetime_timestamp: i64 = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0).timestamp();
/// let add_sec: i64 = 3600; // 1時間
/// let datetime_vec: Vec<NaiveDateTime> = vec![0;100].iter().cloned().enumerate()
/// .map(|(i,_x)| {NaiveDateTime::from_timestamp(start_datetime_timestamp+ (i as i64) *add_sec, 0)}).collect();
/// let bool_vec: Vec<bool> = extract_intraday_bool_vec(&datetime_vec);
/// let extracted_intraday_datetime: Vec<NaiveDateTime> = datetime_vec.iter().cloned().enumerate()
/// .filter(|(i,_x)|{bool_vec[*i]}).map(|(_i,x)|{x}).collect();
/// println!("extracted intraday datetime: {:?}", extracted_intraday_datetime);
/// ~~~~
/// 
/// extracted intraday datetime: [2021-01-01T09:00:00, 2021-01-01T10:00:00, 2021-01-01T11:00:00, 2021-01-01T13:00:00, 2021-01-01T14:00:00,
///  2021-01-02T09:00:00, 2021-01-02T10:00:00, 2021-01-02T11:00:00, 2021-01-02T13:00:00, 2021-01-02T14:00:00, 2021-01-03T09:00:00,
///  2021-01-03T10:00:00, 2021-01-03T11:00:00, 2021-01-03T13:00:00, 2021-01-03T14:00:00, 2021-01-04T09:00:00, 2021-01-04T10:00:00,
///  2021-01-04T11:00:00, 2021-01-04T13:00:00, 2021-01-04T14:00:00]
pub fn extract_intraday_bool_vec(datetime_vec:&Vec<NaiveDateTime>) -> Vec<bool> {
    let intraday_borders_vec = INTRADAY_BORDERS.read().unwrap();
    let mut bool_vec = vec![false;datetime_vec.len()];
    
    let mut now_date = datetime_vec.first().unwrap().date();
    let mut borders_index: usize = 0;
    let mut is_end_today: bool = false;
    let mut is_end_intraday: bool = false;

    for (i, datetime) in datetime_vec.iter().enumerate() {
        // borders_indexのインクリメント, is_end_todayのフラッグを処理
        let date = datetime.date();
        let time = datetime.time();

        if date > now_date {  // 日付が変わるとき
            now_date = date;
            is_end_today = false;  // フラッグを初期化
        }

        if is_end_today {  // その日が終了しているとき
            continue;
        }

        if intraday_borders_vec[borders_index].end <= time {  // timeが終了境界を越えたとき
            borders_index += 1;
            is_end_intraday = true;  // その営業時間が終了

            if borders_index >= intraday_borders_vec.len() {  // borders_indexが境界の数と同じになったとき
                // その日は終了
                borders_index = 0;  // borders_indexを初期化
                is_end_today = true;  // その日が終了
            }

            if (intraday_borders_vec[borders_index].start <= time) & !is_end_today{// timeか境界内に入った場合
                is_end_intraday = false;  // フラッグを初期化
            }

        } else if intraday_borders_vec[borders_index].start <= time{ // timeが境界内の場合
            is_end_intraday = false;  // フラッグを初期化

        } else if intraday_borders_vec[borders_index].start > time {// timeが境界まえの場合
            is_end_intraday = true;  // 便宜的に営業時間が終了
        }

        if is_end_intraday {  // その営業時間が終了しているとき
            continue;
        }

        // bool_vecの変更
        bool_vec[i] = true;
    }

    return bool_vec;
}

/// Vec<NaiveDatetime>から営業日・営業時間のものをboolとして抽出
/// # Argments
/// - datetime_vec: 抽出したい日時のベクター
/// 
/// # Returns
/// ブールのベクター
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDateTime, NaiveDate};
/// use rs_workdays::extract::*;
/// let start_datetime_timestamp: i64 = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0).timestamp();
/// let add_sec: i64 = 3600; // 1時間
/// let datetime_vec: Vec<NaiveDateTime> = vec![0;100].iter().cloned().enumerate()
/// .map(|(i,_x)| {NaiveDateTime::from_timestamp(start_datetime_timestamp+ (i as i64) *add_sec, 0)}).collect();
/// let bool_vec: Vec<bool> = extract_workdays_intraday_bool_vec(&datetime_vec);
/// let extracted_workdays_intraday_datetime: Vec<NaiveDateTime> = datetime_vec.iter().cloned().enumerate()
/// .filter(|(i,_x)|{bool_vec[*i]}).map(|(_i,x)|{x}).collect();
/// println!("extracted workday intraday datetime: {:?}", extracted_workdays_intraday_datetime);
/// ~~~~
/// 
/// extracted workday intraday datetime: [2021-01-04T09:00:00, 2021-01-04T10:00:00, 2021-01-04T11:00:00, 2021-01-04T13:00:00,
///  2021-01-04T14:00:00]
pub fn extract_workdays_intraday_bool_vec(datetime_vec:&Vec<NaiveDateTime>) -> Vec<bool> {
    let mut bool_vec = vec![false;datetime_vec.len()];
    let first_date = datetime_vec.first().unwrap().date();
    let last_date = datetime_vec.last().unwrap().date();

    let workdays_vec = get_workdays(first_date, last_date, "not");
    let mut workdays_iter = workdays_vec.iter();

    let mut one_workday = workdays_iter.next().unwrap_or(&DEFAULT_DATE_1);
    let mut now_date = first_date;

    let intraday_borders_vec = INTRADAY_BORDERS.read().unwrap();
    
    let mut borders_index: usize = 0;
    let mut is_end_today: bool = false;
    let mut is_end_intraday: bool = false;

    // 最初はここで判定
    if one_workday==&now_date {  // その日が営業日の場合
        one_workday = workdays_iter.next().unwrap_or(&DEFAULT_DATE_1);  // onw_workdayをインクリメント
    } else {  // その日が営業日でない場合
        is_end_today = true; // その日が終了
    }

    for (i, datetime) in datetime_vec.iter().enumerate() {
        let date = datetime.date();
        let time = datetime.time();

        if now_date < date { // 日付が変わるとき
            now_date = date;
            if one_workday==&now_date {  // その日が営業日の場合
                is_end_today = false; // フラッグを初期化                
                one_workday = workdays_iter.next().unwrap_or(&DEFAULT_DATE_1);  // onw_workdayをインクリメント

            } else {  // その日が営業日でない場合
                is_end_today = true; // その日が終了
            }

        }

        if is_end_today {  // その日が終了しているとき
            continue;
        }

        if intraday_borders_vec[borders_index].end <= time {  // timeが終了境界を越えたとき
            borders_index += 1;  // borderをすすめる
            is_end_intraday = true;  // その営業時間が終了
            if borders_index >= intraday_borders_vec.len() {  // borders_indexが境界の数と同じになったとき
                // その日は終了
                borders_index = 0;  // borders_indexを初期化
                is_end_today = true;  // その日が終了
            }

            if (intraday_borders_vec[borders_index].start <= time) & !is_end_today{// timeか境界内に入った場合
                is_end_intraday = false;  // フラッグを初期化
            }

        } else if intraday_borders_vec[borders_index].start <= time{ // timeが境界内の場合
            is_end_intraday = false;  // フラッグを初期化
            
        } else if intraday_borders_vec[borders_index].start > time {// timeが境界まえの場合
            is_end_intraday = true;  // 便宜的に営業時間が終了
        }

        if is_end_intraday {  // その営業時間が終了しているとき
            continue;
        }

        // bool_vecの変更
        bool_vec[i] = true;
    }
    return bool_vec;
}
