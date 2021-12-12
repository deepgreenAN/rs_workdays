//! # 営業日・営業時間を取得・抽出

/// 祝日・休日曜日・営業時間などの設定
pub mod global;

pub use global::{
    set_holidays_csvs, 
    set_intraday_borders, 
    set_holiday_weekdays, 
    set_range_holidays,
    add_range_holidays,
    get_range_holidays,
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
#[cfg(feature="source")]
pub mod scrape_holidays;

#[cfg(feature="source")]
pub use scrape_holidays::*;

/// リクエスト
#[cfg(any(feature="source", feature="wasm_source"))]
pub mod request_holidays;

#[cfg(any(feature="source", feature="wasm_source"))]
pub use request_holidays::*;
