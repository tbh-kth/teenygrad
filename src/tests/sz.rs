use std::fs::File;
use std::io::{BufRead, BufReader};
use tabled::{settings::Panel, settings::Style, Table};
use tokenizer_py::{tokenize, Token};
use walkdir::WalkDir;

#[test]
fn lines() -> std::io::Result<()> {
    let token_whitelist = |token: &Token| {
        matches!(
            token,
            Token::OP(_) | Token::Name(_) | Token::Number(_) | Token::String(_)
        )
    };
    let mut table: Vec<(String, i32, f32)> = vec![];

    for entry in WalkDir::new("src/teenygrad") {
        let filepath = entry.unwrap().path().to_string_lossy().to_string();
        if filepath.ends_with(".rs") {
            let lines = BufReader::new(File::open(&filepath).unwrap())
                .lines()
                .filter_map(|line| line.ok())
                .filter(|line| !line.is_empty() && !line.starts_with("//"))
                .collect::<Vec<_>>();
            let _tokens = lines
                .clone()
                .into_iter()
                .map(|item| tokenize(item.to_string()))
                .filter_map(|token| token.ok())
                .filter(|token| token.iter().any(&token_whitelist))
                .collect::<Vec<_>>();
            table.push((
                filepath.chars().skip(4).collect::<String>(),
                lines.len() as i32,
                0.0,
                //(tokens.len() as f32) / (lines.len() as f32), // We will add this later
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
