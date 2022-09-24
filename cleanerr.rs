fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() > 1 {
        panic!("{}: arguments: {}", &argv[0], argv[1..].join(" "));
    }
    println!("no arguments");
}
