# YPBank Parser

Библиотека и cli-утилиты для работы с финансовыми данными в заданных форматах YPBank.

## Структура

- `ypbank_parser` — библиотека для парсинга/сериализации/десериализации трёх форматов (CSV, TXT, BIN).
- `ypbank_converter` — консольная утилита для конвертации файлов между форматами.
- `ypbank_comparer` — консольная утилита для сравнения двух файлов транзакций.

Все операции чтения/записи используют трейты Read и Write из стандартной библиотеки, что позволяет работать с файлами, стандартным вводом/выводом, буферами и другими источниками/приёмниками данных.

## Поддерживаемые форматы

* **CSV**  
  Первая строка — заголовок: TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
Поля разделены запятыми, последнее поле (DESCRIPTION) заключается в двойные кавычки.
Кодировка UTF-8.

* **TXT**.  
  Записи разделяются пустыми строками.
Каждая запись содержит поля вида *КЛЮЧ: ЗНАЧЕНИЕ*.
Комментарии начинаются с *#* и игнорируются.
Поле DESCRIPTION заключается в двойные кавычки.
Пример записи:
  ~~~
  TX_ID: 1234567890123456
  TX_TYPE: DEPOSIT FROM_USER_ID: 0
  TO_USER_ID: 9876543210987654 
  AMOUNT: 10000
  TIMESTAMP: 1633036800000 STATUS: SUCCESS 
  DESCRIPTION: "Terminal deposit"
  ~~~

* **BIN**
  -  Каждая запись начинается с заголовка: 4 байта магического числа YPBN (0x59 0x50 0x42 0x4E) и 4 байта длины тела записи (big-endian).
  - Тело записи содержит поля в фиксированном порядке (все многобайтовые числа — big-endian):

    ~~~
    TX_ID (8 байт, u64)
    TX_TYPE (1 байт: 0 = DEPOSIT, 1 = TRANSFER, 2 = WITHDRAWAL)
    FROM_USER_ID (8 байт, u64)
    TO_USER_ID (8 байт, u64)
    AMOUNT (8 байт, i64)
    TIMESTAMP (8 байт, u64)
    STATUS (1 байт: 0 = SUCCESS, 1 = FAILURE, 2 = PENDING)
    DESC_LEN (4 байта, u32)
    DESCRIPTION (DESC_LEN байт, UTF-8)
    ~~~


## Структура репозитория

~~~
├── Cargo.toml # workspace
├── ypbank_parser/ # библиотека
│ ├── Cargo.toml
│ ├── src/
│ └── examples/ # примеры использования библиотеки
│ ├── data/ # тестовые файлы (example.txt, example.csv, records_example.*)
│ ├── common.rs # общие функции для примеров (пути к файлам)
│ ├── read_txt.rs # чтение текстового файла
│ ├── read_csv.rs # чтение CSV
│ ├── read_bin.rs # чтение бинарного файла
│ ├── convert.rs # конвертация между форматами
│ └── compare.rs # сравнение двух файлов
├── ypbank_converter/ # утилита конвертации
│ ├── Cargo.toml
│ └── src/
│ └── main.rs
└── ypbank_comparer/ # утилита сравнения
├── Cargo.toml
└── src/
└── main.rs
~~~

## Использование

### Сборка

```bash
cargo build
```

### Тесты

```bash
cargo test --workspace
```

### Примеры запуска

Примеры находятся в директории ypbank_parser/examples. Перед запуском примеров убедитесь, что в examples/data есть тестовые файлы (они уже включены в репозиторий).

#### Чтение текстового файла
```bash
cargo run --example read_txt
```

#### Чтение CSV
```bash
cargo run --example read_csv
```

#### Чтение бинарного файла
```bash
cargo run --example read_bin
```

#### Конвертация файлов
```bash
cargo run --example convert ./examples/data/records_example.txt txt csv > converted.csv
```

#### Сравнение двух файлов
```bash
cargo run --example compare ./examples/data/records_example.csv csv ./examples/data/records_example.txt txt
```

### Запуск утилит
* Конвертер (ypbank_converter)

    Читает входной файл в указанном формате и выводит результат в стандартный вывод (можно перенаправить в файл)

    ```bash
    cargo run --bin ypbank_converter -- --input input.csv --in-format csv --out-format txt > output.txt
    ```

    Параметры:  
    * input <ФАЙЛ> - путь к входному файлу.  
    * in-format <ФОРМАТ> — формат входного файла (csv, txt, bin).

    * out-format <ФОРМАТ> — желаемый формат вывода.

* Компаратор (ypbank_comparer).   

    Сравнивает два файла транзакций, сообщает, идентичны ли они, и при различиях показывает первую несовпавшую транзакцию и разницу в длине.

    ```bash
    cargo run --bin ypbank_comparer -- --file1 a.bin --format1 bin --file2 b.csv --format2 csv
    ```

    Параметры:  
    * file1 <ФАЙЛ> — первый файл.
    * format1 <ФОРМАТ> — его формат.
    * file2 <ФАЙЛ> — второй файл.
    * format2 <ФОРМАТ> — его формат.


## Спецификация

Подробные спецификации каждого формата находятся в папке `examples/docs`:

- [CSV-формат](examples/docs/YPBankCsvFormat_ru.md)
- [Текстовый формат](examples/docs/YPBankTextFromat_ru.md)
- [Бинарный формат](examples/docs/YPBankBinFormat_ru.md)


## Документация

```bash
cargo doc --open -p ypbank_parser
```
