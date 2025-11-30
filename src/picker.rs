use std::io::Cursor;

use skim::{
    RankCriteria, Skim,
    prelude::{SkimItemReader, SkimOptionsBuilder},
};

use crate::source::Source;

pub fn fuzzy_pick_session(sources: Vec<Source>) -> Option<Source> {
    let item_str = sources
        .iter()
        .map(|s| s.display_name(true))
        .collect::<Vec<String>>()
        .join("\n");

    // Tiebreak by index to ensure consistent selection order
    let options = SkimOptionsBuilder::default()
        .height("100%".to_string())
        .multi(false)
        .tiebreak(vec![RankCriteria::Index])
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(item_str));

    let output = Skim::run_with(&options, Some(items))?;

    if output.is_abort {
        return None;
    }

    // Get the selected index and return the corresponding Source
    let selected_text = output.selected_items.first()?.output().to_string();
    sources
        .into_iter()
        .find(|s| s.display_name(true) == selected_text)
}
