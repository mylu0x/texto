const WORD_SET_EN: &[&str] = &[
    "hello", "how", "are", "you", "today", "I", "am", "doing", "fine",
    "how", "about", "you", "this", "is", "good", "I", "think", "this", "looks", "good",
    "let", "us", "try", "something", "new", "what", "is", "this",
    "I", "want", "to", "drink", "water", "it", "is", "rainy", "here"
];

pub fn run(words: usize) -> anyhow::Result<()> {
    println!("{}", generate_text(WORD_SET_EN, words));
    Ok(())
}

fn generate_text(word_set: &[&str], words: usize) -> String {
    word_set.iter().cycle().take(words).copied().collect::<Vec<_>>().join(" ")
}
