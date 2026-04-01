use clap::ValueEnum;
use uuid::Uuid;

#[derive(Debug, Clone, ValueEnum, PartialEq, Copy)]
pub enum UuidVersion {
    V4, V7
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Copy)]
pub enum UuidCase {
    Lower, Upper
}

#[derive(Debug, Clone, ValueEnum, Copy)]
pub enum UuidFormat {
    Hyphenated, Simple
}

pub fn run(version: UuidVersion, count: usize, case: UuidCase, format: UuidFormat) -> anyhow::Result<String> {
    let results: Vec<String> = (0..count)
        .map(|_| {
            let uuid = generate_uuid(version);
            
            let formatted_str_uuid = match format {
                UuidFormat::Hyphenated => uuid.as_hyphenated().to_string(),
                UuidFormat::Simple => uuid.as_simple().to_string()
            };
            
            match case {
                UuidCase::Lower => formatted_str_uuid.to_lowercase(),
                UuidCase::Upper => formatted_str_uuid.to_uppercase()
            }
        })
        .collect();
    
    Ok(results.join("\n"))
}

fn generate_uuid(version: UuidVersion) -> Uuid {
    match version {
        UuidVersion::V4 => Uuid::new_v4(),
        UuidVersion::V7 => Uuid::now_v7()
    }
}
