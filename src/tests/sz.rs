use std::fs::File;
use std::io::{BufRead, BufReader};
use tabled::{settings::Panel, settings::Style, Table};
use tokenizer_py::{tokenize, Token};
use walkdir::WalkDir;

#[test]
fn sz() -> std::io::Result<()> {
    let mut table: Vec<(String, i32, f32)> = vec![];

    for entry in WalkDir::new("src/teenygrad") {
        let filepath = entry.unwrap().path().to_string_lossy().to_string();
        if filepath.ends_with(".rs") {
            let tokens = BufReader::new(File::open(&filepath).unwrap())
                .lines()
                .filter_map(|line| line.ok())
                .into_iter()
                .map(|item| {
                    tokenize(item.to_string())
                        .unwrap()
                        .into_iter()
                        .filter(|token| {
                            matches!(
                                token,
                                Token::Name(_)
                                    | Token::OP(_)
                                    | Token::Number(_)
                                    | Token::String(_)
                                    | Token::Comment(_)
                            )
                        })
                        .take_while(|token| !matches!(token, Token::OP(s) if s == "//"))
                        .collect::<Vec<_>>()
                })
                .filter(|vec| !vec.is_empty())
                .collect::<Vec<Vec<_>>>();
            table.push((
                filepath.chars().skip(4).collect::<String>(),
                tokens.len() as i32,
                (tokens.iter().map(|vec| vec.len()).sum::<usize>() as f32) / (tokens.len() as f32),
            ));
        }
    }

    println!(
        "\n{}",
        Table::new(&table).with(Style::psql()).with(Panel::header(
            "NAME                                 | LINES        | TOKENS/LINE"
        ))
    ); // I really wish it was nicer to make this table

    let total_lines = table.iter().map(|row| row.1).sum::<i32>();
    println!("\nTotal line count: {:?}", total_lines);
    assert!(total_lines < 1500, "COME ON BRUV, ITS SUPPOSE TO BE TINY");
    return Ok(());
}
