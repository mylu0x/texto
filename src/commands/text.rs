use clap::ValueEnum;
use crate::data::word_sets::*;

#[derive(Debug, Clone, ValueEnum, PartialEq, Copy)]
pub enum Lang { En, Ja, De, Fr, Es, No, Ru, Pl, It, ZhCn, ZhTw, Ko, Hi }

#[derive(Debug, Clone, ValueEnum, Copy)]
pub enum Format { Plain, Html, Json }

pub fn run(words: usize, count: usize, random: bool, lang: Option<Lang>, format: Format, separator: &str) -> anyhow::Result<String> {
    let mut results: Vec<String> = Vec::with_capacity(count);
    let actual_lang = lang.unwrap_or(Lang::En);
    
    for _ in 0..count {
        results.push(generate_text(words, random, actual_lang, separator));
    }
    
    let result: String = match format {
        Format::Plain => results.join("\n\n"),
        Format::Html => results.iter().map(|item| { format!("<p>{}</p>", item) }).collect::<Vec<String>>().join("\n\n"),
        Format::Json => format!("[{}]", results.iter().map(|item| { format!("\"{}\"", item) }).collect::<Vec<String>>().join(", "))
    };
    
    Ok(result)
}

fn generate_text(words: usize, random: bool, lang: Lang, separator: &str) -> String {
    let word_set: &[&str] = match lang {
        Lang::En => WORD_SET_EN,
        Lang::Ja => WORD_SET_JA,
        Lang::De => WORD_SET_DE,
        Lang::Fr => WORD_SET_FR,
        Lang::Es => WORD_SET_ES,
        Lang::No => WORD_SET_NO,
        Lang::Ru => WORD_SET_RU,
        Lang::Pl => WORD_SET_PL,
        Lang::It => WORD_SET_IT,
        Lang::ZhCn => WORD_SET_ZH_CN,
        Lang::ZhTw => WORD_SET_ZH_TW,
        Lang::Ko => WORD_SET_KO,
        Lang::Hi => WORD_SET_HI
    };
    
    let separator = match lang {
        Lang::Ja | Lang::ZhCn | Lang::ZhTw => "",
        _ => separator
    };
    
    if random {
        let mut word_set_vec = Vec::from(word_set);
        fastrand::shuffle(&mut word_set_vec);
        word_set_vec.iter().cycle().take(words).copied().collect::<Vec<_>>().join(separator)
    } else {
        word_set.iter().cycle().take(words).copied().collect::<Vec<_>>().join(separator)
    }
}
