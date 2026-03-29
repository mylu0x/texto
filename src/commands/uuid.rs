use clap::ValueEnum;
use uuid::Uuid;

#[derive(Debug, Clone, ValueEnum, PartialEq, Copy)]
pub enum UuidVersion {
    V4, V7
}

pub fn run(version: UuidVersion) -> anyhow::Result<()> {
    println!("{}", generate_uuid(version));
    Ok(())
}

fn generate_uuid(version: UuidVersion) -> Uuid {
    let result = match version {
        UuidVersion::V4 => Uuid::new_v4(),
        UuidVersion::V7 => Uuid::now_v7()
    };
    result
}
