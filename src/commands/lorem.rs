use anyhow::Ok;

use crate::data::word_sets::WORD_SET_LOREM;

pub fn run() -> anyhow::Result<()> {
    println!("{}", generate_lorem());
    Ok(())
}

fn generate_lorem() -> String {
    let word_set = WORD_SET_LOREM;
    word_set.iter().copied().collect::<Vec<_>>().join(" ")
}
