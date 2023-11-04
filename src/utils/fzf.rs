use skim::prelude::*;
use std::io::Cursor;

#[allow(clippy::unwrap_used)]
pub fn skim_menu(options: &[&str], prompt: Option<&str>) -> String {
    let choices: String = options.join("\n");

    let options = SkimOptionsBuilder::default()
        .prompt(prompt)
        .no_clear_if_empty(true)
        .reverse(true)
        .build()
        .unwrap();

    loop {
        let item_reader = SkimItemReader::default();
        let items = item_reader.of_bufread(Cursor::new(choices.clone().into_bytes()));

        let selected_items = Skim::run_with(&options, Some(items.clone()))
            .map(|out| out.selected_items)
            .unwrap_or_default();

        if let Some(selected_item) = selected_items.first() {
            return selected_item.output().to_string();
        }
    }
}

#[allow(clippy::unwrap_used)]
pub fn prompt_user(prompt_text: &str) -> String {
    let options = SkimOptionsBuilder::default()
        .prompt(Some(prompt_text))
        .reverse(true)
        .build()
        .unwrap();

    // Use an empty string as the input source to ensure no items are displayed.
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(""));

    Skim::run_with(&options, Some(items))
        .map(|out| out.query) // We extract the query string, which is user input.
        .map(|query| query.to_string())
        .unwrap()
}
