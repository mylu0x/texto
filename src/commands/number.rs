pub fn run(min: isize, max: isize, count: usize) -> anyhow::Result<String> {
    let (clear_min, clear_max) = if min <= max { (min, max) } else { (max, min) };
    let mut result: Vec<isize> = Vec::with_capacity(count);
    
    for _ in 0..count {
        result.push(fastrand::isize(clear_min..=clear_max));
    }
    
    let joined_result: String = result.iter().map(|num| format!("{} ", num)).collect();
    
    Ok(joined_result)
}
