//! Общие утилиты для примеров, в частности пути к тестовым файлам.

use std::path::{Path, PathBuf};

/// test_data_dir возвращает абсолютный путь к директории с тестовыми данными.
pub fn test_data_dir() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir)
        .parent()
        .expect("should have parent (workspace root?)")
        .join("examples/data")
}

/// Возвращает путь к файлу `records_example.txt` в тестовой директории.
#[allow(dead_code)]
pub fn records_txt_path() -> PathBuf {
    test_data_dir().join("records_example.txt")
}

/// Возвращает путь к файлу `records_example.csv` в тестовой директории.
#[allow(dead_code)]
pub fn records_csv_path() -> PathBuf {
    test_data_dir().join("records_example.csv")
}

/// Возвращает путь к файлу `records_example.bin` в тестовой директории.
#[allow(dead_code)]
pub fn records_bin_path() -> PathBuf {
    test_data_dir().join("records_example.bin")
}