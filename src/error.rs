#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("reqest error in scraping")]
    RequestError(#[from] reqwest::Error),

    #[error("error in read csv path:{path_str:?}")]
    ReadCsvError{path_str: String},

    #[error("error in write csv path:{path_str:?}")]
    WriteCsvError{path_str: String},

    #[error("date parse error for {date_str:?}")]
    ParseDateError{date_str: String},

    #[error(transparent)]
    Other(#[from] anyhow::Error)
}