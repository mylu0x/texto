const WORD_SET_EN: &[&str] = &[
    "hello", "how", "are", "you", "today", "I", "am", "doing", "fine",
    "how", "about", "you", "this", "is", "good", "I", "think", "this", "looks", "good",
    "let", "us", "try", "something", "new", "what", "is", "this",
    "I", "want", "to", "drink", "water", "it", "is", "rainy", "here"
];

pub fn run(words: usize, count: usize, random: bool) -> anyhow::Result<()> {
    let mut results: Vec<String> = Vec::with_capacity(count);
    
    for _ in 0..count {
        results.push(generate_text(WORD_SET_EN, words, random));
    }
    
    println!("{}", results.join("\n\n"));
    Ok(())
}

fn generate_text(word_set: &[&str], words: usize, random: bool) -> String {
    if random {
        let mut word_set_vec = word_set.to_vec();
        fastrand::shuffle(&mut word_set_vec);
        word_set_vec.iter().cycle().take(words).copied().collect::<Vec<_>>().join(" ")
    } else {
        word_set.iter().cycle().take(words).copied().collect::<Vec<_>>().join(" ")
    }
}
