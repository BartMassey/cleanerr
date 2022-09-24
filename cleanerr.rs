fn main() -> Result<(), String> {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() > 1 {
        return Err(format!("{}: arguments: {}", &argv[0], argv[1..].join(" ")));
    }
    println!("no arguments");
    Ok(())
}
