use std::collections::HashSet;
use chrono::{NaiveDate, Datelike};

use super::global::{RANGE_HOLIDAYS_VEC, ONE_HOLIDAY_WEEKDAY_SET, DEFAULT_DATE_1};

pub fn get_workdays(start_date: NaiveDate, end_date: NaiveDate, closed: &str) -> Vec<NaiveDate> {

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
    if closed=="left" { // 開始日の重なりを許容
        if workdays_vec.len() > 0 {
            if workdays_vec.last().unwrap() == &end_date {
                workdays_vec.remove(workdays_vec.len()-1);  // pop
            }
        }
    } else if closed=="right" {  // 終了日の重なりを許容
        if workdays_vec.len() > 0 {
            if workdays_vec.first().unwrap() == &start_date {
                workdays_vec.remove(0);  // 最初の値を削除
            }
        }
    } else if closed=="not" {  // どちらも許容しない
        if workdays_vec.len() > 0 {
            if workdays_vec.last().unwrap() == &end_date {
                workdays_vec.remove(workdays_vec.len()-1);  // pop
            }
        }
        if workdays_vec.len() > 0 {
            if workdays_vec.first().unwrap() == &start_date {
                workdays_vec.remove(0);  // 最初の値を削除
            }
        }
    } // "both"などそれ以外はどちらも許容

    return workdays_vec;
}


pub fn check_workday(select_date: NaiveDate) -> bool {
    let holidays_vec = RANGE_HOLIDAYS_VEC.read().unwrap();
    let one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.read().unwrap();

    let holidays_set: HashSet<NaiveDate> = holidays_vec.iter().cloned().collect();
    let is_holiday: bool = holidays_set.contains(&select_date);
    let is_holiday_weekday: bool = one_holiday_weekday_set.contains(&select_date.weekday());
    (!is_holiday) & (!is_holiday_weekday)
}


pub fn get_next_workday(select_date: NaiveDate, days: i32) -> NaiveDate {
    let holidays_vec = RANGE_HOLIDAYS_VEC.read().unwrap();
    let one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.read().unwrap();
    
    let holidays_bigger_select: Vec<NaiveDate> = holidays_vec.iter().cloned().filter(|x| {x >= &select_date}).collect();

    // daysを計算するカウンター
    let mut counter:i32 = 0;

    // イテレーターの作成
    let mut holiday_iter = holidays_bigger_select.iter();
    let mut day_iter = select_date.iter_days(); 

    let mut one_holiday = holiday_iter.next().unwrap_or(&DEFAULT_DATE_1);
    let mut one_day = day_iter.next().unwrap();

    // 最初はloopの外で，さらに初日がworkdaysでもカウントしない
    if one_day==*one_holiday {
        one_holiday = holiday_iter.next().unwrap_or(&DEFAULT_DATE_1);
    }

    one_day = day_iter.next().unwrap();

    loop {
        if one_day==*one_holiday { // その日が祝日である
            one_holiday = holiday_iter.next().unwrap_or(&DEFAULT_DATE_1);
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


pub fn get_previous_workday(select_date: NaiveDate, days: i32) -> NaiveDate {
    let holidays_vec = RANGE_HOLIDAYS_VEC.read().unwrap();
    let one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.read().unwrap();

    let mut holidays_smaller_select: Vec<NaiveDate> = holidays_vec.iter().cloned().filter(|x| {x <= &select_date}).collect();
    holidays_smaller_select.reverse();

    // daysを計算するカウンター
    let mut counter:i32 = 0;

    // イテレーターの作成
    let mut holiday_iter = holidays_smaller_select.iter();

    let mut one_holiday = holiday_iter.next().unwrap_or(&DEFAULT_DATE_1);
    let mut one_day = select_date;

    // 最初はloopの外で，さらに初日がworkdaysでもカウントしない
    if one_day==*one_holiday {
        one_holiday = holiday_iter.next().unwrap_or(&DEFAULT_DATE_1);
    }

    one_day = one_day.pred_opt().unwrap();

    loop {
        if one_day==*one_holiday { // その日が祝日である
            one_holiday = holiday_iter.next().unwrap_or(&DEFAULT_DATE_1);
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


pub fn get_next_workdays_number(start_date: NaiveDate, days: i32) -> Vec<NaiveDate>{
    let holidays_vec = RANGE_HOLIDAYS_VEC.read().unwrap();
    let one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.read().unwrap();
    
    let holidays_bigger_select: Vec<NaiveDate> = holidays_vec.iter().cloned().filter(|x| {x >= &start_date}).collect();

    // daysを計算するカウンター
    let mut counter:i32 = 0;

    // イテレーターの作成
    let mut holiday_iter = holidays_bigger_select.iter();
    let mut day_iter = start_date.iter_days(); 

    let mut one_holiday = holiday_iter.next().unwrap_or(&DEFAULT_DATE_1);
    let mut one_day = day_iter.next().unwrap();

    let mut workdays_vec: Vec<NaiveDate> = Vec::new();

    // 初日もカウントする
    loop {
        if one_day==*one_holiday { // その日が祝日である
            one_holiday = holiday_iter.next().unwrap_or(&DEFAULT_DATE_1);
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


pub fn get_previous_workdays_number(start_date: NaiveDate, days: i32) -> Vec<NaiveDate>{
    let holidays_vec = RANGE_HOLIDAYS_VEC.read().unwrap();
    let one_holiday_weekday_set = ONE_HOLIDAY_WEEKDAY_SET.read().unwrap();

    let mut holidays_smaller_select: Vec<NaiveDate> = holidays_vec.iter().cloned().filter(|x| {x <= &start_date}).collect();
    holidays_smaller_select.reverse();

    // daysを計算するカウンター
    let mut counter:i32 = 0;

    // イテレーターの作成
    let mut holiday_iter = holidays_smaller_select.iter();

    let mut one_holiday = holiday_iter.next().unwrap_or(&DEFAULT_DATE_1);
    let mut one_day = start_date;

    let mut workdays_vec: Vec<NaiveDate> = Vec::new();

    // 初日もカウントする
    loop {
        if one_day==*one_holiday { // その日が祝日である
            one_holiday = holiday_iter.next().unwrap_or(&DEFAULT_DATE_1);
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


pub fn get_workdays_number(start_date: NaiveDate, days: i32) -> Vec<NaiveDate> {
    if days > 0 {
        get_next_workdays_number(start_date, days)
    } else if days < 0 {
        get_previous_workdays_number(start_date, days)
    } else { // 0 の場合
        let nan_vec: Vec<NaiveDate> = Vec::new();
        return nan_vec;
    }
}

