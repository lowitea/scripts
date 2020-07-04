use rstest::rstest;
use std::process::Command;

#[rstest(
    input,
    expected,
    case("abc", 3),
    case(
        "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abc\
         defghijklmnopqrstuvwxyz{|}~",
        94
    ),
    case("𐌰𐌱𐌲𐌳𐌴𐌵𐌶𐌷𐌸𐌹𐌺𐌻𐌼𐌽𐌾𐌿𐍀𐍁𐍂𐍃𐍄𐍅𐍆𐍇𐍈𐍉𐍊", 27),
    case(
        "ЀЁЂЃЄЅІЇЈЉЊЋЌЍЎЏАБВГДЕЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ\
         абвгдежзийклмнопрстуфхцчшщъыьэюяѐёђѓєѕіїјљњћќѝўџ\
         ѠѡѢѣѤѥѦѧѨѩѪѫѬѭѮѯѰѱѲѳѴѵѶѷѸѹѺѻѼѽѾѿ",
        128
    )
)]
fn functional_test(input: &str, expected: u32) {
    // запускаем код как внешнюю команду
    let output = Command::new("cargo")
        .arg("run")
        .arg(input)
        .output()
        .expect("failed to execute process");

    // преобразуем вывод в удобный для сравнения вид
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stdout: u32 = stdout.trim().parse().unwrap();

    assert_eq!(stdout, expected)
}
