use clap::ValueEnum;
use crate::data::word_sets::*;

#[derive(Debug, Clone, ValueEnum, PartialEq, Copy)]
pub enum Lang { En, Ja, De, Fr, Es }

pub fn run(words: usize, count: usize, random: bool, lang: Option<Lang>) -> anyhow::Result<()> {
    let mut results: Vec<String> = Vec::with_capacity(count);
    let actual_lang = lang.unwrap_or(Lang::En);
    
    for _ in 0..count {
        results.push(generate_text(words, random, actual_lang));
    }
    
    println!("{}", results.join("\n\n"));
    Ok(())
}

fn generate_text(words: usize, random: bool, lang: Lang) -> String {
    let word_set: &[&str] = match lang {
        Lang::En => WORD_SET_EN,
        Lang::Ja => WORD_SET_JA,
        Lang::De => WORD_SET_DE,
        Lang::Fr => WORD_SET_FR,
        Lang::Es => WORD_SET_ES
    };
    
    let separator = if lang == Lang::Ja { "" } else { " " };
    
    if random {
        let mut word_set_vec = Vec::from(word_set);
        fastrand::shuffle(&mut word_set_vec);
        word_set_vec.iter().cycle().take(words).copied().collect::<Vec<_>>().join(separator)
    } else {
        word_set.iter().cycle().take(words).copied().collect::<Vec<_>>().join(separator)
    }
}
