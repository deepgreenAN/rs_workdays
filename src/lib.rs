//! # 営業日・営業時間を取得・抽出

/// 祝日・休日曜日・営業時間などの設定
pub mod global;
pub use global::{
    set_holidays_csvs, 
    set_intraday_borders, 
    set_one_holiday_weekday_set, 
    set_range_holidays
};

/// 営業日の取得・チェック
pub mod workdays;
pub use workdays::*;

/// 営業時間内かチェック・営業時間のDuration演算
pub mod intraday;
pub use intraday::*;

/// 営業時間内のデータの抽出
pub mod extract;
pub use extract::*;

/// エラー
pub mod error;
pub use error::Error;

/// スクレイピング
#[cfg(not(features="wasm"))]
pub mod scrape_holidays;

#[cfg(not(features="wasm"))]
pub use scrape_holidays::*;
