use std::env;

pub fn get_file() -> Result<_, &str> {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        return Err("No filepath provided.");
    }
    println!("{}", args[0]);
    Ok(())
}