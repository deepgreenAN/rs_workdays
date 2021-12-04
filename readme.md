# 営業日・営業時間のデータを取得・抽出
営業日のデータを取得，`Vec<NaiveDateTime>`から営業日・営業時間のデータを抽出できる．[こちら](https://github.com/deepgreenAN/py_workdays)のrust実装バージョン．

## installation
Cargo.tomlに以下を追加
```
[dependencies]
rs_workdays = {git = "https://github.com/deepgreenAN/rs_workdays.git"}
```
csvファイルを自分のプロジェクトのデフォルトの位置`source/holiday_naikaku.csv`に置くか[こちら](https://drive.google.com/file/d/15V46C74rT1kdYvZGnGnps5hFF8j1UBJB/view?usp=sharing)
(2022年までの日本の祝日)からダウンロードして配置．csvファイルは以下のような形になっていればよい．
```
1955-01-01,元日
1955-01-15,成人の日
1955-03-21,春分の日
```

## 使い方
基本的に`chrono::NaiveDateTime`・`chrono::NaiveDate`を引数として`chrono::NaiveDateTime`・`chrono::NaiveDate`やそのベクター，文字列とのタプルを返す関数である．
```rust
use chrono::{NaiveDate, Duration, NaiveDateTime};

use rs_workdays::workdays::{get_workdays, check_workday, get_next_workday, get_previous_workday};
use rs_workdays::workdays::{get_near_workday, get_next_workdays_number, get_previous_workdays_number, get_workdays_number};

use rs_workdays::intraday::{check_workday_intraday, get_next_border_workday_intraday, get_previous_border_workday_intraday};
use rs_workdays::intraday::{add_workday_intraday_datetime, sub_workday_intraday_datetime, get_timedelta_workdays_intraday};
use rs_workdays::extract::{extract_workdays_bool, extract_intraday_bool, extract_workdays_intraday_bool};
```
### 指定期間の営業日を取得
```rust
let workday_start_date = NaiveDate::from_ymd(2021,1,1);
let workday_end_date = NaiveDate::from_ymd(2021,2,1);
let workdays_vec = get_workdays(workday_start_date, workday_end_date, &"left");
println!("workdays_vec: {:?}", workdays_vec);
```
```
workdays_vec: [2021-01-04, 2021-01-05, 2021-01-06, 2021-01-07, 2021-01-08, 2021-01-12, 2021-01-13, 2021-01-14, 2021-01-15, 2021-01-18, 2021-01-19, 2021-01-20, 2021-01-21, 2021-01-22, 2021-01-25, 2021-01-26, 2021-01-27, 2021-01-28, 2021-01-29]
```

### 営業日かどうか判定
```rust
let select_date = NaiveDate::from_ymd(2021,1,1);
let is_workday = check_workday(select_date);
println!("{:?} is_workday: {:?}",select_date, is_workday);
```
```
2021-01-01 is_workday: false
```

### 次の営業日を取得
```rust
// get_next_workday
let select_date = NaiveDate::from_ymd(2021,1,1);
let next_workday = get_next_workday(select_date, 6);
println!("next workday of {:?} is {:?}", select_date, next_workday);
```
```
next workday of 2021-01-01 is 2021-01-12
```

### 指定する日数分の営業日を取得
```rust
let start_date = NaiveDate::from_ymd(2021, 1, 1);
let workdays_vec = get_workdays_number(start_date, 19);
println!("workdays_vec: {:?}", workdays_vec);
```
```
workdays_vec: [2021-01-04, 2021-01-05, 2021-01-06, 2021-01-07, 2021-01-08, 2021-01-12, 2021-01-13, 2021-01-14, 2021-01-15, 2021-01-18, 2021-01-19, 2021-01-20, 2021-01-21, 2021-01-22, 2021-01-25, 2021-01-26, 2021-01-27, 2021-01-28, 2021-01-29]
```

### 営業日・営業時間内か判定
デフォルトでは，東京証券取引所の営業日(土日・祝日，振替休日を除く)・営業時間(9時～11時30分，12時30分～15時)として利用できる．
```rust
let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(10,0,0);
let is_workday_intraday = check_workday_intraday(select_datetime);
println!("{:?} is workday and intraday: {:?}", select_datetime, is_workday_intraday);
```
```
2021-01-01T10:00:00 is workday and intraday: false
```

### 指定日時から最も近い次の営業日・営業時間の日時を取得
```rust
let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
let (next_border_datetime, border_symbol) = get_next_border_workday_intraday(select_datetime);
println!("next_border_datetime: {:?}, border_symbol: {:?}", next_border_datetime, border_symbol);
```
```
next_border_datetime: 2021-01-04T09:00:00, border_symbol: "border_start"
```

### 指定日時とtimedeltaから営業時間分加算する
```rust
let select_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
let add_duration = Duration::hours(2);
let added_workday_intraday_datetime = add_workday_intraday_datetime(select_datetime, add_duration);
println!("add_workday_intraday_datetime: {:?} + {:?} = {:?}", select_datetime, add_duration, added_workday_intraday_datetime);
```
```
add_workday_intraday_datetime: 2021-01-01T00:00:00 + Duration { secs: 7200, nanos: 0 } = 2021-01-04T11:00:00
```

### 指定期間の営業時間分のchrono::Durationを取得する
```rust
let start_datetime = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0);
let end_datetime = NaiveDate::from_ymd(2021,1,4).and_hms(15,0,0);
let span_duration = get_timedelta_workdays_intraday(start_datetime, end_datetime);
println!("{:?} and {:?} timedelta: {:?}", start_datetime, end_datetime, span_duration);
```
```
2021-01-01T00:00:00 and 2021-01-04T15:00:00 timedelta: Duration { secs: 18000, nanos: 0 }
```

### `Vec<NaiveDateTime>`から営業日・営業時間のものを示す`Vec<bool>`を取得
```rust
let start_datetime_timestamp: i64 = NaiveDate::from_ymd(2021,1,1).and_hms(0,0,0).timestamp();
let add_sec: i64 = 3600; // 1時間
let datetime_vec: Vec<NaiveDateTime> = vec![0;100].iter().cloned().enumerate()
.map(|(i,_x)| {NaiveDateTime::from_timestamp(start_datetime_timestamp+ (i as i64) *add_sec, 0)}).collect();
let bool_vec: Vec<bool> = extract_workdays_intraday_bool(&datetime_vec);
let extracted_workdays_intraday_datetime: Vec<NaiveDateTime> = datetime_vec.iter().cloned().enumerate()
.filter(|(i,_x)|{bool_vec[*i]}).map(|(_i,x)|{x}).collect();
println!("extracted workday intraday datetime: {:?}", extracted_workdays_intraday_datetime);
```
```
extracted workday intraday datetime: [2021-01-04T09:00:00, 2021-01-04T10:00:00, 2021-01-04T11:00:00, 2021-01-04T13:00:00, 2021-01-04T14:00:00]
```


### 休日曜日・営業時間の変更
```rust
use std::collections::HashSet;
use chrono::{Weekday, NaiveTime};
use rs_workdays::global::{set_one_holiday_weekday_set, set_intraday_borders, TimeBorder};
```
```rust
let weekday_set: HashSet<Weekday> = [Weekday::Mon, Weekday::Tue].iter().cloned().collect();
set_one_holiday_weekday_set(weekday_set);

let intraday_borders: Vec<TimeBorder> =[
    TimeBorder {start: NaiveTime::from_hms(8,0,0), end:NaiveTime::from_hms(10,0,0)}
].to_vec();
set_intraday_borders(intraday_borders);
```

### 祝日データの読み込み
デフォルトに設定しなくても後からcsvファイルを読み込める．範囲年を明示する．
```rust
use rs_workdays::global::{set_holidays_csv};
```
```rust
set_holidays_csv("source/holiday_naikaku.csv".to_string(), 2016, 2021);
```