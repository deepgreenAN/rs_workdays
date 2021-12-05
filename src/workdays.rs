use std::collections::HashSet;
use chrono::{NaiveDate, Datelike};

use crate::global::{RANGE_HOLIDAYS_VEC, ONE_HOLIDAY_WEEKDAY_SET, IMPOSSIBLE_DATE_1};

/// 期間の端を含む(閉じる)かどうかを指定する
#[derive(Debug, Clone, Copy)]
pub enum Closed {
    /// 左側のみを閉じる
    Left,
    /// 右側のみを閉じる
    Right,
    /// どちらも閉じない
    Not,
    /// どちらも閉じる
    Both
}


/// start_dateからend_dateまでの営業日を取得  
/// Argments
/// - start_date: 開始日
/// - end_date: 終了日
/// - closed: 境界を含めるかどうか
///     - left: 終了境界を含めない
///     - right: 開始境界を含めない
///     - not: どちらの境界も含める
///     - both: どちらの境界も含めない
/// 
/// Return  
/// workdays_vec: 営業日のべクター
/// 
/// # Example
/// ~~~~
/// use chrono::{NaiveDate};
/// use rs_workdays::workdays::*;
/// let workday_start_date = NaiveDate::from_ymd(2021,1,1);
/// let workday_end_date = NaiveDate::from_ymd(2021,2,1);
/// let workdays_vec = get_workdays(workday_start_date, workday_end_date, Closed::Left);
/// println!("workdays_vec: {:?}", workdays_vec);
/// ~~~~
/// 
/// workdays_vec: [2021-01-04, 2021-01-05, 2021-01-06, 2021-01-07, 2021-01-08, 2021-01-12, 2021-01-13,
///  2021-01-14, 2021-01-15, 2021-01-18, 2021-01-19, 2021-01-20, 2021-01-21, 2021-01-22, 2021-01-25,
///  2021-01-26, 2021-01-27, 2021-01-28, 2021-01-29]
///
pub fn get_workdays(start_date: NaiveDate, end_date: NaiveDate, closed: Closed) -> Vec<NaiveDate> {

    let holidays_vec = RANGE_HOLIDAYS_VEC.read().unwrap();
    let one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.read().unwrap();

    let all_day_set: HashSet<NaiveDate> = start_date.iter_days()
    .take_while(|x| {x<=&end_date}).collect();  // 全ての日

    let holiday_weekdays_set: HashSet<NaiveDate> = start_date.iter_days()
    .take_while(|x| {x<=&end_date}).filter(|x|{one_holiday_weekday_set.contains(&x.weekday())}).collect();

    // setによる演算
    let holidays_set: HashSet<NaiveDate> = holidays_vec.iter().cloned().collect();
    let all_holidays_set: HashSet<NaiveDate> = holidays_set.union(&holiday_weekdays_set).cloned().collect();

    let mut workdays_vec: Vec<NaiveDate> = all_day_set.difference(&all_holidays_set).cloned().collect();
    workdays_vec.sort();

    // 開始日と終了日の処理
    match closed {
        Closed::Left => {  // 開始日を許容
            if workdays_vec.last().unwrap_or(&IMPOSSIBLE_DATE_1)==&end_date {workdays_vec.remove(workdays_vec.len()-1);}
        },
        Closed::Right => {  // 終了日を許容
            if workdays_vec.first().unwrap_or(&IMPOSSIBLE_DATE_1)==&start_date {workdays_vec.remove(0);}            
        },
        Closed::Both => {  // どちらも許容
        },
        Closed::Not => {  // どちらも削除
            if workdays_vec.last().unwrap_or(&IMPOSSIBLE_DATE_1)==&end_date {workdays_vec.remove(workdays_vec.len()-1);}
            if workdays_vec.first().unwrap_or(&IMPOSSIBLE_DATE_1)==&start_date {workdays_vec.remove(0);}
        }
    }

    return workdays_vec;
}

/// select_dateが営業日であるか判定
/// Argment
/// - select_date: 指定する日
/// 
/// Return
/// 営業日であるかどうか
/// 
/// # Example
/// ~~~~
/// use chrono::{NaiveDate};
/// use rs_workdays::workdays::*;
/// let select_date = NaiveDate::from_ymd(2021,1,1);
/// let is_workday = check_workday(select_date);
/// assert!(!is_workday);
/// ~~~~
pub fn check_workday(select_date: NaiveDate) -> bool {
    let holidays_vec = RANGE_HOLIDAYS_VEC.read().unwrap();
    let one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.read().unwrap();

    let holidays_set: HashSet<NaiveDate> = holidays_vec.iter().cloned().collect();
    let is_holiday: bool = holidays_set.contains(&select_date);
    let is_holiday_weekday: bool = one_holiday_weekday_set.contains(&select_date.weekday());
    (!is_holiday) & (!is_holiday_weekday)
}

/// select_dateからdays分の次の営業日を取得
/// Argments
/// - select_date: 指定する日
/// - days: 進める日数
/// 
/// Return
/// one_day: 次の営業日
/// 
/// # Example
/// ~~~~
/// use chrono::{NaiveDate};
/// use rs_workdays::workdays::*;
/// let select_date = NaiveDate::from_ymd(2021,1,1);
/// let next_workday = get_next_workday(select_date, 6);
/// assert_eq!(next_workday, NaiveDate::from_ymd(2021,01,12));
/// ~~~~
pub fn get_next_workday(select_date: NaiveDate, days: i32) -> NaiveDate {
    let holidays_vec = RANGE_HOLIDAYS_VEC.read().unwrap();
    let one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.read().unwrap();
    
    let holidays_bigger_select: Vec<NaiveDate> = holidays_vec.iter().cloned().filter(|x| {x >= &select_date}).collect();

    // daysを計算するカウンター
    let mut counter:i32 = 0;

    // イテレーターの作成
    let mut holiday_iter = holidays_bigger_select.iter();
    let mut day_iter = select_date.iter_days(); 

    let mut one_holiday = holiday_iter.next().unwrap_or(&IMPOSSIBLE_DATE_1);
    let mut one_day = day_iter.next().unwrap();

    // 最初はloopの外で，さらに初日がworkdaysでもカウントしない
    if one_day==*one_holiday {
        one_holiday = holiday_iter.next().unwrap_or(&IMPOSSIBLE_DATE_1);
    }

    one_day = day_iter.next().unwrap();

    loop {
        if one_day==*one_holiday { // その日が祝日である
            one_holiday = holiday_iter.next().unwrap_or(&IMPOSSIBLE_DATE_1);
        } else { // その日が祝日でない
            if !one_holiday_weekday_set.contains(&one_day.weekday()) { // その日が休日曜日でない
                counter += 1; // カウンターをインクリメント
            }
        }

        if counter >= days {
            break;
        }

        one_day = day_iter.next().unwrap();
    }

    return one_day;
}

/// select_dateからdays分の前の営業日を取得
/// Argment
/// - select_date: 指定する日
/// - days: 減らす日数
/// 
/// Return
/// one_day: 前の営業日
/// 
/// # Example
/// ~~~~
/// use chrono::{NaiveDate};
/// use rs_workdays::workdays::*;
/// let select_date = NaiveDate::from_ymd(2021,1,12);
/// let previous_workday = get_previous_workday(select_date, 6);
/// assert_eq!(previous_workday, NaiveDate::from_ymd(2020,12,31));
/// ~~~~
pub fn get_previous_workday(select_date: NaiveDate, days: i32) -> NaiveDate {
    let holidays_vec = RANGE_HOLIDAYS_VEC.read().unwrap();
    let one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.read().unwrap();

    let mut holidays_smaller_select: Vec<NaiveDate> = holidays_vec.iter().cloned().filter(|x| {x <= &select_date}).collect();
    holidays_smaller_select.reverse();

    // daysを計算するカウンター
    let mut counter:i32 = 0;

    // イテレーターの作成
    let mut holiday_iter = holidays_smaller_select.iter();

    let mut one_holiday = holiday_iter.next().unwrap_or(&IMPOSSIBLE_DATE_1);
    let mut one_day = select_date;

    // 最初はloopの外で，さらに初日がworkdaysでもカウントしない
    if one_day==*one_holiday {
        one_holiday = holiday_iter.next().unwrap_or(&IMPOSSIBLE_DATE_1);
    }

    one_day = one_day.pred_opt().unwrap();

    loop {
        if one_day==*one_holiday { // その日が祝日である
            one_holiday = holiday_iter.next().unwrap_or(&IMPOSSIBLE_DATE_1);
        } else { // その日が祝日でない
            if !one_holiday_weekday_set.contains(&one_day.weekday()) { // その日が休日曜日でない
                counter += 1; // カウンターをインクリメント
            }
        }

        if counter >= days {
            break;
        }

        one_day = one_day.pred_opt().unwrap();
    }

    return one_day;
}

/// 最近の営業日を取得
/// Argments
/// - select_date: 指定する日
/// - is_after: 後の営業日を所得するかどうか
/// 
/// Return
/// 最近の営業日
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDate};
/// use rs_workdays::workdays::*;
/// let select_date = NaiveDate::from_ymd(2021, 1, 1);
/// let near_workday = get_near_workday(select_date, true);
/// assert_eq!(near_workday, NaiveDate::from_ymd(2021,01,04));
/// 
/// let select_date = NaiveDate::from_ymd(2021, 1, 1);
/// let near_workday = get_near_workday(select_date, false);
/// assert_eq!(near_workday, NaiveDate::from_ymd(2020,12,31))
/// ~~~~
pub fn get_near_workday(select_date: NaiveDate, is_after: bool) -> NaiveDate{
    if check_workday(select_date) { // 指定日が営業日である場合
        select_date
    } else {
        if is_after {
            get_next_workday(select_date, 1)
        } else {
            get_previous_workday(select_date, 1)
        }
    }
}

/// start_dateからdays分だけ後ろの営業日のベクターを取得
/// Argments
/// - start_date: 開始日
/// - days: 日数
/// 
/// Return
/// workdays_vec: 営業日のベクター
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDate};
/// use rs_workdays::workdays::*;
/// let start_date = NaiveDate::from_ymd(2021, 1, 1);
/// let workdays_vec = get_next_workdays_number(start_date, 19);
/// println!("workdays_vec: {:?}", workdays_vec);
/// ~~~~
/// 
/// workdays_vec: [2021-01-04, 2021-01-05, 2021-01-06, 2021-01-07, 2021-01-08, 2021-01-12, 2021-01-13,
///  2021-01-14, 2021-01-15, 2021-01-18, 2021-01-19, 2021-01-20, 2021-01-21, 2021-01-22, 2021-01-25,
///  2021-01-26, 2021-01-27, 2021-01-28, 2021-01-29]
pub fn get_next_workdays_number(start_date: NaiveDate, days: i32) -> Vec<NaiveDate>{
    let holidays_vec = RANGE_HOLIDAYS_VEC.read().unwrap();
    let one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.read().unwrap();
    
    let holidays_bigger_select: Vec<NaiveDate> = holidays_vec.iter().cloned().filter(|x| {x >= &start_date}).collect();

    // daysを計算するカウンター
    let mut counter:i32 = 0;

    // イテレーターの作成
    let mut holiday_iter = holidays_bigger_select.iter();
    let mut day_iter = start_date.iter_days(); 

    let mut one_holiday = holiday_iter.next().unwrap_or(&IMPOSSIBLE_DATE_1);
    let mut one_day = day_iter.next().unwrap();

    let mut workdays_vec: Vec<NaiveDate> = Vec::new();

    // 初日もカウントする
    loop {
        if one_day==*one_holiday { // その日が祝日である
            one_holiday = holiday_iter.next().unwrap_or(&IMPOSSIBLE_DATE_1);
        } else { // その日が祝日でない
            if !one_holiday_weekday_set.contains(&one_day.weekday()) { // その日が休日曜日でない
                counter += 1; // カウンターをインクリメント
                workdays_vec.push(one_day)  // workdays_vecに追加
            }
        }

        if counter >= days {
            break;
        }

        one_day = day_iter.next().unwrap();
    }

    return workdays_vec;
}

/// start_dateからdays分だけ前の営業日のベクターを取得
/// Argments
/// - start_date: 開始日
/// - days: 日数
/// 
/// Return
/// workdays_vec: 営業日のベクター
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDate};
/// use rs_workdays::workdays::*;
/// let start_date = NaiveDate::from_ymd(2021,1,29);
/// let mut workdays_vec = get_previous_workdays_number(start_date, 19);
/// workdays_vec.sort();
/// println!("workdays_vec: {:?}", workdays_vec);
/// ~~~~
/// 
/// workdays_vec: [2021-01-04, 2021-01-05, 2021-01-06, 2021-01-07, 2021-01-08, 2021-01-12, 2021-01-13,
///  2021-01-14, 2021-01-15, 2021-01-18, 2021-01-19, 2021-01-20, 2021-01-21, 2021-01-22, 2021-01-25,
///  2021-01-26, 2021-01-27, 2021-01-28, 2021-01-29]
pub fn get_previous_workdays_number(start_date: NaiveDate, days: i32) -> Vec<NaiveDate>{
    let holidays_vec = RANGE_HOLIDAYS_VEC.read().unwrap();
    let one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.read().unwrap();

    let mut holidays_smaller_select: Vec<NaiveDate> = holidays_vec.iter().cloned().filter(|x| {x <= &start_date}).collect();
    holidays_smaller_select.reverse();

    // daysを計算するカウンター
    let mut counter:i32 = 0;

    // イテレーターの作成
    let mut holiday_iter = holidays_smaller_select.iter();

    let mut one_holiday = holiday_iter.next().unwrap_or(&IMPOSSIBLE_DATE_1);
    let mut one_day = start_date;

    let mut workdays_vec: Vec<NaiveDate> = Vec::new();

    // 初日もカウントする
    loop {
        if one_day==*one_holiday { // その日が祝日である
            one_holiday = holiday_iter.next().unwrap_or(&IMPOSSIBLE_DATE_1);
        } else { // その日が祝日でない
            if !one_holiday_weekday_set.contains(&one_day.weekday()) { // その日が休日曜日でない
                counter += 1; // カウンターをインクリメント
                workdays_vec.push(one_day)  // workdays_vecに追加
            }
        }

        if counter >= days {
            break;
        }

        one_day = one_day.pred_opt().unwrap();
    }

    return workdays_vec;
}

/// start_dateからdays分だけの営業日のベクターを取得
/// Argments
/// - start_date: 開始日
/// - days: 日数
/// 
/// Return
/// workdays_vec: 営業日のベクター
/// 
/// # Examples
/// ~~~~
/// use chrono::{NaiveDate};
/// use rs_workdays::workdays::*;
/// let start_date = NaiveDate::from_ymd(2021, 1, 1);
/// let workdays_vec = get_workdays_number(start_date, 19);
/// println!("workdays_vec: {:?}", workdays_vec);
/// ~~~~
/// 
/// workdays_vec: [2021-01-04, 2021-01-05, 2021-01-06, 2021-01-07, 2021-01-08, 2021-01-12, 2021-01-13,
///  2021-01-14, 2021-01-15, 2021-01-18, 2021-01-19, 2021-01-20, 2021-01-21, 2021-01-22, 2021-01-25,
///  2021-01-26, 2021-01-27, 2021-01-28, 2021-01-29]
pub fn get_workdays_number(start_date: NaiveDate, days: i32) -> Vec<NaiveDate> {
    if days > 0 {
        get_next_workdays_number(start_date, days)
    } else if days < 0 {
        get_previous_workdays_number(start_date, days.abs())
    } else { // 0 の場合
        let nan_vec: Vec<NaiveDate> = Vec::new();
        return nan_vec;
    }
}

