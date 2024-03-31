use std::collections::HashMap;
use std::io::Result;

use calamine::DataType;

use crate::{LocaleIndexConf, LocaleRowConf};

/// 解析首行，由`ID`与`zh-CN`关键信息生成语言座标配置(`LocaleIndexConf`)
pub fn parse_first_row(row: &[DataType]) -> Vec<LocaleIndexConf> {
    let mut rst: Vec<LocaleIndexConf> = Vec::new();
    let mut should_locale = false;

    for (idx, cell) in row.iter().enumerate() {
        let cell_string = cell.to_string();
        if cell_string == String::from("ID") {
            rst.push(LocaleIndexConf {
                lang: None,
                source_idx: idx,
            });
        } else if cell_string == String::from("zh-CN") || should_locale {
            if !should_locale {
                should_locale = true;
            }
            rst.push(LocaleIndexConf {
                lang: Some(cell_string),
                source_idx: idx,
            });
        }
    }

    return rst;
}

/// 解析行，返回不同语言的配置(`LocaleRowConf`)
/// 
/// example:
/// ```bash
/// row_parsed=[LocaleRowConf { lang: "zh-CN", key: "contactManagement.newContact.maxNumber.hint", value: "最多可上传10000名联系人。" }, LocaleRowConf { lang: "en-US", key: "contactManagement.newContact.maxNumber.hint", value: "The maximum number of contacts that can be uploaded is 10,000." }, LocaleRowConf { lang: "es-ES", key: "contactManagement.newContact.maxNumber.hint", value: "El número máximo de contactos que se pueden cargar es de 10.000." }, LocaleRowConf { lang: "id-ID", key: "contactManagement.newContact.maxNumber.hint", value: "Jumlah maksimum kontak yang dapat diunggah adalah 10.000." }, LocaleRowConf { lang: "pt-BR", key: "contactManagement.newContact.maxNumber.hint", value: "O número máximo de contactos que podem ser carregados é de 10.000." }]
/// ```
pub fn parse_row(
    row: &[DataType],
    index_conf: &Vec<LocaleIndexConf>,
) -> Result<Vec<LocaleRowConf>> {
    let mut locale_index_dict: HashMap<usize, String> = HashMap::new();
    let mut key_index: usize = 0;
    for locale in index_conf {
        match &locale.lang {
            Some(lang) => {
                locale_index_dict.insert(locale.source_idx, lang.clone());
            }
            None => {
                key_index = locale.source_idx;
            }
        }
    }

    let mut locales: Vec<LocaleRowConf> = Vec::new();
    let mut key_string: String = String::new();
    for (idx, cell) in row.iter().enumerate() {
        if idx == key_index {
            key_string = cell.to_string();
        } else {
            let lang = locale_index_dict.get(&idx);
            match lang {
                Some(lang) => {
                    if key_string.is_empty() {
                        continue;
                    }
                    let mut value = cell.to_string();
                    
                    if value.contains("\n") {
                        value = value.replace("\n", "\\n");
                    }
                    if value.contains("\"") {
                        value = value.replace("\"", "\\\"");
                    }

                    let item = LocaleRowConf {
                        lang: lang.clone(),
                        key: key_string.to_string(),
                        value,
                    };
                    locales.push(item);
                }
                None => {}
            }
        }
    }
    Ok(locales)
}
