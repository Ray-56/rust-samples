/// Column configuration for the unique `ID` field and language columns
#[derive(Debug)]
pub struct LocaleIndexConf {
    /// The language in the file; `None` means the current column is the ID column
    pub lang: Option<String>,
    /// Column index in Excel
    pub source_idx: usize,
}

/// Row configuration
#[derive(Debug, Clone)]
pub struct LocaleRowConf {
    /// Language
    pub lang: String,
    /// Generated JSON key
    pub key: String,
    /// Generated JSON value
    pub value: String,
}
