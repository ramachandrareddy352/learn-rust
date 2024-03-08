use core::num;
use std::fs;
use std::io;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> CliError {
        CliError::IoError(e)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(e: num::ParseIntError) -> CliError {
        CliError::ParseError(e)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? automatically converts io::Error to CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    // example
    let my_str = "hello";
    // three conversions below all depends on the fact: String implements From<&str>:
    let string1 = String::from(my_str);
    let string2 = my_str.to_string();
    // Explicit type annotation is required here
    let string3: String = my_str.into();

    // 1
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    let i3: u32 = 'a'.into();
    let s: String = 'a'.into();

    // 2
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30_i32.into();
    assert_eq!(num.value, 30);

    // 4
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(n: i32) -> Number {
        Number { value: n }
    }
}
// or:
// impl From<i32> for Number {
//     fn from(n: i32) -> Self {
//         Self { value: n }
//     }
// }
