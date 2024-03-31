/// 唯一值`ID`与多种语言在哪列的配置
#[derive(Debug)]
pub struct LocaleIndexConf {
    /// 文件中的 语言，如果为 None 则表示当前为 ID 列
    pub lang: Option<String>,
    /// excel 中的下标
    pub source_idx: usize,
}

/// 行配置
#[derive(Debug, Clone)]
pub struct LocaleRowConf {
    /// 语言
    pub lang: String,
    /// 生成 JSON 的 Key
    pub key: String,
    /// 生成 JSON 的 Value
    pub value: String,
}
