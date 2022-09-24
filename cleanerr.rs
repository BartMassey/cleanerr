fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() > 1 {
        eprintln!("{}: arguments: {}", &argv[0], argv[1..].join(" "));
        std::panic::resume_unwind(Box::new(()));
    }
    println!("no arguments");
}
