use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub async fn process_dictionary(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let words = reader
        .lines()
        .filter(|line| !line.as_deref().map_or(false, |s| s.starts_with(";")))
        .map(|line| {
            let mut buffer = String::new();
            for c in line.unwrap().chars() {
                if c != ';' {
                    buffer.push(c);
                } else {
                    break;
                }
            }
            unidecode::unidecode(&buffer)
        })
        .skip(2)
        .collect::<Vec<_>>();

    words
}
