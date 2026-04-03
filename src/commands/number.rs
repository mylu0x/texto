pub fn run(min: isize, max: isize) -> anyhow::Result<String> {
    let (clear_min, clear_max) = if min <= max { (min, max) } else { (max, min) };
    let result = fastrand::isize(clear_min..=clear_max);
    
    Ok(result.to_string())
}
