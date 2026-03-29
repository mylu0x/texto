use clap::ValueEnum;
use uuid::Uuid;

#[derive(Debug, Clone, ValueEnum, PartialEq, Copy)]
pub enum UuidVersion {
    V4, V7
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Copy)]
pub enum Case {
    Lower, Upper
}

pub fn run(version: UuidVersion, count: usize, case: Case) -> anyhow::Result<()> {
    let results: Vec<String> = (0..count)
        .map(|_| {
            let uuid = generate_uuid(version);
            match case {
                Case::Lower => format!("{:x}", uuid),
                Case::Upper => format!("{:X}", uuid)
            }
        })
        .collect();
    
    println!("{}", results.join("\n"));
    Ok(())
}

fn generate_uuid(version: UuidVersion) -> Uuid {
    let result = match version {
        UuidVersion::V4 => Uuid::new_v4(),
        UuidVersion::V7 => Uuid::now_v7()
    };
    result
}
