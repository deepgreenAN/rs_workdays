//! # 営業日・営業時間を取得・抽出

/// 祝日・休日曜日・営業時間などの設定
pub mod global;

pub use global::{
    set_holidays_csvs, 
    set_intraday_borders, 
    set_one_holiday_weekday_set, 
    set_range_holidays,
    get_range_holidays_vec,
    get_holiday_weekdays,
    get_intraday_borders
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
#[cfg(not(feature="wasm"))]
pub mod scrape_holidays;

#[cfg(not(feature="wasm"))]
pub use scrape_holidays::*;

/// リクエスト
pub mod request_holidays;
pub use request_holidays::*;
