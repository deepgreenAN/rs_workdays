//! # 営業日・営業時間を取得・抽出

/// 祝日・休日曜日・営業時間などの設定
pub mod global;

/// 営業日の取得・チェック
pub mod workdays;

/// 営業時間内かチェック・営業時間のDuration演算
pub mod intraday;

/// 営業時間内のデータの抽出
pub mod extract;

/// エラー
pub mod error;

/// スクレイピング
pub mod scrape_holidays;
