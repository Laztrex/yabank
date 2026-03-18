use std::process::{Command, Stdio};
use std::io::Write;

#[test]
fn test_converter_stdin() {
    // Тестовые данные в формате CSV
    let input_data = "\
TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"
1000000000000001,TRANSFER,9223372036854775807,9223372036854775807,200,1633036920000,PENDING,\"Record number 2\"\n";

    // Запускаем конвертер с аргументами (читаем из stdin, выводим в txt)
    let mut child = Command::new(env!("CARGO_BIN_EXE_ypbank_converter"))
        .arg("--input")
        .arg("-")
        .arg("--in-format")
        .arg("csv")
        .arg("--out-format")
        .arg("txt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn converter");

    // Пишем данные в stdin
    let mut stdin = child.stdin.take().expect("Failed to get stdin");
    stdin.write_all(input_data.as_bytes()).expect("Failed to write to stdin");
    drop(stdin); // закрываем stdin

    // Ждём завершения
    let output = child.wait_with_output().expect("Failed to wait for child");

    // Проверяем успешность
    assert!(output.status.success(), "Converter failed: {}", String::from_utf8_lossy(&output.stderr));

    // Проверяем, что вывод содержит ожидаемые строки в формате TXT
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("TX_ID: 1000000000000000"));
    assert!(stdout.contains("TX_TYPE: DEPOSIT"));
    assert!(stdout.contains("DESCRIPTION: \"Record number 1\""));
    assert!(stdout.contains("TX_ID: 1000000000000001"));
    assert!(stdout.contains("TX_TYPE: TRANSFER"));
}