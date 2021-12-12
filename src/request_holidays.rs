use chrono::NaiveDate;

use crate::global::set_range_holidays;
use crate::error::Error;

/// 内閣府による祝日データを取得して祝日に設定する(同期)(feature!="wasm")
/// Argments  
/// - start_year: 利用範囲の開始年
/// - end_year: 利用範囲の終了年
#[cfg(feature = "source")]
pub fn request_holidays_naikaku(start_year: i32, end_year: i32) -> Result<(), Error>{
    let url = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";
    let res = reqwest::blocking::get(url)?;
    let res_bytes = res.bytes()?;

    let (decoded_content, _, _) = encoding_rs::SHIFT_JIS.decode(&res_bytes);
    let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(decoded_content.as_bytes());

    let mut holidays: Vec<NaiveDate> = Vec::new();

    for result in rdr.records() {
        let record = result.map_err(|_|{Error::ReadCsvError{path_str: url.to_string()}})?;
        holidays.push(
            NaiveDate::parse_from_str(record[0].into(), "%Y/%m/%d")
            .map_err(|_|{Error::ParseDateError{date_str:record[0].into()}})?
        );
    }

    set_range_holidays(&holidays, start_year, end_year);
    Ok(())
}

/// 内閣府による祝日データを取得して祝日に設定する(非同期)(feature="wasm")
/// Argments  
/// - start_year: 利用範囲の開始年
/// - end_year: 利用範囲の終了年
#[cfg(feature = "wasm_source")]
pub async fn request_holidays_naikaku(start_year: i32, end_year: i32) -> Result<(), Error>{
    let url = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";
    let res = reqwest_wasm::get(url).await?;
    let res_bytes = res.bytes().await?;

    let (decoded_content, _, _) = encoding_rs::SHIFT_JIS.decode(&res_bytes);
    let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(decoded_content.as_bytes());

    let mut holidays: Vec<NaiveDate> = Vec::new();

    for result in rdr.records() {
        let record = result.map_err(|_|{Error::ReadCsvError{path_str: url.to_string()}})?;
        holidays.push(
            NaiveDate::parse_from_str(record[0].into(), "%Y/%m/%d")
            .map_err(|_|{Error::ParseDateError{date_str:record[0].into()}})?
        );
    }

    set_range_holidays(&holidays, start_year, end_year);
    Ok(())
}