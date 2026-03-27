use clap::ValueEnum;

const WORD_SET_EN: &[&str] = &[
    "hello", "how", "are", "you", "today", "I", "am", "doing", "fine",
    "how", "about", "you", "this", "is", "good", "I", "think", "this", "looks", "good",
    "let", "us", "try", "something", "new", "what", "is", "this",
    "I", "want", "to", "drink", "water", "it", "is", "rainy", "here"
];

const WORD_SET_JA: &[&str] = &[
    "こんにちは", "元気", "ですか", "今日", "私", "は", "元気", "です",
    "あなた", "は", "どう", "ですか", "これ", "は", "いい", "と思う",
    "これ", "は", "良さそう", "私たち", "は", "とても", "暇です",
    "何", "ですか", "これ", "私", "は", "水", "を", "飲みたい", "ここ", "は", "雨", "です"
];

const WORD_SET_DE: &[&str] = &[
    "hallo", "wie", "geht", "es", "dir", "heute", "ich", "bin", "gut",
    "und", "du", "dies", "ist", "gut", "ich", "denke", "dies", "sieht", "gut", "aus",
    "lass", "uns", "etwas", "neues", "versuchen", "was", "ist", "dies",
    "ich", "möchte", "trinken", "wasser", "es", "regnet", "hier"
];

const WORD_SET_FR: &[&str] = &[
    "bonjour", "comment", "ça", "va", "aujourd'hui", "je", "vais", "bien",
    "et", "toi", "ceci", "est", "bon", "je", "pense", "cela", "semble", "bien",
    "laissons", "nous", "essayer", "quelque", "chose", "de", "nouveau", "que", "est", "ceci",
    "je", "veux", "boire", "de", "l'eau", "il", "pleut", "ici"
];

const WORD_SET_ES: &[&str] = &[
    "hola", "cómo", "estás", "hoy", "yo", "estoy", "bien",
    "y", "tú", "esto", "es", "bueno", "creo", "que", "esto", "parece", "bien",
    "vamos", "a", "probar", "algo", "nuevo", "qué", "es", "esto",
    "quiero", "beber", "agua", "aquí", "está", "lloviendo"
];

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
