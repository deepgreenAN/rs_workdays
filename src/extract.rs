use chrono::{NaiveDateTime};

use super::global::{INTRADAY_BORDERS, DEFAULT_DATE_1};
use super::workdays::{get_workdays};

pub fn extract_workdays_bool_vec(datetime_vec:&Vec<NaiveDateTime>) -> Vec<bool> {
    let mut bool_vec = vec![false;datetime_vec.len()];
    let first_date = datetime_vec.first().unwrap().date();
    let last_date = datetime_vec.last().unwrap().date();

    let workdays_vec = get_workdays(first_date, last_date, "both");
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

pub fn extract_workdays_intraday_bool_vec(datetime_vec:&Vec<NaiveDateTime>) -> Vec<bool> {
    let mut bool_vec = vec![false;datetime_vec.len()];
    let first_date = datetime_vec.first().unwrap().date();
    let last_date = datetime_vec.last().unwrap().date();

    let workdays_vec = get_workdays(first_date, last_date, "both");
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
