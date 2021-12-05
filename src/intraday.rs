use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Duration};

use crate::global::{INTRADAY_BORDERS, TimeBorder};
use crate::workdays::{check_workday, get_next_workday, get_previous_workday, get_workdays, Closed};


/// select_datetimeが営業日・営業時間内であるかどうかを判定  
/// Argment
/// - select_datetime: 指定する日時
/// 
/// Return
/// 営業日・営業時間内であるかどうか
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDate};
/// use rs_workdays::intraday::*;
/// let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(10,0,0);
/// let is_workday_intraday = check_workday_intraday(select_datetime);
/// assert!(!is_workday_intraday);
/// ~~~~
pub fn check_workday_intraday(select_datetime: NaiveDateTime) -> bool {
    let intraday_borders_vec = INTRADAY_BORDERS.read().unwrap();
    let select_date = select_datetime.date();
    
    if check_workday(select_date) {// 営業日である
        let select_time = select_datetime.time();
        let is_intraday: bool =  intraday_borders_vec.iter().any(
            |x| {(x.start <= select_time) & (select_time < x.end)}
        );
        return is_intraday;
    } else {
        false
    }
}

/// 次の営業日・営業時間内のdatetimeをその状態とともに取得
/// Argment
/// - select_datetime: 指定する日時
/// 
/// Returns
/// - out_datetime: 次の営業日・営業時間内のdatetime
/// - 状態を示す文字列
///     - 'border_start': 営業時間の開始
///     - 'border_end': 営業時間の終了
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDate};
/// use rs_workdays::intraday::*;
/// let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
/// let (next_border_datetime, border_symbol) = get_next_border_workday_intraday(select_datetime);
/// assert_eq!((next_border_datetime, border_symbol), (NaiveDate::from_ymd(2021, 1, 4).and_hms(9,0,0), "border_start"));
/// ~~~~
pub fn get_next_border_workday_intraday(select_datetime: NaiveDateTime) -> (NaiveDateTime, &'static str) {
    let intraday_borders_vec = INTRADAY_BORDERS.read().unwrap();
    let select_date = select_datetime.date();

    if check_workday(select_date) {  // 営業日の場合
        let select_time = select_datetime.time();
        if check_workday_intraday(select_datetime) { // 営業日・営業時間の場合
            let bigger_border_ends: Vec<NaiveTime> = intraday_borders_vec.iter()
                .filter(|x| {select_time < x.end}).map(|x| {x.end}).collect();
            let out_time = bigger_border_ends.iter().min().unwrap();
            let out_datetime: NaiveDateTime = select_date.and_time(*out_time);
            return (out_datetime, "border_end");

        } else { // 営業時間でない場合
            let border_starts: Vec<NaiveTime> = intraday_borders_vec.iter()
                .map(|x| {x.start}).collect();
            let bigger_border_starts: Vec<NaiveTime> = border_starts.iter().cloned()
                .filter(|x| {x > &select_time}).collect();
         
            if bigger_border_starts.len() > 0 { // 指定時間より遅い営業時間の開始ボーダーがある場合
                let out_time = bigger_border_starts.iter().min().unwrap();
                let out_datetime: NaiveDateTime = select_date.and_time(*out_time);
                return (out_datetime, "border_start");
            } else { // 指定時間より遅い営業時間が存在しない場合
                let out_date: NaiveDate = get_next_workday(select_date, 1); // 次の営業日
                let out_time = border_starts.iter().min().unwrap();
                let out_datetime: NaiveDateTime = out_date.and_time(*out_time);
                return (out_datetime, "border_start");
            }
        }
    } else {  // 営業日でない場合
        let border_starts: Vec<NaiveTime> = intraday_borders_vec.iter()
            .map(|x| {x.start}).collect();
        let out_date: NaiveDate = get_next_workday(select_date, 1); // 次の営業日
        let out_time = border_starts.iter().min().unwrap();
        let out_datetime: NaiveDateTime = out_date.and_time(*out_time);
        return (out_datetime, "border_start");
    }
}

/// 前の営業日・営業時間内のdatetimeをその状態とともに取得
/// Argment
/// - select_datetime: 指定する日時
/// 
/// Returns
/// - out_datetime: 前の営業日・営業時間内のdatetime
/// - 状態を示す文字列
///     - 'border_start': 営業時間の開始
///     - 'border_end': 営業時間の終了
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDate};
/// use rs_workdays::intraday::*;
/// let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
/// let (previous_border_datetime, border_symbol) = get_previous_border_workday_intraday(select_datetime, false);
/// assert_eq!((previous_border_datetime, border_symbol), (NaiveDate::from_ymd(2020, 12, 31).and_hms(15,0,0), "border_end"));
/// 
/// let select_datetime = NaiveDate::from_ymd(2021,1,4).and_hms(15,0,0);
/// let (previous_border_datetime, border_symbol) = get_previous_border_workday_intraday(select_datetime, false);
/// assert_eq!((previous_border_datetime, border_symbol), (NaiveDate::from_ymd(2021, 1, 4).and_hms(15,0,0), "border_end"));
/// 
/// let select_datetime = NaiveDate::from_ymd(2021,1,4).and_hms(15,0,0);
/// let (previous_border_datetime, border_symbol) = get_previous_border_workday_intraday(select_datetime, true);
/// assert_eq!((previous_border_datetime, border_symbol), (NaiveDate::from_ymd(2021, 1, 4).and_hms(12,30,0), "border_start"));
/// ~~~~
pub fn get_previous_border_workday_intraday(select_datetime: NaiveDateTime, force_is_end:bool) -> (NaiveDateTime, &'static str) {
    let intraday_borders_vec = INTRADAY_BORDERS.read().unwrap();
    let select_date = select_datetime.date();
    let select_time = select_datetime.time();

    if check_workday(select_date) {  // 営業日の場合
        let border_starts: Vec<NaiveTime> = intraday_borders_vec.iter().map(|x|{x.start}).collect();
        let is_start_border: bool = border_starts.iter().any(
            |x|{x==&select_time}
        );
        if check_workday_intraday(select_datetime) & !is_start_border { // 営業時間であり，開始境界でない場合
            let smaller_border_starts: Vec<NaiveTime> = border_starts.iter().cloned().filter(|x|{x<&select_time}).collect();
            let out_time = smaller_border_starts.iter().max().unwrap();
            let out_datetime: NaiveDateTime = select_date.and_time(*out_time);
            return (out_datetime, "border_start");
        } else { // 営業時間でないか，開始境界である．
            let border_ends: Vec<NaiveTime> = intraday_borders_vec.iter().map(|x|{x.end}).collect();
            if force_is_end {  // 終了境界で次の開始境界に行くのを強制する
                let is_end_border: bool = border_ends.iter().any(
                    |x|{x==&select_time}
                );
                if is_end_border {  // 終了境界
                    let smaller_border_starts: Vec<NaiveTime> = border_starts.iter().cloned().filter(|x|{x<&select_time}).collect();
                    let out_time = smaller_border_starts.iter().max().unwrap();
                    let out_datetime: NaiveDateTime = select_date.and_time(*out_time);
                    return (out_datetime, "border_start");
                }
            }
            let smaller_border_ends: Vec<NaiveTime> = border_ends.iter().cloned().filter(|x|{x<=&select_time}).collect();
            if smaller_border_ends.len() > 0 {  // 指定時間より早い営業時間の終了ボーダーがある場合
                let out_time = smaller_border_ends.iter().max().unwrap();
                let out_datetime: NaiveDateTime = select_date.and_time(*out_time);
                return (out_datetime, "border_end");
            } else {  // 指定時間より早い営業時間が存在しない場合
                let out_date: NaiveDate = get_previous_workday(select_date, 1);
                let out_time = border_ends.iter().max().unwrap();
                let out_datetime: NaiveDateTime = out_date.and_time(*out_time);
                return (out_datetime, "border_end");
            }
        }
    } else {  // 営業日でない場合
        let border_ends: Vec<NaiveTime> = intraday_borders_vec.iter().map(|x|{x.end}).collect();
        let out_date = get_previous_workday(select_date, 1);
        let out_time = border_ends.iter().max().unwrap();
        let out_datetime: NaiveDateTime = out_date.and_time(*out_time);
        return (out_datetime, "border_end");
    }
}

/// 最近の営業日・営業時間内のdatetimeをその状態とともに取得．select_datetimeが営業日・営業時間内の場合そのまま返る．
/// Argments
/// - select_datetime: 指定する日時
/// - is_after: 後ろを探索するかどうか
/// 
/// Returns
/// - out_datetime: 前の営業日・営業時間内のdatetime
/// - 状態を示す文字列
///     - 'border_intra': 営業時間内
///     - 'border_start': 営業時間の開始
///     - 'border_end': 営業時間の終了
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDate};
/// use rs_workdays::intraday::*;
/// let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
/// let (near_workday_intraday_datetime, border_symbol) = get_near_workday_intraday(select_datetime, true);
/// assert_eq!((near_workday_intraday_datetime, border_symbol), (NaiveDate::from_ymd(2021, 1, 4).and_hms(9,0,0), "border_start"));
/// 
/// let select_datetime = NaiveDate::from_ymd(2021,1,4).and_hms(10,0,0);
/// let (near_workday_intraday_datetime, border_symbol) = get_near_workday_intraday(select_datetime, true);
/// assert_eq!((near_workday_intraday_datetime, border_symbol), (NaiveDate::from_ymd(2021, 1, 4).and_hms(10,0,0), "border_intra"));
/// 
/// let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
/// let (near_workday_intraday_datetime, border_symbol) = get_near_workday_intraday(select_datetime, false);
/// assert_eq!((near_workday_intraday_datetime, border_symbol), (NaiveDate::from_ymd(2020, 12, 31).and_hms(15,0,0), "border_end"));
/// ~~~~
pub fn get_near_workday_intraday(select_datetime: NaiveDateTime, is_after:bool) -> (NaiveDateTime, &'static str) {
    if check_workday_intraday(select_datetime) {
        return (select_datetime, "border_intra");
    } else {
        if is_after {
            return get_next_border_workday_intraday(select_datetime);
        } else {
            return get_previous_border_workday_intraday(select_datetime, false);
        }
    }
}

/// 営業日・営業時間を考慮しDateTimeを加算する．
/// Argments
/// - select_datetime: 指定する日時
/// - dela_time: 加算するDuration
/// 
/// Return
/// 加算された日時
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDate, Duration};
/// use rs_workdays::intraday::*;
/// let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
/// let add_duration = Duration::hours(2);
/// let added_workday_intraday_datetime = add_workday_intraday_datetime(select_datetime, add_duration);
/// assert_eq!(added_workday_intraday_datetime, NaiveDate::from_ymd(2021,1,4).and_hms(11,0,0));
/// ~~~~
pub fn add_workday_intraday_datetime(select_datetime: NaiveDateTime, delta_time: Duration) -> NaiveDateTime {
    let mut all_delta_time = delta_time;
    let intraday_borders_vec = INTRADAY_BORDERS.read().unwrap();

    // 営業時間一日分のdurationを作成
    let mut one_workday_delta_time = Duration::zero();

    for one_borders in intraday_borders_vec.iter() {
        one_workday_delta_time =  one_workday_delta_time + (one_borders.end - one_borders.start);
    } 

    let select_date = select_datetime.date();
    let (next_border_datetime, next_border_symbol) = get_next_border_workday_intraday(select_datetime);

    if check_workday_intraday(select_datetime) & (next_border_symbol=="border_end") {  //  select_datetimeが営業時間内にある場合
        let delta_select_date_intraday = next_border_datetime - select_datetime;

        if all_delta_time <= delta_select_date_intraday {  // 残りがその営業時間内以下の場合
            let out_datetime = select_datetime + all_delta_time;
            
            if all_delta_time==delta_select_date_intraday {  // 残りがその営業時間と同じ場合
                let (out_datetime, out_datetime_symbol) = get_next_border_workday_intraday(out_datetime);
                assert_eq!(out_datetime_symbol, "border_start");
                return out_datetime;
            }
            return out_datetime;
        } else {
            all_delta_time = all_delta_time - delta_select_date_intraday;
        }
    }

    if next_border_datetime.date() == select_date {  // その日の営業時間がまだ残っている場合
        let next_border_time = next_border_datetime.time();
        let bigger_intraday_borders: Vec<TimeBorder> = intraday_borders_vec.iter().cloned().filter(|x|{x.start >= next_border_time}).collect();

        for bigger_intraday_border in bigger_intraday_borders.iter(){
            let delta_select_date_intraday = bigger_intraday_border.end.signed_duration_since(bigger_intraday_border.start);
            if all_delta_time <= delta_select_date_intraday {  // 残りがその営業時間内以下の場合 
              let out_datetime_start = select_date.and_time(bigger_intraday_border.start);
              let out_datetime = out_datetime_start + all_delta_time;
              if all_delta_time == delta_select_date_intraday{  // 残りがその営業時間と同じ場合
                let (out_datetime, out_datetime_symbol) = get_next_border_workday_intraday(out_datetime);
                assert_eq!(out_datetime_symbol, "border_start");
                return out_datetime;
              }
              return out_datetime;
            } else {
                all_delta_time = all_delta_time - delta_select_date_intraday;   // 営業時間分を減らす
            }
        }
    }

    let mut add_day_number: i32 = 1;  // 追加が必要な営業日の日数

    loop {
        if all_delta_time <= one_workday_delta_time {
            break;
        }
        all_delta_time = all_delta_time - one_workday_delta_time;
        add_day_number += 1;
    }

    let out_date = get_next_workday(select_date, add_day_number);  // 出力する営業日

    for intraday_border in intraday_borders_vec.iter() {
        let delta_out_date_intraday = intraday_border.end.signed_duration_since(intraday_border.start);
        
        if all_delta_time <= delta_out_date_intraday {  // 残りがその営業時間内以下の場合
            let out_datetime_start = out_date.and_time(intraday_border.start);
            let out_datetime = out_datetime_start + all_delta_time;
            if all_delta_time == delta_out_date_intraday {  // 残りがその営業時間と同じ場合 
                let (out_datetime, out_datetime_symbol) = get_next_border_workday_intraday(out_datetime);   
                assert_eq!(out_datetime_symbol, "border_start");
                return out_datetime;
            }
            return out_datetime;
        } else {
            all_delta_time = all_delta_time - delta_out_date_intraday;  // 営業時間分を減らす
        }
    }

    return select_datetime;  // 計算に失敗している
}

/// 営業日・営業時間を考慮しDateTimeを減算する．
/// Argments
/// - select_datetime: 指定する日時
/// - dela_time: 加算するDuration
/// 
/// Return
/// 減算された日時
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDate, Duration};
/// use rs_workdays::intraday::*;
/// let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
/// let sub_duration = Duration::hours(2);
/// let subed_workday_intraday_datetime = sub_workday_intraday_datetime(select_datetime, sub_duration);
/// assert_eq!(subed_workday_intraday_datetime, NaiveDate::from_ymd(2020,12,31).and_hms(13,0,0));
/// ~~~~
pub fn sub_workday_intraday_datetime(select_datetime: NaiveDateTime, delta_time: Duration) -> NaiveDateTime {
    let mut all_delta_time = delta_time;
    let intraday_borders_vec = INTRADAY_BORDERS.read().unwrap();

    // 営業時間一日分のdurationを作成
    let mut one_workday_delta_time = Duration::zero();

    for one_borders in intraday_borders_vec.iter() {
        one_workday_delta_time =  one_workday_delta_time + (one_borders.end - one_borders.start);
    }

    let select_date = select_datetime.date();
    let (previous_border_datetime, previous_border_symbol) = get_previous_border_workday_intraday(select_datetime, false);
    if check_workday_intraday(select_datetime) & (previous_border_symbol=="border_start") {  // select_datetimeが営業時間内にある場合
        let delta_select_date_intraday = select_datetime - previous_border_datetime;
        if all_delta_time <= delta_select_date_intraday {  // 残りがその営業時間内以下の場合
            let out_datetime = select_datetime - all_delta_time;
            return out_datetime;
        } else {
            all_delta_time = all_delta_time - delta_select_date_intraday;  // 営業時間分を減らす
        }
    }

    if previous_border_datetime.date()==select_date{  // その日の営業時間がまだ残っている場合
        let previous_border_time = previous_border_datetime.time();
        let smaller_intraday_borders: Vec<TimeBorder> = intraday_borders_vec.iter().cloned().filter(|x|{x.end <= previous_border_time}).collect();
    
        for smaller_intraday_border in smaller_intraday_borders.iter() {
            let delta_select_date_intraday = smaller_intraday_border.end.signed_duration_since(smaller_intraday_border.start);
            if all_delta_time <= delta_select_date_intraday{  // 残りがその営業時間内以下の場合
                let out_datetime_end = select_date.and_time(smaller_intraday_border.end);
                let out_datetime = out_datetime_end - all_delta_time;
                return out_datetime;
            } else {
                all_delta_time = all_delta_time - delta_select_date_intraday;  // 営業時間分を減らす
            }
        }
    }

    let mut sub_day_number:i32 = 1;
    
    loop {
        if all_delta_time <= one_workday_delta_time{
            break;
        }
        all_delta_time = all_delta_time - one_workday_delta_time;
        sub_day_number += 1;
    }

    let out_date = get_previous_workday(select_date, sub_day_number);  // 出力する営業日

    for intraday_border in intraday_borders_vec.iter().rev() {  // 逆順
        let delta_out_date_intraday = intraday_border.end.signed_duration_since(intraday_border.start);
        if all_delta_time <= delta_out_date_intraday {
            let out_datetime_end = out_date.and_time(intraday_border.end);
            let out_datetime = out_datetime_end - all_delta_time;
            return out_datetime;
        } else {
            all_delta_time = all_delta_time - delta_out_date_intraday;  // 営業時間分を減らす
        }
    }

    return select_datetime;  // 計算に失敗している
}

/// start_datetimeからend_datetimeの営業日・営業時間を取得
/// Argments
/// - start_datetime: 開始日時
/// - end_datetime: 終了日時
/// 
/// Return
/// 営業日・営業時間のDuration
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDate, Duration};
/// use rs_workdays::intraday::*;
/// let start_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
/// let end_datetime = NaiveDate::from_ymd(2021,1,4).and_hms(15,0,0);
/// let span_duration = get_timedelta_workdays_intraday(start_datetime, end_datetime);
/// assert_eq!(span_duration, Duration::hours(5));
/// ~~~~
pub fn get_timedelta_workdays_intraday(start_datetime: NaiveDateTime, end_datetime: NaiveDateTime) -> Duration {
    let mut all_delta_time = Duration::zero();
    let intraday_borders_vec = INTRADAY_BORDERS.read().unwrap();

    // 営業時間一日分のdurationを作成
    let mut one_workday_delta_time = Duration::zero();

    for one_borders in intraday_borders_vec.iter() {
        one_workday_delta_time =  one_workday_delta_time + (one_borders.end - one_borders.start);
    }

    let start_date = start_datetime.date();
    let start_time = start_datetime.time();
    let end_date = end_datetime.date();
    let end_time = end_datetime.time();

    // start_dateについて
    if check_workday(start_date) {  // start_dateが営業日の場合
        if check_workday_intraday(start_datetime) {  // start_datetimeが営業時間内の場合
            let bigger_border_ends: Vec<NaiveTime> = intraday_borders_vec.iter()
            .filter(|x|{x.end > start_time}).map(|x|{x.end}).collect();
            let near_border_end = bigger_border_ends.iter().min().unwrap();
            all_delta_time = all_delta_time + near_border_end.signed_duration_since(start_time);
        }

        let bigger_intraday_borders: Vec<TimeBorder> = intraday_borders_vec.iter().cloned()
        .filter(|x|{x.start > start_time}).collect();

        for bigger_intraday_border in bigger_intraday_borders.iter() {
            all_delta_time = all_delta_time +  bigger_intraday_border.end.signed_duration_since(bigger_intraday_border.start);
        }
    }

    // 開始時刻から終了時刻までの営業日(開始・終了はふくまない)
    let workdays = get_workdays(start_date, end_date, Closed::Not);

    for _ in 0..workdays.len() {
        all_delta_time = all_delta_time + one_workday_delta_time;
    }

    // end_dateについて
    if check_workday(end_date) { // end_dateが営業日の場合
        if check_workday_intraday(end_datetime) {  // end_datetimeが営業時間内の場合
            let smaller_border_starts: Vec<NaiveTime> = intraday_borders_vec.iter()
            .filter(|x|{x.start <= end_time}).map(|x|{x.start}).collect();
            let near_border_start = smaller_border_starts.iter().max().unwrap();
            all_delta_time = all_delta_time + end_time.signed_duration_since(*near_border_start);
        }

        let smaller_intraday_borders: Vec<TimeBorder> = intraday_borders_vec.iter().cloned()
        .filter(|x|{x.end <= end_time}).collect();

        for smaller_intraday_border in smaller_intraday_borders.iter() {
            all_delta_time = all_delta_time + smaller_intraday_border.end.signed_duration_since(smaller_intraday_border.start);
        }
    }

    return all_delta_time;
}