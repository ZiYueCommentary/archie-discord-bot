use std::collections::HashMap;
use std::fs;
use std::fs::{DirEntry, File};
use std::sync::OnceLock;

static LOCALES_HASH: OnceLock<HashMap<String, HashMap<String, String>>> = OnceLock::new();

pub fn init() {
    let mut locales: HashMap<String, HashMap<String, String>> = HashMap::new();

    let directory = fs::read_dir("locales").unwrap();
    for file in directory {
        let entry: DirEntry = file.unwrap();
        let path = entry.path().as_path().to_owned();
        if !path.is_file() {
            continue;
        }
        if let Some(file_name) = path.to_owned().file_stem() {
            if let Some(file_name) = file_name.to_str() {
                let i10n: HashMap<String, String> =
                    serde_json::from_reader(std::io::BufReader::new(File::open(path).unwrap()))
                        .unwrap();
                locales.insert(file_name.to_string(), i10n);
            }
        }
    }

    LOCALES_HASH.get_or_init(|| locales);
}

pub fn get_option(locale: &str, key: &'static str) -> Option<&'static str> {
    let map = LOCALES_HASH.get();
    map.and_then(|locales| locales.get(locale))
        .and_then(|i10n| i10n.get(key))
        .map(|s| s.as_str())
}

pub fn get(locale: &str, key: &'static str) -> &'static str {
    let map = LOCALES_HASH.get();
    if let Some(result) = map
        .and_then(|locales| locales.get(locale))
        .and_then(|i10n| i10n.get(key))
        .map(|s| s.as_str())
    {
        return result;
    }
    if let Some(fallback) = map
        .and_then(|locales| locales.get("en-US"))
        .and_then(|i10n| i10n.get(key))
    {
        return fallback;
    }
    key
}

pub fn get_all(key: &'static str) -> HashMap<String, String> {
    let map = LOCALES_HASH.get().unwrap();
    let mut result: HashMap<String, String> = HashMap::new();
    for (locale, translations) in map {
        let t: Option<&String> = translations.get(key);
        if let Some(value) = t {
            result.insert(locale.to_string(), value.to_string());
        }
    }
    result
}
