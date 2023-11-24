use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

use encoding_rs::WINDOWS_1251;
use regex::Regex;

const VOWELS_REGEX: &'static str = r#"[уеыаоэёяию]*"#;

pub(crate) fn generate_words<'a>(
    number: &str,
    from: &'a str,
) -> Result<Vec<&'a str>, std::io::Error> {
    let regex = generate_regex_from_number(number);
    // println!("Regex: {}", regex);
    let mut words = HashSet::new();
    for matched in regex.find_iter(&from) {
        let s = matched.as_str().trim_end();
        words.insert(&s[..s.len() - 2]);
    }
    let mut collect = words.into_iter().collect::<Vec<_>>();
    collect.sort();
    Ok(collect)
}

pub(crate) fn read_file_to_string_utf8(
    file_path: &str,
) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub(crate) fn read_file_to_string_windows_1251(
    file_path: &str,
) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let (string, _, _) = WINDOWS_1251.decode(&buffer);
    Ok(string.into_owned())
}

pub(crate) fn generate_regex_from_number(word: &str) -> Regex {
    // let mut regex = String::from(r#"^"#);
    // regex.push_str(VOWELS_REGEX);
    let mut regex = String::from(VOWELS_REGEX);
    for number in word.chars() {
        let pair = translate_number(number);
        regex.push_str(pair);
        regex.push_str(VOWELS_REGEX);
    }
    // regex.push_str(r#"$"#);
    regex.push_str(r#":S\n"#);
    Regex::new(&regex).unwrap()
}

pub(crate) fn translate_number(n: char) -> &'static str {
    match n {
        '0' => "[мн]",
        '1' => "[гж]",
        '2' => "[дт]",
        '3' => "[кх]",
        '4' => "[чщ]",
        '5' => "[пб]",
        '6' => "[шл]",
        '7' => "[сз]",
        '8' => "[вф]",
        '9' => "[рц]",
        _ => unreachable!(),
    }
}
