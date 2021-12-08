use std::path::Path;
use std::fs::{create_dir_all};
use csv::Writer;

#[cfg(not(feature="wasm"))]
use reqwest::blocking::get;

use encoding_rs::SHIFT_JIS;
use chrono::NaiveDate;
use anyhow::Context;
use crate::error::Error;

/// 指定したパスにcsvファイルを保存
/// Argments
/// - holidays: 祝日データ
/// - holiday_names: 祝日名データ
/// - source_path: 保存パス
fn write_csv_file<P:AsRef<Path>>(holidays: &Vec<NaiveDate>, holiday_names: &Vec<String>, source_path: P) -> Result<(), Error>{
    let source_path: &Path = source_path.as_ref();
    let source_path_str = source_path.to_str().context("cannot convert source path to string")?;

    // 親ディレクトリを作成
    let source_parent_path = source_path.parent()
        .context(format!("cannot get parent from source '{:}'", source_path_str))?;
    if !source_parent_path.exists() {
        create_dir_all(source_parent_path)
        .context(format!("create dir error for parent of '{:}'", source_path_str))?;
    }

    // csvファイルを書き込む
    let mut wtr = Writer::from_path(source_path)
        .map_err(|_|{Error::WriteCsvError{path_str: source_path_str.to_string()}})?;
    for i in 0..holidays.len() {
        wtr.write_record(&[
            holidays[i].format("%Y-%m-%d").to_string(),
            holiday_names[i].clone()
        ]).map_err(|_|{Error::WriteCsvError{path_str: source_path_str.to_string()}})?;
    }
    Ok(())
}

/// 内閣府のデータを指定したパスにソースとして保存
/// Argment
/// - source_path: 保存するcsvのパス
pub fn make_source_naikaku<P:AsRef<Path>>(source_path: P) -> Result<(), Error>{
    let url = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";
    let res = get(url)?;
    let res_bytes = res.bytes()?;

    let (decoded_content, _, _) = SHIFT_JIS.decode(&res_bytes);
    let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(decoded_content.as_bytes());

    let mut holidays: Vec<NaiveDate> = Vec::new();
    let mut holiday_names: Vec<String> = Vec::new();

    for result in rdr.records() {
        let record = result.map_err(|_|{Error::ReadCsvError{path_str: url.to_string()}})?;
        holidays.push(
            NaiveDate::parse_from_str(record[0].into(), "%Y/%m/%d")
            .map_err(|_|{Error::ParseDateError{date_str:record[0].into()}})?
        );
        holiday_names.push(record[1].into());
    }

    write_csv_file(&holidays, &holiday_names, source_path)?;
    Ok(())
}   