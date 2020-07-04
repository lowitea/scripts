use rstest::*;
use std::process::Command;

#[fixture]
fn test_setup() {
    // собираем дев бинарник для использования в тесте
    Command::new("cargo")
        .arg("build")
        .output()
        .expect("failed to execute process");
}

#[rstest(
    param,
    input,
    expected,
    // тесткейсы для секунд
    case("-s", "100", "1970-01-01 03:01:40 +03:00"),
    case("-s", "31536000", "1971-01-01 03:00:00 +03:00"),
    case("-s", "1593861988", "2020-07-04 14:26:28 +03:00"),
    // тесткейсы для миллисекунд
    case("-m", "100", "1970-01-01 03:00:00.100 +03:00"),
    case("-m", "31536000000", "1971-01-01 03:00:00 +03:00"),
    case("-m", "1593861988793", "2020-07-04 14:26:28.793 +03:00"),
    // тесткейсы для даты
    case(
        "-d",
        "2020-07-04 14:26:28.793676540 +03:00",
        "seconds:      1593861988\nmilliseconds: 1593861988793"
    ),
    case(
        "-d",
        "1970-01-01 03:00:00 +03:00",
        "seconds:      0\nmilliseconds: 0"
    ),
    case(
        "-d",
        "1970-01-01 00:00:00.100 +00:00",
        "seconds:      0\nmilliseconds: 100"
    ),
    case(
        "-d",
        "1970-01-01T01:00:00.100Z",
        "seconds:      3600\nmilliseconds: 3600100"
    )
)]
fn functional_test(test_setup: (), param: &str, input: &str, expected: &str) {
    // запускаем код как внешнюю команду
    let output = Command::new("target/debug/ts")
        .arg(param)
        .arg(input)
        .output()
        .expect("failed to execute process");

    // преобразуем вывод в удобный для сравнения вид
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stdout = stdout.trim();

    assert_eq!(stdout, expected)
}
