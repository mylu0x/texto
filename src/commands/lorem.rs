use anyhow::Ok;

use crate::data::word_sets::WORD_SET_LOREM;

pub fn run(words: usize) -> anyhow::Result<()> {
    println!("{}", generate_lorem(words));
    Ok(())
}

fn generate_lorem(words: usize) -> String {
    let word_set = WORD_SET_LOREM;
    word_set.iter().cycle().take(words).copied().collect::<Vec<_>>().join(" ")
}
