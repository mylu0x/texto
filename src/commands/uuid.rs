use uuid::Uuid;

pub fn run() -> anyhow::Result<()> {
    println!("{}", generate_uuid());
    Ok(())
}

fn generate_uuid() -> String {
    let result = Uuid::new_v4().to_string();
    result
}
