use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum, Copy)]
pub enum DecimalFormat {
    Plain, Json
}

pub fn run(min: isize, max: isize, count: usize, format: DecimalFormat) -> anyhow::Result<String> {
    let (clear_min, clear_max) = if min <= max { (min, max) } else { (max, min) };
    let mut result: Vec<isize> = Vec::with_capacity(count);
    
    for _ in 0..count {
        result.push(fastrand::isize(clear_min..=clear_max));
    }
    
    let result_vec = result.iter().map(|num| format!("{}", num)).collect::<Vec<_>>();
    
    let joined_result = match format {
        DecimalFormat::Plain => result_vec.join(" "),
        DecimalFormat::Json => format!("[{}]", result_vec.join(","))
    };
    
    Ok(joined_result)
}
